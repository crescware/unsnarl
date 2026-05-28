use std::collections::BTreeSet;
use std::env;
use std::fs;
use std::path::Path;

use regex::Regex;

fn extract_enum_variants(lib_rs: &str) -> Vec<String> {
    let mut variants = Vec::new();
    let mut in_enum = false;
    let variant_re = Regex::new(r"^\s+([A-Z][A-Za-z0-9]+),?\s*$")
        .expect("AstType variant regex is a static literal and must compile");
    for line in lib_rs.lines() {
        if line.contains("pub enum AstType {") {
            in_enum = true;
            continue;
        }
        if in_enum {
            if line.starts_with('}') {
                break;
            }
            if let Some(caps) = variant_re.captures(line) {
                variants.push(caps[1].to_string());
            }
        }
    }
    variants
}

const OXC_TYPE_ALIAS_EXPANSIONS: &[(&str, &[&str])] = &[
    (
        "FunctionType",
        &[
            "FunctionDeclaration",
            "FunctionExpression",
            "TSDeclareFunction",
            "TSEmptyBodyFunctionExpression",
        ],
    ),
    ("ClassType", &["ClassDeclaration", "ClassExpression"]),
    (
        "MethodDefinitionType",
        &["MethodDefinition", "TSAbstractMethodDefinition"],
    ),
    (
        "PropertyDefinitionType",
        &["PropertyDefinition", "TSAbstractPropertyDefinition"],
    ),
    (
        "AccessorPropertyType",
        &["AccessorProperty", "TSAbstractAccessorProperty"],
    ),
];

fn extract_oxc_types(types_dts: &str) -> (Vec<String>, Vec<String>) {
    let literal_re = Regex::new(r#"(?m)^\s+type:\s*"([^"]+)";"#)
        .expect("oxc types.d.ts literal type regex is a static literal and must compile");
    let alias_re = Regex::new(r"(?m)^\s+type:\s*([A-Z][A-Za-z0-9]+);")
        .expect("oxc types.d.ts alias regex is a static literal and must compile");
    let alias_map: std::collections::HashMap<&str, &[&str]> =
        OXC_TYPE_ALIAS_EXPANSIONS.iter().copied().collect();

    let mut types = BTreeSet::new();
    let mut unknown_aliases = BTreeSet::new();

    for caps in literal_re.captures_iter(types_dts) {
        types.insert(caps[1].to_string());
    }
    for caps in alias_re.captures_iter(types_dts) {
        let alias_name = &caps[1];
        if let Some(expansion) = alias_map.get(alias_name) {
            for &v in *expansion {
                types.insert(v.to_string());
            }
        } else {
            unknown_aliases.insert(alias_name.to_string());
        }
    }
    let mut types_vec: Vec<String> = types.into_iter().collect();
    types_vec.sort();
    let mut unknown_vec: Vec<String> = unknown_aliases.into_iter().collect();
    unknown_vec.sort();
    (types_vec, unknown_vec)
}

fn main() {
    let manifest_dir =
        env::var("CARGO_MANIFEST_DIR").expect("Cargo guarantees CARGO_MANIFEST_DIR in build.rs");
    let out_dir = env::var("OUT_DIR").expect("Cargo guarantees OUT_DIR in build.rs");

    let lib_rs_path = Path::new(&manifest_dir).join("src/lib.rs");
    let lib_rs = fs::read_to_string(&lib_rs_path).expect("cannot read src/lib.rs");
    let variants = extract_enum_variants(&lib_rs);

    let workspace_root = Path::new(&manifest_dir)
        .parent()
        .and_then(Path::parent)
        .expect("workspace root is two levels above crate dir");
    let types_dts_path =
        workspace_root.join("fixtures/oxc-parity/node_modules/@oxc-project/types/types.d.ts");
    let (oxc_entries, unknown_aliases) = if types_dts_path.exists() {
        let types_dts = fs::read_to_string(&types_dts_path).expect("cannot read types.d.ts");
        extract_oxc_types(&types_dts)
    } else {
        (Vec::new(), Vec::new())
    };

    let dest = Path::new(&out_dir).join("generated_parity.rs");
    let mut code = String::new();

    code.push_str("pub const AST_TYPE_ENUM_VARIANTS: &[&str] = &[\n");
    for v in &variants {
        code.push_str(&format!("    \"{v}\",\n"));
    }
    code.push_str("];\n\n");

    code.push_str("pub const OXC_TYPES_DTS_ENTRIES: &[&str] = &[\n");
    for e in &oxc_entries {
        code.push_str(&format!("    \"{e}\",\n"));
    }
    code.push_str("];\n\n");

    code.push_str("pub const OXC_UNKNOWN_ALIASES: &[&str] = &[\n");
    for a in &unknown_aliases {
        code.push_str(&format!("    \"{a}\",\n"));
    }
    code.push_str("];\n");

    fs::write(&dest, code)
        .unwrap_or_else(|e| panic!("failed to write generated parity table to {dest:?}: {e}"));

    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed={}", types_dts_path.display());
}
