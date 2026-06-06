//! Synthesise the inner `ClassName` binding + definition for a class
//! declaration.

use oxc_index::IndexVec;

use unsnarl_ir::ids::{DefinitionId, ScopeId, VariableId};
use unsnarl_ir::primitive::{AstIdentifier, AstNode};
use unsnarl_ir::scope::{DefinitionData, ScopeData, VariableData};
use unsnarl_ir::DefinitionType;
use unsnarl_oxc_parity::AstType;

/// Synthesise the inner `ClassName` binding plus its `ClassName`
/// definition for a class declaration. Returns the new `VariableId`
/// so the caller can record it for the reference-mapping rebind pass.
pub(super) fn push_inner_class_name(
    scopes: &mut IndexVec<ScopeId, ScopeData>,
    variables: &mut IndexVec<VariableId, VariableData>,
    definitions: &mut IndexVec<DefinitionId, DefinitionData>,
    scope: ScopeId,
    name: &str,
    id_span: oxc_span::Span,
    class_span: oxc_span::Span,
) -> VariableId {
    let identifier = AstIdentifier::new(AstType::Identifier, name.to_string(), id_span);
    let var_id = variables.push(VariableData::new(
        name.to_string(),
        scope,
        vec![identifier.clone()],
        Vec::new(),
        Vec::new(),
    ));
    scopes[scope].insert_into_set(name.to_string(), var_id);
    scopes[scope].variables.push(var_id);
    let def_id = definitions.push(DefinitionData {
        r#type: DefinitionType::ClassName,
        name: identifier,
        node: AstNode::new(AstType::ClassDeclaration, class_span),
        parent: None,
        init: None,
        declaration_kind: None,
        import_source: None,
        imported_name: None,
    });
    variables[var_id].defs.push(def_id);
    var_id
}
