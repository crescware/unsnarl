//! Concrete implementation of the [`Annotations`] lookup trait.
//!
//! Keys the three side-tables by arena ID.
//!
//! "Missing entries return zero-value defaults" — `Annotations`' doc
//! states this as a contract for each implementor. This impl
//! materialises the defaults once at construction time and hands
//! out references to them on miss; the alternative ("build a fresh
//! default per call") would force the trait to return owned values,
//! costing an allocation on every lookup.

use std::collections::HashMap;

use unsnarl_annotations::{
    Annotations, ReferenceAnnotation, ReferenceAnnotationFlags, ScopeAnnotation, VariableAnnotation,
};
use unsnarl_ir::nesting_kind::{NestingDepth, NestingDepths};
use unsnarl_ir::reference::ReferenceCompletion;
use unsnarl_ir::{ReferenceId, ScopeId, VariableId};

pub struct AnnotationsImpl {
    references: HashMap<ReferenceId, ReferenceAnnotation>,
    scopes: HashMap<ScopeId, ScopeAnnotation>,
    variables: HashMap<VariableId, VariableAnnotation>,
    empty_reference: ReferenceAnnotation,
    empty_scope: ScopeAnnotation,
    empty_variable: VariableAnnotation,
}

impl AnnotationsImpl {
    pub fn new() -> Self {
        Self {
            references: HashMap::new(),
            scopes: HashMap::new(),
            variables: HashMap::new(),
            empty_reference: empty_reference_annotation(),
            empty_scope: empty_scope_annotation(),
            empty_variable: empty_variable_annotation(),
        }
    }

    pub fn set_reference(&mut self, id: ReferenceId, annotation: ReferenceAnnotation) {
        self.references.insert(id, annotation);
    }

    pub fn set_scope(&mut self, id: ScopeId, annotation: ScopeAnnotation) {
        self.scopes.insert(id, annotation);
    }

    pub fn set_variable(&mut self, id: VariableId, annotation: VariableAnnotation) {
        self.variables.insert(id, annotation);
    }
}

impl Default for AnnotationsImpl {
    fn default() -> Self {
        Self::new()
    }
}

impl Annotations for AnnotationsImpl {
    fn of_reference(&self, id: ReferenceId) -> &ReferenceAnnotation {
        self.references.get(&id).unwrap_or(&self.empty_reference)
    }

    fn of_scope(&self, id: ScopeId) -> &ScopeAnnotation {
        self.scopes.get(&id).unwrap_or(&self.empty_scope)
    }

    fn of_variable(&self, id: VariableId) -> &VariableAnnotation {
        self.variables.get(&id).unwrap_or(&self.empty_variable)
    }
}

fn empty_reference_annotation() -> ReferenceAnnotation {
    ReferenceAnnotation {
        owners: Vec::new(),
        flags: ReferenceAnnotationFlags {
            call: false,
            receiver: false,
        },
        predicate_container: None,
        completion: ReferenceCompletion::Normal,
        jsx_element: None,
        expression_statement_container: None,
    }
}

fn empty_scope_annotation() -> ScopeAnnotation {
    ScopeAnnotation {
        block_context: None,
        callback_argument: None,
        falls_through: false,
        exits_function: false,
        nesting_depths: NestingDepths::uniform(NestingDepth(0)),
        abrupt_statements: Vec::new(),
    }
}

fn empty_variable_annotation() -> VariableAnnotation {
    VariableAnnotation { is_unused: false }
}

#[cfg(test)]
#[path = "annotations_impl_test.rs"]
mod annotations_impl_test;
