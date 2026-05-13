import { fixtureSnapshot } from "../../../fixture-snapshot.js";

// Baseline (no -H): the `return { a, b }` shorthand reads both
// identifiers, so the IR/graph carries one read reference per
// shorthand entry alongside the two const declarations.
fixtureSnapshot(import.meta.url);

// -H a (no -r): the declaration on L2 plus the shorthand read inside
// the return on L4 share the name `a`, so both nodes get the highlight
// style. Whether the inner-scope `b` (untouched by the highlight)
// keeps its default paint is the interesting bit -- shorthand property
// keys should NOT count as references to `a`.
fixtureSnapshot(import.meta.url, {
  highlight: { mode: "queries", raw: "a" },
  slug: "name-a",
  label: "highlight-only: a",
});

// -r a -C 1 -H (roots mode): the key contrast with `-H a` -- this
// produces a different highlight set, not just a different graph
// scope. Pruning matches the `a` declaration only because a bare
// `-r a` is filtered through `NAME_QUERY_EXCLUDED` (which drops
// `WriteOp` / `ReturnUse` to keep `-r counter` from lighting up
// every assignment / JSX use). Highlight in roots mode mirrors that
// match set verbatim, so the shorthand `a` read in the return
// stays UNCOLORED here even though it sits next to the highlighted
// declaration and inside the pruned graph. The connecting edge
// still picks up the highlight stroke because one endpoint -- the
// declaration -- is in the set.
fixtureSnapshot(import.meta.url, {
  pruning: { roots: "a", descendants: 1, ancestors: 1 },
  highlight: { mode: "roots" },
  slug: "roots-a-c1",
  label: "pruned+highlight: -r a -C 1 -H",
});

// -r a -C 1 -H a (queries mode with the same identifier): pruning
// still narrows the graph to `a`'s 1-gen neighborhood, but the
// explicit `-H a` opts into the looser highlight matcher and paints
// every `a` node inside the pruned graph -- declaration AND the
// shorthand `ReturnUse`. Putting this side by side with the
// `roots-a-c1` variant above is the cleanest way to see that the
// roots / queries split is a real semantic divergence, not just
// syntactic sugar: same `-r`, same input, same yellow palette, but a
// different set of nodes ends up coloured because the explicit form
// passes through the highlight-specific matcher.
fixtureSnapshot(import.meta.url, {
  pruning: { roots: "a", descendants: 1, ancestors: 1 },
  highlight: { mode: "queries", raw: "a" },
  slug: "roots-a-c1-highlight-a",
  label: "pruned+highlight: -r a -C 1 -H a",
});
