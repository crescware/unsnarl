//! Builds the human-readable label for a subgraph (e.g.
//! `foo()<br/>L3-12`).

use std::collections::HashMap;

use unsnarl_visual_graph::subgraph_kind::SubgraphKind;
use unsnarl_visual_graph::visual_node::VisualNode;
use unsnarl_visual_graph::visual_subgraph::VisualSubgraph;

use crate::escape::escape_into;
use crate::line_range_label::line_range_label_into;

pub fn subgraph_label(
    sg: &VisualSubgraph,
    node_map: &HashMap<String, &VisualNode>,
    debug: bool,
) -> String {
    let mut out = String::new();
    subgraph_label_into(&mut out, sg, node_map, debug);
    out
}

/// Destination-arg variant of [`subgraph_label`]: writes the full
/// `<header><br/>L<start>(-<end>)?(<br/>kind)?` label straight into
/// `out` so `emit_plain_subgraph` can assemble the surrounding
/// `subgraph <id>["<label>"]` line in a single `String` rather than
/// stacking `line_range_label` → `escape` → `base_label` → outer
/// `format!`.
pub fn subgraph_label_into(
    out: &mut String,
    sg: &VisualSubgraph,
    node_map: &HashMap<String, &VisualNode>,
    debug: bool,
) {
    base_label_into(out, sg, node_map);
    if debug {
        out.push_str("<br/>");
        out.push_str(sg.kind().as_str());
    }
}

fn base_label_into(out: &mut String, sg: &VisualSubgraph, node_map: &HashMap<String, &VisualNode>) {
    match sg.kind() {
        SubgraphKind::Function => {
            // Prefer the name baked onto the subgraph at build
            // time; the owner node may be absent after pruning even
            // when the subgraph survives. ownerName is empty when
            // the owner variable was missing at build time -- fall
            // back to the live node_map entry in that case.
            let owner_node_id = sg.owner_node_id();
            if owner_node_id.is_none() {
                // Callback-argument header takes precedence over
                // the bare `(anonymous)` fallback so the subgraph
                // is self-identifying without having to scan the
                // surrounding diagram for the matching call proxy.
                if let Some((callee, arg_index)) = sg.callback_arg() {
                    escape_into(out, callee);
                    out.push_str("(args[");
                    out.push_str(&arg_index.to_string());
                    out.push_str("])<br/>");
                } else {
                    out.push_str("(anonymous)<br/>");
                }
                line_range_label_into(out, sg);
                return;
            }
            let owner_name = sg.owner_name().unwrap_or("");
            if !owner_name.is_empty() {
                escape_into(out, owner_name);
            } else if let Some(id) = owner_node_id {
                if let Some(n) = node_map.get(id) {
                    escape_into(out, n.name());
                }
            }
            out.push_str("()<br/>");
            line_range_label_into(out, sg);
        }
        SubgraphKind::CallProxy => {
            // The proxy subgraph reuses the leaf-node label the
            // pre-subgraph implementation would have produced
            // (`render_head_expression`'s rendering of the head,
            // e.g. `run()` or `console.log()`). Falling back to the
            // empty string keeps the layout robust when the build
            // path bypasses `describe_subgraph` (test fixtures
            // primarily).
            let name = sg.call_name().unwrap_or("");
            escape_into(out, name);
            out.push_str("<br/>");
            line_range_label_into(out, sg);
        }
        SubgraphKind::Class => match sg.class_name() {
            None => {
                out.push_str("class (anonymous)<br/>");
                line_range_label_into(out, sg);
            }
            Some(name) => {
                out.push_str("class ");
                escape_into(out, name);
                out.push_str("<br/>");
                line_range_label_into(out, sg);
            }
        },
        SubgraphKind::Switch => {
            out.push_str("switch ");
            line_range_label_into(out, sg);
        }
        SubgraphKind::Case => match sg.case_test() {
            None => {
                out.push_str("default ");
                line_range_label_into(out, sg);
            }
            Some(test) => {
                out.push_str("case ");
                escape_into(out, test);
                out.push(' ');
                line_range_label_into(out, sg);
            }
        },
        SubgraphKind::If => {
            out.push_str("if ");
            line_range_label_into(out, sg);
        }
        SubgraphKind::Else => {
            out.push_str("else ");
            line_range_label_into(out, sg);
        }
        SubgraphKind::IfElseContainer => {
            if sg.has_else() {
                out.push_str("if-else ");
            } else {
                out.push_str("if ");
            }
            line_range_label_into(out, sg);
        }
        SubgraphKind::Try => {
            out.push_str("try ");
            line_range_label_into(out, sg);
        }
        SubgraphKind::Catch => {
            out.push_str("catch ");
            line_range_label_into(out, sg);
        }
        SubgraphKind::Finally => {
            out.push_str("finally ");
            line_range_label_into(out, sg);
        }
        SubgraphKind::For => {
            out.push_str("for ");
            line_range_label_into(out, sg);
        }
        SubgraphKind::While => {
            out.push_str("while ");
            line_range_label_into(out, sg);
        }
        SubgraphKind::DoWhile => {
            out.push_str("do-while ");
            line_range_label_into(out, sg);
        }
        SubgraphKind::Return => {
            out.push_str("return ");
            line_range_label_into(out, sg);
        }
        SubgraphKind::Throw => {
            out.push_str("throw ");
            line_range_label_into(out, sg);
        }
        SubgraphKind::Block => {
            out.push_str("block ");
            line_range_label_into(out, sg);
        }
        SubgraphKind::Module => {
            // The module header names the import source only; unlike
            // every other subgraph kind it carries no `L<start>-<end>`
            // range, because an import source is not a contiguous span
            // the way a function or block body is.
            out.push_str("module ");
            escape_into(out, sg.module_source().unwrap_or(""));
        }
    }
}

#[cfg(test)]
#[path = "subgraph_label_test.rs"]
mod subgraph_label_test;
