//! Which React hook a wrapped binding is wrapped in.
//!
//! `useCallback` peels the wrapper so the binding's `init` points
//! at the inner function; `useMemo` keeps the init as the
//! `CallExpression` so the IR reads as an IIFE-style invocation.

pub const REACT_MODULE: &str = "react";
pub const HOOK_USE_CALLBACK: &str = "useCallback";
pub const HOOK_USE_MEMO: &str = "useMemo";

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum HookKind {
    UseCallback,
    UseMemo,
}

pub fn as_hook_kind(name: &str) -> Option<HookKind> {
    match name {
        HOOK_USE_CALLBACK => Some(HookKind::UseCallback),
        HOOK_USE_MEMO => Some(HookKind::UseMemo),
        _ => None,
    }
}
