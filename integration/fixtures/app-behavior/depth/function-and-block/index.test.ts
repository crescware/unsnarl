import {
  NESTING_KIND,
  uniformNestingDepths,
} from "../../../../../src/serializer/nesting-kind.js";
import { fixtureSnapshot } from "../../../../fixture-snapshot.js";

fixtureSnapshot(import.meta.url);

// function depth narrow, block depth wide:
//   only the outermost function survives, every if nests freely until
//   the inner function (and everything inside it) collapses.
fixtureSnapshot(import.meta.url, {
  depths: {
    ...uniformNestingDepths(10),
    [NESTING_KIND.Function]: 1,
  },
  slug: "fn1-block10",
});

// block depth narrow, function depth wide:
//   every function renders, but the second if-body collapses, taking
//   the inner function with it (it lives inside if(b)).
fixtureSnapshot(import.meta.url, {
  depths: {
    ...uniformNestingDepths(10),
    [NESTING_KIND.If]: 1,
    [NESTING_KIND.For]: 1,
    [NESTING_KIND.While]: 1,
    [NESTING_KIND.Switch]: 1,
    [NESTING_KIND.TryCatchFinally]: 1,
    [NESTING_KIND.Block]: 1,
  },
  slug: "fn10-block1",
});

// both axes restricted to 2:
//   f1 + f2 visible; if(a) + if(b) visible; if(c) (if-depth 3) collapses.
fixtureSnapshot(import.meta.url, {
  depths: uniformNestingDepths(2),
  slug: "fn2-block2",
});
