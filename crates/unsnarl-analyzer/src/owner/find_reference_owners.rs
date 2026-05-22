//! Walk a reference's ancestor chain to locate the owning binding
//! slot.
//!
//! Returns an [`OwnerLookup`] describing the path entry the call
//! site should resolve back to an AST handle. The pipeline then
//! calls [`all_binding_variables`] or [`assignment_target_variables`]
//! with that handle.
//!
//! [`all_binding_variables`]: super::all_binding_variables::all_binding_variables
//! [`assignment_target_variables`]: super::all_binding_variables::assignment_target_variables

use unsnarl_oxc_parity::AstType;

use crate::path_entry::PathEntry;

/// Owner-slot classification of an ancestor chain.
pub enum OwnerLookup {
    /// No owning slot found before the path ran out.
    None,
    /// A function / class boundary was hit first; no owners cross
    /// that boundary, so the analyzer reports an empty owner list.
    Boundary,
    /// The nearest owner is a `VariableDeclarator` at this path index;
    /// the call site should fetch the AST node's `id` slot
    /// (`BindingPattern`) and call [`all_binding_variables`].
    ///
    /// [`all_binding_variables`]: super::all_binding_variables::all_binding_variables
    VariableDeclarator { path_index: usize },
    /// The nearest owner is an `AssignmentExpression` at this path
    /// index; the call site should fetch the AST node's `left` slot
    /// (`AssignmentTarget`) and call [`assignment_target_variables`].
    ///
    /// [`assignment_target_variables`]: super::all_binding_variables::assignment_target_variables
    AssignmentExpression { path_index: usize },
}

pub fn locate_reference_owner_slot(path: &[PathEntry]) -> OwnerLookup {
    for (i, entry) in path.iter().enumerate().rev() {
        match entry.node.r#type {
            AstType::VariableDeclarator => {
                return OwnerLookup::VariableDeclarator { path_index: i };
            }
            AstType::AssignmentExpression => {
                return OwnerLookup::AssignmentExpression { path_index: i };
            }
            AstType::FunctionDeclaration
            | AstType::FunctionExpression
            | AstType::ArrowFunctionExpression
            | AstType::ClassDeclaration
            | AstType::ClassExpression
            | AstType::MethodDefinition
            | AstType::PropertyDefinition
            | AstType::AccessorProperty => {
                return OwnerLookup::Boundary;
            }
            _ => {}
        }
    }
    OwnerLookup::None
}

#[cfg(test)]
#[path = "find_reference_owners_test.rs"]
mod find_reference_owners_test;
