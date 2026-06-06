//! The seven nesting counters and their `NestingDepths` snapshot.

use unsnarl_ir::nesting_kind::{NestingDepth, NestingDepths, NestingKind};

pub(super) struct Counters {
    function: u32,
    r#if: u32,
    r#for: u32,
    r#while: u32,
    switch: u32,
    try_catch_finally: u32,
    block: u32,
}

impl Counters {
    pub(super) fn zero() -> Self {
        Self {
            function: 0,
            r#if: 0,
            r#for: 0,
            r#while: 0,
            switch: 0,
            try_catch_finally: 0,
            block: 0,
        }
    }

    pub(super) fn inc(&mut self, kind: NestingKind) {
        match kind {
            NestingKind::Function => self.function += 1,
            NestingKind::If => self.r#if += 1,
            NestingKind::For => self.r#for += 1,
            NestingKind::While => self.r#while += 1,
            NestingKind::Switch => self.switch += 1,
            NestingKind::TryCatchFinally => self.try_catch_finally += 1,
            NestingKind::Block => self.block += 1,
        }
    }

    pub(super) fn dec(&mut self, kind: NestingKind) {
        match kind {
            NestingKind::Function => self.function -= 1,
            NestingKind::If => self.r#if -= 1,
            NestingKind::For => self.r#for -= 1,
            NestingKind::While => self.r#while -= 1,
            NestingKind::Switch => self.switch -= 1,
            NestingKind::TryCatchFinally => self.try_catch_finally -= 1,
            NestingKind::Block => self.block -= 1,
        }
    }

    pub(super) fn snapshot(&self) -> NestingDepths {
        NestingDepths {
            function: NestingDepth(self.function),
            r#if: NestingDepth(self.r#if),
            r#for: NestingDepth(self.r#for),
            r#while: NestingDepth(self.r#while),
            switch: NestingDepth(self.switch),
            try_catch_finally: NestingDepth(self.try_catch_finally),
            block: NestingDepth(self.block),
        }
    }
}
