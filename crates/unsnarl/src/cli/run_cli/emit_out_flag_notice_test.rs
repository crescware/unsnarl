use super::*;

fn capture(out_dir: Option<&str>) -> String {
    let mut buf = Vec::new();
    emit_out_flag_notice(out_dir, &mut buf);
    String::from_utf8(buf).expect("output should be valid UTF-8")
}

#[test]
fn writes_nothing_when_out_dir_is_none() {
    assert_eq!(capture(None), "");
}

#[test]
fn writes_nothing_for_a_dir_path_without_an_extension() {
    assert_eq!(capture(Some("build/out")), "");
}

#[test]
fn writes_nothing_for_a_dotfile_style_basename_no_extname() {
    assert_eq!(capture(Some(".cache")), "");
}

#[test]
fn writes_a_notice_when_a_dir_paths_basename_has_an_extension() {
    assert_eq!(
        capture(Some("graph.mmd")),
        "uns: notice: -o 'graph.mmd' is treated as a directory name; use --out-file to write to that path as a file.\n",
    );
}

#[test]
fn writes_a_notice_for_a_deep_dir_path_whose_tail_has_an_extension() {
    assert_eq!(
        capture(Some("build/out.json")),
        "uns: notice: -o 'build/out.json' is treated as a directory name; use --out-file to write to that path as a file.\n",
    );
}
