import { fixtureSnapshot } from "../../../fixture-snapshot.js";

fixtureSnapshot(import.meta.url);

// -C 10 (both 10) is the implicit default when -r is given without
// -A/-B/-C; -A 1 (descendants only) and -B 1 (ancestors only) probe the
// single-direction radii. -A and -B are intentionally never combined here.
for (const [roots, slugBase] of [
  ["10", "r10"],
  ["10-11", "r10-11"],
  ["10-12", "r10-12"],
  ["19", "r19"],
  ["23", "r23"],
  ["24", "r24"],
] as const) {
  fixtureSnapshot(import.meta.url, {
    roots,
    descendants: 10,
    ancestors: 10,
    slug: slugBase,
    label: `--roots ${roots}`,
  });
  fixtureSnapshot(import.meta.url, {
    roots,
    descendants: 1,
    ancestors: 0,
    slug: `${slugBase}-a1`,
    label: `--roots ${roots} -A 1`,
  });
  fixtureSnapshot(import.meta.url, {
    roots,
    descendants: 0,
    ancestors: 1,
    slug: `${slugBase}-b1`,
    label: `--roots ${roots} -B 1`,
  });
}
