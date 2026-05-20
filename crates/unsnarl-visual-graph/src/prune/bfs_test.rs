use super::*;

fn adj_of(pairs: &[(&str, &str)]) -> HashMap<String, Vec<String>> {
    let mut out: HashMap<String, Vec<String>> = HashMap::new();
    for (from, to) in pairs {
        out.entry(from.to_string())
            .or_default()
            .push(to.to_string());
    }
    out
}

fn set_of(items: &[&str]) -> HashSet<String> {
    items.iter().map(|s| s.to_string()).collect()
}

fn sorted(set: HashSet<String>) -> Vec<String> {
    let mut v: Vec<String> = set.into_iter().collect();
    v.sort();
    v
}

#[test]
fn max_depth_zero_returns_the_start_set_unchanged() {
    let adj = adj_of(&[("a", "b")]);
    assert_eq!(sorted(bfs(&set_of(&["a"]), &adj, 0)), vec!["a"]);
}

#[test]
fn max_depth_one_reaches_direct_neighbors() {
    let adj = adj_of(&[("a", "b"), ("b", "c")]);
    assert_eq!(sorted(bfs(&set_of(&["a"]), &adj, 1)), vec!["a", "b"]);
}

#[test]
fn max_depth_two_reaches_grandchildren() {
    let adj = adj_of(&[("a", "b"), ("b", "c")]);
    assert_eq!(sorted(bfs(&set_of(&["a"]), &adj, 2)), vec!["a", "b", "c"]);
}

#[test]
fn multiple_start_nodes_union_their_reachable_sets() {
    let adj = adj_of(&[("a", "x"), ("b", "y")]);
    assert_eq!(
        sorted(bfs(&set_of(&["a", "b"]), &adj, 1)),
        vec!["a", "b", "x", "y"]
    );
}

#[test]
fn cycles_do_not_loop_infinitely() {
    let adj = adj_of(&[("a", "b"), ("b", "a")]);
    assert_eq!(sorted(bfs(&set_of(&["a"]), &adj, 10)), vec!["a", "b"]);
}

#[test]
fn disconnected_nodes_stay_unreached() {
    let adj = adj_of(&[("a", "b")]);
    assert_eq!(sorted(bfs(&set_of(&["a"]), &adj, 5)), vec!["a", "b"]);
}

#[test]
fn frontier_exhaustion_bails_early_without_iterating_extra_depths() {
    let adj = adj_of(&[("a", "b")]);
    assert_eq!(sorted(bfs(&set_of(&["a"]), &adj, 100)), vec!["a", "b"]);
}

#[test]
fn negative_max_depth_behaves_like_zero() {
    let adj = adj_of(&[("a", "b")]);
    assert_eq!(sorted(bfs(&set_of(&["a"]), &adj, -1)), vec!["a"]);
}
