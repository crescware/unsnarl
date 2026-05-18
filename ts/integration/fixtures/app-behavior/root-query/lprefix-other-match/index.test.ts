import {
  fixtureResolutions,
  fixtureSnapshot,
} from "../../../../fixture-snapshot.js";

fixtureSnapshot(import.meta.url);

// -r L12 → line with notice ([Ll]<n> exists with different casing, exact name does not).
fixtureSnapshot(import.meta.url, {
  pruning: { roots: "L12", descendants: 1, ancestors: 1 },
  slug: "L12-c1",
});
fixtureResolutions(import.meta.url, {
  roots: "L12",
  descendants: 1,
  ancestors: 1,
  expected: [{ raw: "L12", line: 12, name: "L12", resolvedAs: "line" }],
});
