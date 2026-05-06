import {
  fixtureResolutions,
  fixtureSnapshot,
} from "../../../../fixture-snapshot.js";

fixtureSnapshot(import.meta.url);

// -r L12 → identifier (the source declares `const L12 = 1` exactly).
fixtureSnapshot(import.meta.url, {
  roots: "L12",
  descendants: 1,
  ancestors: 1,
  slug: "L12-c1",
});
fixtureResolutions(import.meta.url, {
  roots: "L12",
  descendants: 1,
  ancestors: 1,
  expected: [{ raw: "L12", line: 12, name: "L12", resolvedAs: "name" }],
});

// -r L1-3 → range, no resolution (hyphenated form bypasses the resolver).
fixtureSnapshot(import.meta.url, {
  roots: "L1-3",
  descendants: 1,
  ancestors: 1,
  slug: "L1-3-c1",
});
fixtureResolutions(import.meta.url, {
  roots: "L1-3",
  descendants: 1,
  ancestors: 1,
  expected: [],
});
