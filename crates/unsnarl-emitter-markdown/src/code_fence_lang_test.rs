use super::*;

#[test]
fn tsx_maps_to_tsx() {
    assert_eq!(code_fence_lang(Language::Tsx), "tsx");
}

#[test]
fn jsx_maps_to_jsx() {
    assert_eq!(code_fence_lang(Language::Jsx), "jsx");
}

#[test]
fn js_maps_to_js() {
    assert_eq!(code_fence_lang(Language::Js), "js");
}

#[test]
fn ts_maps_to_ts() {
    assert_eq!(code_fence_lang(Language::Ts), "ts");
}
