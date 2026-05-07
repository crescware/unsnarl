import { uniformNestingDepths } from "../../../../../src/serializer/nesting-kind.js";
import { fixtureSnapshot } from "../../../../fixture-snapshot.js";

fixtureSnapshot(import.meta.url);

for (const n of [1, 2, 5, 6, 10] as const) {
  fixtureSnapshot(import.meta.url, {
    depths: uniformNestingDepths(n),
    slug: String(n),
  });
}
