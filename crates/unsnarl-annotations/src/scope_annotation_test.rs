use unsnarl_ir::nesting_kind::{NestingDepth, NestingDepths};

use super::*;

#[test]
fn emits_keys_in_declared_order() {
    let ann = ScopeAnnotation {
        block_context: None,
        falls_through: false,
        exits_function: false,
        nesting_depths: NestingDepths::uniform(NestingDepth(0)),
    };
    let json = serde_json::to_string(&ann).unwrap();
    assert_eq!(
        json,
        r#"{"blockContext":null,"fallsThrough":false,"exitsFunction":false,"nestingDepths":{"function":0,"if":0,"for":0,"while":0,"switch":0,"try-catch-finally":0,"block":0}}"#
    );
}
