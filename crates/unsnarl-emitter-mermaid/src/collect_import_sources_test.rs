use std::collections::HashMap;

use unsnarl_visual_graph::visual_node::{
    BindingNodeKind, BindingVisualNode, SyntheticNodeKind, SyntheticVisualNode, VisualNode,
};

use super::collect_import_sources;
use crate::mermaid_fixtures::{base_simple_binding, base_simple_synthetic};

fn synthetic(id: &str, kind: SyntheticNodeKind) -> VisualNode {
    SyntheticVisualNode {
        id: id.to_string(),
        ..base_simple_synthetic(kind)
    }
    .into()
}

fn binding(id: &str, kind: BindingNodeKind) -> VisualNode {
    BindingVisualNode {
        id: id.to_string(),
        ..base_simple_binding(kind)
    }
    .into()
}

#[test]
fn collects_ids_of_module_source_and_import_intermediate_nodes() {
    let a = synthetic("mod_a", SyntheticNodeKind::SyntheticModuleSource);
    let b = synthetic("import_b", SyntheticNodeKind::SyntheticImportIntermediate);
    let x: VisualNode = crate::mermaid_fixtures::base_const_binding().into();
    let mut map: HashMap<String, &VisualNode> = HashMap::new();
    map.insert("mod_a".to_string(), &a);
    map.insert("import_b".to_string(), &b);
    map.insert("n_x".to_string(), &x);
    let mut got: Vec<String> = collect_import_sources(&map).into_iter().collect();
    got.sort();
    assert_eq!(got, vec!["import_b".to_string(), "mod_a".to_string()]);
}

#[test]
fn excludes_other_synthetic_kinds() {
    let sink = synthetic("module_root", SyntheticNodeKind::SyntheticModuleSink);
    let mut map: HashMap<String, &VisualNode> = HashMap::new();
    map.insert("module_root".to_string(), &sink);
    assert_eq!(collect_import_sources(&map).len(), 0);
}

#[test]
fn excludes_non_synthetic_kinds() {
    let x: VisualNode = crate::mermaid_fixtures::base_const_binding().into();
    let f = binding("n_f", BindingNodeKind::FunctionDeclaration);
    let mut map: HashMap<String, &VisualNode> = HashMap::new();
    map.insert("n_x".to_string(), &x);
    map.insert("n_f".to_string(), &f);
    assert_eq!(collect_import_sources(&map).len(), 0);
}

#[test]
fn empty_map_returns_empty_set() {
    let map: HashMap<String, &VisualNode> = HashMap::new();
    assert_eq!(collect_import_sources(&map).len(), 0);
}
