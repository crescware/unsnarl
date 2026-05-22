//! The signature is `(u32, usize)`, so "depth < 1" collapses to
//! "depth == 0" (the only non-positive u32) and "negative palette
//! length" is unrepresentable.

use super::nest_palette_index;

#[test]
fn depth_1_with_a_non_empty_palette_maps_to_index_0() {
    assert_eq!(nest_palette_index(1, 4), 0);
}

#[test]
fn depths_within_palette_length_map_one_to_one_onto_zero_based_indices() {
    assert_eq!(nest_palette_index(2, 4), 1);
    assert_eq!(nest_palette_index(3, 4), 2);
    assert_eq!(nest_palette_index(4, 4), 3);
}

#[test]
fn depth_beyond_palette_length_wraps_back_to_the_start() {
    assert_eq!(nest_palette_index(5, 4), 0);
    assert_eq!(nest_palette_index(6, 4), 1);
    assert_eq!(nest_palette_index(9, 4), 0);
}

#[test]
fn palette_of_length_1_always_maps_to_index_0() {
    assert_eq!(nest_palette_index(1, 1), 0);
    assert_eq!(nest_palette_index(7, 1), 0);
}

#[test]
#[should_panic(expected = "paletteLength must be > 0")]
fn rejects_a_palette_length_of_zero() {
    let _ = nest_palette_index(1, 0);
}

#[test]
#[should_panic(expected = "depth must be >= 1")]
fn rejects_a_depth_of_zero() {
    let _ = nest_palette_index(0, 4);
}
