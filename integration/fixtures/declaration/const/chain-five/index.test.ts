import { fixtureSnapshot } from "../../../../fixture-snapshot.js";

fixtureSnapshot(import.meta.url);

// a -> b -> c -> d -> e (linear read chain), e is unused.
for (const v of [
  { roots: "a", descendants: 1, ancestors: 1, slug: "a-c1" },
  { roots: "e", descendants: 1, ancestors: 1, slug: "e-c1" },
  { roots: "c", descendants: 1, ancestors: 0, slug: "c-a1" },
  { roots: "c", descendants: 0, ancestors: 1, slug: "c-b1" },
] as const) {
  fixtureSnapshot(import.meta.url, {
    pruning: {
      roots: v.roots,
      descendants: v.descendants,
      ancestors: v.ancestors,
    },
    slug: v.slug,
  });
}
