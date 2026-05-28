//! Scope-side annotation marking a function scope as the `arg_index`-th
//! argument of an enclosing `CallExpression` (or `NewExpression`) that
//! itself stands at `ExpressionStatement` level.
//!
//! Populated by the analyzer when a function scope is detected to be
//! a direct argument of a call whose enclosing statement is an
//! `ExpressionStatement`; consumed by the visual-graph builder to
//! group such function scopes under a single call-proxy subgraph
//! and by the mermaid emitter to label them with their argument
//! position.
//!
//! `statement_offset` matches the `start_span.offset` of the
//! enclosing `ExpressionStatement` (the same offset used to key the
//! synthetic call-proxy element). `call_offset` is the
//! `CallExpression`'s own start offset (`statement_offset` and
//! `call_offset` coincide when the statement's direct expression is
//! the call itself; they differ when the call sits inside an
//! `AwaitExpression`, a member chain, etc.). `arg_index` is the
//! zero-based slot in the call's `arguments` list.

use serde::Serialize;
use unsnarl_oxc_parity::AstType;

use crate::primitive::Utf16CodeUnitOffset;

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CallbackArgument {
    pub statement_offset: Utf16CodeUnitOffset,
    pub call_offset: Utf16CodeUnitOffset,
    pub call_parent_type: AstType,
    pub arg_index: u32,
}

impl CallbackArgument {
    pub fn new(
        statement_offset: Utf16CodeUnitOffset,
        call_offset: Utf16CodeUnitOffset,
        call_parent_type: AstType,
        arg_index: u32,
    ) -> Self {
        Self {
            statement_offset,
            call_offset,
            call_parent_type,
            arg_index,
        }
    }

    pub fn statement_offset(&self) -> Utf16CodeUnitOffset {
        self.statement_offset
    }

    pub fn call_offset(&self) -> Utf16CodeUnitOffset {
        self.call_offset
    }

    pub fn call_parent_type(&self) -> &AstType {
        &self.call_parent_type
    }

    pub fn arg_index(&self) -> u32 {
        self.arg_index
    }
}
