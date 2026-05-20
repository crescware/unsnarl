use super::*;

use unsnarl_ir::serialized::serialized_ir::SERIALIZED_IR_VERSION;
use unsnarl_ir::serialized::{SerializedIR, SerializedSource};
use unsnarl_ir::Language;

struct NoopPlugin {
    name: &'static str,
}

impl UnsnarlPlugin for NoopPlugin {
    fn name(&self) -> &str {
        self.name
    }
    fn transform(&self, ir: SerializedIR) -> SerializedIR {
        ir
    }
}

fn empty_ir() -> SerializedIR {
    SerializedIR {
        version: SERIALIZED_IR_VERSION,
        source: SerializedSource {
            path: "".into(),
            language: Language::Ts,
        },
        raw: "".into(),
        scopes: vec![],
        variables: vec![],
        references: vec![],
        unused_variable_ids: vec![],
        diagnostics: vec![],
    }
}

#[test]
fn activate_all_returns_plugins_in_input_order() {
    let mut registry = PluginRegistry::new();
    registry.register("a", Box::new(NoopPlugin { name: "plugin-a" }));
    registry.register("b", Box::new(NoopPlugin { name: "plugin-b" }));
    let names = vec!["b".to_string(), "a".to_string()];
    let activated = registry.activate_all(&names).expect("activate_all ok");
    assert_eq!(activated.len(), 2);
    assert_eq!(activated[0].name(), "plugin-b");
    assert_eq!(activated[1].name(), "plugin-a");
}

#[test]
fn activate_all_passes_ir_through_each_plugin() {
    let mut registry = PluginRegistry::new();
    registry.register("a", Box::new(NoopPlugin { name: "noop" }));
    let names = vec!["a".to_string()];
    let plugins = registry.activate_all(&names).expect("activate_all ok");
    let ir = plugins
        .into_iter()
        .fold(empty_ir(), |acc, p| p.transform(acc));
    assert_eq!(ir.raw, "");
}

#[test]
fn activate_all_errors_on_unknown_plugin() {
    let registry = PluginRegistry::new();
    let names = vec!["missing".to_string()];
    let err = match registry.activate_all(&names) {
        Ok(_) => panic!("missing plugin must error"),
        Err(e) => e,
    };
    assert_eq!(err.name(), "missing");
    assert!(format!("{err}").contains("unsnarl-plugin-missing"));
}
