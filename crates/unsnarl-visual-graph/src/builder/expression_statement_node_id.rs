//! Mirrors `ts/src/visual-graph/builder/expression-statement-node-id.ts`.

pub fn expression_statement_node_id(offset: u32) -> String {
    format!("expr_stmt_{offset}")
}
