//! Constructor and block-classification helpers for
//! `NestingDepthVisitor` (the non-`Visit`-trait inherent methods).

use std::collections::HashMap;

use oxc_ast::AstKind;

use unsnarl_ir::nesting_kind::NestingKind;

use super::counters::Counters;
use super::{NestingDepthVisitor, ParentKind};

impl NestingDepthVisitor {
    pub(super) fn new() -> Self {
        Self {
            counters: Counters::zero(),
            depths_by_offset: HashMap::new(),
            enter_stack: Vec::new(),
            parent_types: Vec::new(),
            key_stack: Vec::new(),
        }
    }

    pub(super) fn classify_block(&self) -> Option<NestingKind> {
        let parent = self.parent_types.last().copied();
        let key = self.key_stack.last().copied().flatten();
        let Some(parent) = parent else {
            return Some(NestingKind::Block);
        };
        match (parent, key) {
            (ParentKind::Function | ParentKind::Arrow, Some("body")) => None,
            (ParentKind::If, Some("consequent") | Some("alternate")) => Some(NestingKind::If),
            (ParentKind::For | ParentKind::ForIn | ParentKind::ForOf, Some("body")) => {
                Some(NestingKind::For)
            }
            (ParentKind::While | ParentKind::DoWhile, Some("body")) => Some(NestingKind::While),
            (ParentKind::Try, Some("block") | Some("finalizer")) => {
                Some(NestingKind::TryCatchFinally)
            }
            (ParentKind::Catch, Some("body")) => Some(NestingKind::TryCatchFinally),
            _ => Some(NestingKind::Block),
        }
    }

    pub(super) fn parent_kind_of(kind: &AstKind<'_>) -> ParentKind {
        match kind {
            AstKind::Function(_) => ParentKind::Function,
            AstKind::ArrowFunctionExpression(_) => ParentKind::Arrow,
            AstKind::IfStatement(_) => ParentKind::If,
            AstKind::ForStatement(_) => ParentKind::For,
            AstKind::ForInStatement(_) => ParentKind::ForIn,
            AstKind::ForOfStatement(_) => ParentKind::ForOf,
            AstKind::WhileStatement(_) => ParentKind::While,
            AstKind::DoWhileStatement(_) => ParentKind::DoWhile,
            AstKind::TryStatement(_) => ParentKind::Try,
            AstKind::CatchClause(_) => ParentKind::Catch,
            _ => ParentKind::Other,
        }
    }
}
