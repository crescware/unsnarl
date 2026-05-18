//! Nesting depth tracking. Ports `ts/src/serializer/nesting-kind.ts` and
//! the `NestingDepths` shape from
//! `ts/src/ir/annotations/scope-annotation.ts`. `NestingDepths` lives
//! here (rather than in `unsnarl-annotations`) because `SerializedScope`
//! embeds it and `unsnarl-ir` cannot depend on `unsnarl-annotations`.

use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum NestingKind {
    Function,
    If,
    For,
    While,
    Switch,
    TryCatchFinally,
    Block,
}

#[derive(Serialize)]
pub struct NestingDepths {
    pub function: u32,
    pub r#if: u32,
    pub r#for: u32,
    pub r#while: u32,
    pub switch: u32,
    #[serde(rename = "try-catch-finally")]
    pub try_catch_finally: u32,
    pub block: u32,
}

impl NestingDepths {
    pub fn uniform(value: u32) -> Self {
        Self {
            function: value,
            r#if: value,
            r#for: value,
            r#while: value,
            switch: value,
            try_catch_finally: value,
            block: value,
        }
    }
}

#[cfg(test)]
#[path = "nesting_kind_test.rs"]
mod nesting_kind_test;
