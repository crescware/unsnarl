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
//! synthetic call-proxy element). `call_start_offset` /
//! `call_end_offset` carry the enclosing `CallExpression` /
//! `NewExpression`'s own span; the **pair** is required to identify
//! "which call in the chain" because a chained shape such as
//! `a.b().c(cb)` has every nested `CallExpression` sharing the chain
//! root's `span.start`, so `call_start_offset` alone does not
//! disambiguate. Consumers look up the matching `Call` / `New` node
//! inside the corresponding `ExpressionStatementContainer.head` (whose
//! own `Call` / `New` variants carry the same span pair) and render
//! its `callee` subtree directly -- no callee text is duplicated into
//! this annotation. `arg_index` is the zero-based slot in the call's
//! `arguments` list.

use serde::Serialize;

use crate::primitive::Utf16CodeUnitOffset;

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CallbackArgument {
    pub statement_offset: Utf16CodeUnitOffset,
    pub call_start_offset: Utf16CodeUnitOffset,
    pub call_end_offset: Utf16CodeUnitOffset,
    pub arg_index: u32,
}

impl CallbackArgument {
    pub fn new(
        statement_offset: Utf16CodeUnitOffset,
        call_start_offset: Utf16CodeUnitOffset,
        call_end_offset: Utf16CodeUnitOffset,
        arg_index: u32,
    ) -> Self {
        Self {
            statement_offset,
            call_start_offset,
            call_end_offset,
            arg_index,
        }
    }

    pub fn statement_offset(&self) -> Utf16CodeUnitOffset {
        self.statement_offset
    }

    pub fn call_start_offset(&self) -> Utf16CodeUnitOffset {
        self.call_start_offset
    }

    pub fn call_end_offset(&self) -> Utf16CodeUnitOffset {
        self.call_end_offset
    }

    pub fn arg_index(&self) -> u32 {
        self.arg_index
    }
}
