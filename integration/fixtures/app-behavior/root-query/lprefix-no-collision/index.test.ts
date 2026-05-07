import {
  fixtureResolutions,
  fixtureSnapshot,
} from "../../../../fixture-snapshot.js";

fixtureSnapshot(import.meta.url);

// -r L12 → silent line (no [Ll]<n> identifier exists, falls back to line query).
fixtureSnapshot(import.meta.url, {
  pruning: { roots: "L12", descendants: 1, ancestors: 1 },
  slug: "L12-c1",
});
fixtureResolutions(import.meta.url, {
  roots: "L12",
  descendants: 1,
  ancestors: 1,
  expected: [],
});
