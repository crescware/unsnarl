//! The variable a `VariableDeclarator`-host's call result is bound to.

use unsnarl_ir::serialized::{SerializedCallbackHost, SerializedDefinition, SerializedScope};

use super::context::BuilderContext;

/// The variable a `VariableDeclarator`-host's call result is bound to:
/// the variable whose declarator init starts where the host's bound
/// expression starts. They are the same AST node, so the start offsets
/// match exactly.
///
/// The search begins in `scope` and walks up its ancestor scopes. The
/// usual case finds the variable in `scope` on the first step, but a
/// callback hosted by a ternary arm is rendered inside a synthesised arm
/// `Block` scope (see `synthesise_conditional_arms`) whose own
/// `variables` are empty — the bound variable lives in an enclosing
/// scope. The match is by exact init offset, which only the one real
/// declarator can satisfy, so walking outward never produces a false
/// positive.
pub fn result_var_for_host(
    host: &SerializedCallbackHost,
    scope: &SerializedScope,
    ctx: &BuilderContext<'_>,
) -> Option<String> {
    let init_start = host.start_span.offset.0;
    let mut current = Some(scope);
    while let Some(s) = current {
        let found = s.variables.iter().find_map(|vid| {
            let v = ctx.variable_map.get(vid.value())?;
            match v.defs.first()? {
                SerializedDefinition::Variable(d) => {
                    (d.init()?.span.offset.0 == init_start).then(|| v.id.value().to_string())
                }
                _ => None,
            }
        });
        if found.is_some() {
            return found;
        }
        current = s
            .upper
            .as_ref()
            .and_then(|u| ctx.scope_map.get(u.value()).copied());
    }
    None
}
