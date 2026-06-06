//! The variable a `VariableDeclarator`-host's call result is bound to.

use unsnarl_ir::serialized::{SerializedCallbackHost, SerializedDefinition, SerializedScope};

use super::context::BuilderContext;

/// The variable a `VariableDeclarator`-host's call result is bound to:
/// the variable in `scope` whose declarator init starts where the
/// host's bound expression starts. They are the same AST node, so the
/// start offsets match exactly.
pub fn result_var_for_host(
    host: &SerializedCallbackHost,
    scope: &SerializedScope,
    ctx: &BuilderContext<'_>,
) -> Option<String> {
    let init_start = host.start_span.offset.0;
    scope.variables.iter().find_map(|vid| {
        let v = ctx.variable_map.get(vid.value())?;
        match v.defs.first()? {
            SerializedDefinition::Variable(d) => {
                (d.init()?.span.offset.0 == init_start).then(|| v.id.value().to_string())
            }
            _ => None,
        }
    })
}
