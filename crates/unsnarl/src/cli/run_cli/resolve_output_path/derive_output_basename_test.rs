use std::path::Path;

use super::*;

use unsnarl_ir::SourceLine;
use unsnarl_root_query::GenerationCount;

fn name(n: &str) -> ParsedRootQuery {
    ParsedRootQuery::Name {
        name: n.to_string(),
        raw: n.to_string(),
    }
}

fn line(n: u32) -> ParsedRootQuery {
    ParsedRootQuery::Line {
        line: SourceLine(n),
        raw: n.to_string(),
    }
}

fn line_name(n: u32, id: &str) -> ParsedRootQuery {
    ParsedRootQuery::LineName {
        line: SourceLine(n),
        name: id.to_string(),
        raw: format!("{n}:{id}"),
    }
}

fn range(s: u32, e: u32) -> ParsedRootQuery {
    ParsedRootQuery::Range {
        start: SourceLine(s),
        end: SourceLine(e),
        raw: format!("{s}-{e}"),
    }
}

fn range_name(s: u32, e: u32, id: &str) -> ParsedRootQuery {
    ParsedRootQuery::RangeName {
        start: SourceLine(s),
        end: SourceLine(e),
        name: id.to_string(),
        raw: format!("{s}-{e}:{id}"),
    }
}

mod root_tokenization {
    use super::*;

    #[test]
    fn kind_name_uses_identifier() {
        let actual = derive_output_basename(&[name("value")], None, None, None, Path::new(""));
        assert_eq!(actual, "value");
    }

    #[test]
    fn kind_line_is_l_n() {
        let actual = derive_output_basename(&[line(42)], None, None, None, Path::new(""));
        assert_eq!(actual, "l42");
    }

    #[test]
    fn kind_line_name_is_l_n_dash_id_with_single_hyphen() {
        let actual =
            derive_output_basename(&[line_name(42, "render")], None, None, None, Path::new(""));
        assert_eq!(actual, "l42-render");
    }

    #[test]
    fn kind_range_is_l_n_dash_m() {
        let actual = derive_output_basename(&[range(10, 12)], None, None, None, Path::new(""));
        assert_eq!(actual, "l10-12");
    }

    #[test]
    fn kind_range_name_is_l_n_dash_m_dash_id() {
        let actual = derive_output_basename(
            &[range_name(10, 12, "render")],
            None,
            None,
            None,
            Path::new(""),
        );
        assert_eq!(actual, "l10-12-render");
    }

    #[test]
    fn multiple_roots_joined_with_plus() {
        let actual = derive_output_basename(
            &[name("value"), name("foo")],
            None,
            None,
            None,
            Path::new(""),
        );
        assert_eq!(actual, "value+foo");
    }

    #[test]
    fn multiple_roots_mixing_kinds() {
        let actual = derive_output_basename(
            &[line_name(42, "render"), name("foo")],
            None,
            None,
            None,
            Path::new(""),
        );
        assert_eq!(actual, "l42-render+foo");
    }
}

mod radius_suffix_inclusion {
    use super::*;

    #[test]
    fn no_radius_flag_yields_no_suffix() {
        let actual = derive_output_basename(&[name("value")], None, None, None, Path::new(""));
        assert_eq!(actual, "value");
    }

    #[test]
    fn descendants_only_yields_a_n() {
        let actual = derive_output_basename(
            &[name("value")],
            Some(GenerationCount(1)),
            None,
            None,
            Path::new(""),
        );
        assert_eq!(actual, "value-a1");
    }

    #[test]
    fn ancestors_only_yields_b_n() {
        let actual = derive_output_basename(
            &[name("param")],
            None,
            Some(GenerationCount(2)),
            None,
            Path::new(""),
        );
        assert_eq!(actual, "param-b2");
    }

    #[test]
    fn context_only_yields_c_n() {
        let actual = derive_output_basename(
            &[range(10, 12)],
            None,
            None,
            Some(GenerationCount(2)),
            Path::new(""),
        );
        assert_eq!(actual, "l10-12-c2");
    }

    #[test]
    fn a_and_b_yields_a_n_b_m() {
        let actual = derive_output_basename(
            &[name("v")],
            Some(GenerationCount(1)),
            Some(GenerationCount(2)),
            None,
            Path::new(""),
        );
        assert_eq!(actual, "v-a1-b2");
    }

    #[test]
    fn b_and_c_yields_alphabetical_b_n_c_m() {
        let actual = derive_output_basename(
            &[name("v")],
            None,
            Some(GenerationCount(2)),
            Some(GenerationCount(3)),
            Path::new(""),
        );
        assert_eq!(actual, "v-b2-c3");
    }

    #[test]
    fn c_and_a_yields_alphabetical_a_n_c_m() {
        let actual = derive_output_basename(
            &[name("v")],
            Some(GenerationCount(7)),
            None,
            Some(GenerationCount(3)),
            Path::new(""),
        );
        assert_eq!(actual, "v-a7-c3");
    }

    #[test]
    fn a_and_b_drop_c_from_the_filename() {
        let actual = derive_output_basename(
            &[name("v")],
            Some(GenerationCount(1)),
            Some(GenerationCount(2)),
            Some(GenerationCount(3)),
            Path::new(""),
        );
        assert_eq!(actual, "v-a1-b2");
    }
}

mod input_file_fallback {
    use super::*;

    #[test]
    fn strips_ts_extension() {
        let actual = derive_output_basename(&[], None, None, None, Path::new("foo.ts"));
        assert_eq!(actual, "foo");
    }

    #[test]
    fn preserves_camel_case_basename() {
        let actual = derive_output_basename(&[], None, None, None, Path::new("fooBar.ts"));
        assert_eq!(actual, "fooBar");
    }

    #[test]
    fn preserves_kebab_case_basename() {
        let actual = derive_output_basename(&[], None, None, None, Path::new("foo-bar.ts"));
        assert_eq!(actual, "foo-bar");
    }

    #[test]
    fn strips_tsx_extension() {
        let actual = derive_output_basename(&[], None, None, None, Path::new("Component.tsx"));
        assert_eq!(actual, "Component");
    }

    #[test]
    fn strips_only_the_last_extension() {
        let actual = derive_output_basename(&[], None, None, None, Path::new("foo.test.ts"));
        assert_eq!(actual, "foo.test");
    }

    #[test]
    fn no_extension_keeps_full_basename() {
        let actual = derive_output_basename(&[], None, None, None, Path::new("Makefile"));
        assert_eq!(actual, "Makefile");
    }

    #[test]
    fn strips_path_components() {
        let actual = derive_output_basename(&[], None, None, None, Path::new("src/deep/foo.ts"));
        assert_eq!(actual, "foo");
    }
}
