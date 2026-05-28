//! Walks a single scope's containing node and collects its direct
//! `BreakStatement` / `ContinueStatement` children, packaged as
//! [`AbruptStatement`] rows the visualization layer can consume.
//!
//! Recursion stops at every nested scope boundary (`BlockStatement`,
//! `Function*`, `Class*`, `For*` / `While` / `DoWhile`,
//! `SwitchStatement`'s cases, etc.) — those scopes call back into this
//! function for their own anchor. Control statements that do *not*
//! introduce a scope (`IfStatement`, `LabeledStatement`) are walked
//! through so a break that's nested under `if (...)` directly is
//! still attributed to the enclosing scope.

use oxc_ast::ast::{BreakStatement, ContinueStatement, Statement};
use oxc_ast::AstKind;

use unsnarl_ir::primitive::{SourceIndex, Utf8ByteOffset};
use unsnarl_ir::scope::{AbruptStatement, AbruptStatementType};

pub fn collect_abrupt_statements<'a>(
    kind: &AstKind<'a>,
    index: &SourceIndex<'_>,
) -> Vec<AbruptStatement> {
    let mut out = Vec::new();
    match kind {
        AstKind::BlockStatement(b) => {
            for s in &b.body {
                walk_statement(s, &mut out, index);
            }
        }
        AstKind::Function(f) => {
            if let Some(body) = f.body.as_ref() {
                for s in &body.statements {
                    walk_statement(s, &mut out, index);
                }
            }
        }
        AstKind::ArrowFunctionExpression(arr) => {
            for s in &arr.body.statements {
                walk_statement(s, &mut out, index);
            }
        }
        AstKind::ForStatement(fs) => {
            walk_statement(&fs.body, &mut out, index);
        }
        AstKind::WhileStatement(ws) => {
            walk_statement(&ws.body, &mut out, index);
        }
        AstKind::DoWhileStatement(d) => {
            walk_statement(&d.body, &mut out, index);
        }
        AstKind::ForInStatement(fis) => {
            walk_statement(&fis.body, &mut out, index);
        }
        AstKind::ForOfStatement(fos) => {
            walk_statement(&fos.body, &mut out, index);
        }
        AstKind::SwitchCase(sc) => {
            for s in &sc.consequent {
                walk_statement(s, &mut out, index);
            }
        }
        AstKind::SwitchStatement(_) | AstKind::CatchClause(_) | AstKind::Class(_) => {
            // SwitchStatement: each case is its own scope, they
            //   collect their own.
            // CatchClause: the catch body is a BlockStatement which
            //   is its own scope.
            // Class: only carries methods (function scopes).
        }
        _ => {}
    }
    out
}

fn walk_statement<'a>(
    stmt: &Statement<'a>,
    out: &mut Vec<AbruptStatement>,
    index: &SourceIndex<'_>,
) {
    match stmt {
        Statement::BreakStatement(b) => push_break(out, b, index),
        Statement::ContinueStatement(c) => push_continue(out, c, index),
        // Scope-introducing statements: their own scope's
        // collect_abrupt_statements call will pick the inner
        // break / continue up.
        Statement::BlockStatement(_)
        | Statement::FunctionDeclaration(_)
        | Statement::ClassDeclaration(_)
        | Statement::ForStatement(_)
        | Statement::WhileStatement(_)
        | Statement::DoWhileStatement(_)
        | Statement::ForInStatement(_)
        | Statement::ForOfStatement(_)
        | Statement::SwitchStatement(_)
        | Statement::TryStatement(_) => {}
        // Statements that do not introduce a new scope but can
        // syntactically host a break / continue directly.
        Statement::IfStatement(if_stmt) => {
            walk_statement(&if_stmt.consequent, out, index);
            if let Some(alt) = if_stmt.alternate.as_ref() {
                walk_statement(alt, out, index);
            }
        }
        Statement::LabeledStatement(ls) => {
            walk_statement(&ls.body, out, index);
        }
        _ => {}
    }
}

fn push_break(out: &mut Vec<AbruptStatement>, b: &BreakStatement<'_>, index: &SourceIndex<'_>) {
    let span = index.span_at(Utf8ByteOffset(b.span.start));
    let end_span = index.span_at(Utf8ByteOffset(b.span.end));
    let target = b.label.as_ref().map(|l| l.name.as_str().to_string());
    out.push(AbruptStatement {
        r#type: AbruptStatementType::Break,
        target,
        span,
        end_span,
    });
}

fn push_continue(
    out: &mut Vec<AbruptStatement>,
    c: &ContinueStatement<'_>,
    index: &SourceIndex<'_>,
) {
    let span = index.span_at(Utf8ByteOffset(c.span.start));
    let end_span = index.span_at(Utf8ByteOffset(c.span.end));
    let target = c.label.as_ref().map(|l| l.name.as_str().to_string());
    out.push(AbruptStatement {
        r#type: AbruptStatementType::Continue,
        target,
        span,
        end_span,
    });
}
