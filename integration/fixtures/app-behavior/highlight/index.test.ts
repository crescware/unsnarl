import { fixtureSnapshot } from "../../../fixture-snapshot.js";

// Baseline (no -H): every node renders in the default palette.
fixtureSnapshot(import.meta.url);

// -H a (no -r): the full graph is rendered; every node named `a`
// (declaration on L3 + read on L4) is painted yellow, and any edge
// touching one of those nodes joins the highlight set.
fixtureSnapshot(import.meta.url, {
  highlight: { mode: "queries", raw: "a" },
  slug: "name-a",
  label: "highlight-only: a",
});

// -H 8 (no -r): line-based highlight target the `return x + seed;`
// expression. Both the `x` read and the `seed` read are on L8, so the
// two read nodes -- and the edges that touch them -- get coloured.
fixtureSnapshot(import.meta.url, {
  highlight: { mode: "queries", raw: "8" },
  slug: "line-8",
  label: "highlight-only: line 8",
});

// -H zzz (no -r): the query matches no node. Output should be
// indistinguishable from the baseline (no `style` / `linkStyle` lines).
fixtureSnapshot(import.meta.url, {
  highlight: { mode: "queries", raw: "zzz" },
  slug: "miss",
  label: "highlight-only: query matches nothing",
});

// -r b -C 1 -H (roots mode): the pruning keeps `b`'s 1-generation
// neighborhood and the highlight follows the pruning roots, so the
// `b` nodes are coloured along with the edges that connect them to
// their immediate neighbours.
fixtureSnapshot(import.meta.url, {
  pruning: { roots: "b", descendants: 1, ancestors: 1 },
  highlight: { mode: "roots" },
  slug: "roots-b-c1",
  label: "pruned+highlight: -r b -C 1 -H",
});

// -r b -C 1 -H c: pruning is still driven by `b`, but the highlight
// targets `c`. Because c sits inside b's 1-gen window the pruned graph
// keeps it, so the c nodes get painted while b stays in its default
// colour.
fixtureSnapshot(import.meta.url, {
  pruning: { roots: "b", descendants: 1, ancestors: 1 },
  highlight: { mode: "queries", raw: "c" },
  slug: "roots-b-c1-highlight-c",
  label: "pruned+highlight: -r b -C 1 -H c",
});

// -r b -C 1 -H seed: the highlight target lives OUTSIDE the pruned
// graph (seed is past b's 1-gen window), so the resolved highlight id
// set is empty and the output should match the plain pruned variant.
fixtureSnapshot(import.meta.url, {
  pruning: { roots: "b", descendants: 1, ancestors: 1 },
  highlight: { mode: "queries", raw: "seed" },
  slug: "roots-b-c1-highlight-outside",
  label: "pruned+highlight: target outside pruned graph",
});
