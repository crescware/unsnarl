import { fixtureSnapshot } from "../../../fixture-snapshot.js";

fixtureSnapshot(import.meta.url);
fixtureSnapshot(import.meta.url, {
  roots: "10",
  descendants: 1,
  ancestors: 1,
  slug: "r10-c1",
});
fixtureSnapshot(import.meta.url, {
  roots: "counter",
  descendants: 2,
  ancestors: 0,
  slug: "counter-a2",
});
fixtureSnapshot(import.meta.url, {
  roots: "counter",
  descendants: 0,
  ancestors: 2,
  slug: "counter-b2",
});
