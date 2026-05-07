import { uniformNestingDepths } from "../../../../src/serializer/nesting-kind.js";
import { fixtureSnapshot } from "../../../fixture-snapshot.js";

fixtureSnapshot(import.meta.url);

// Combined pruning + depth: -r inner -A 2 -B 1 --depth 1.
//   The asymmetric A/B keeps the CLI form as `-A 2 -B 1` (not `-C N`),
//   and the non-default depth fits onto the same `## Query` line.
fixtureSnapshot(import.meta.url, {
  pruning: { roots: "inner", descendants: 2, ancestors: 1 },
  depths: uniformNestingDepths(1),
  slug: "inner-A2-B1-d1",
});
