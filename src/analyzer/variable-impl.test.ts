import { describe, expect, test } from "vitest";

import { VariableImpl } from "../boundary/eslint-scope/variable-impl.js";
import type { Reference } from "../ir/reference/reference.js";
import type { Scope } from "../ir/scope/scope.js";
import { isUnused } from "./is-unused.js";

const fakeScope = {} as Scope;

function fakeRef(opts: {
  init: boolean;
  read: boolean;
  write: boolean;
}): Reference {
  const { init, read, write } = opts;
  return {
    init,
    isRead: () => read,
    isWrite: () => write,
    isWriteOnly: () => write && !read,
    isReadOnly: () => read && !write,
    isReadWrite: () => read && write,
  } as unknown as Reference;
}

describe("isUnused", () => {
  test("returns true when there are no references", () => {
    const v = new VariableImpl("x", fakeScope);
    expect(isUnused(v)).toBe(true);
  });

  test("returns true when only an init Write reference exists (e.g. `const a = 1;` with no reader)", () => {
    const v = new VariableImpl("a", fakeScope);
    v.references.push(fakeRef({ init: true, read: false, write: true }));
    expect(isUnused(v)).toBe(true);
  });

  test("returns false when an init Read reference is present (e.g. `const x = a;` so `a` is read in another initializer)", () => {
    const v = new VariableImpl("a", fakeScope);
    v.references.push(fakeRef({ init: true, read: false, write: true }));
    v.references.push(fakeRef({ init: true, read: true, write: false }));
    expect(isUnused(v)).toBe(false);
  });

  test("returns false when a non-init Read reference is present (e.g. `console.log(a)` after declaration)", () => {
    const v = new VariableImpl("a", fakeScope);
    v.references.push(fakeRef({ init: true, read: false, write: true }));
    v.references.push(fakeRef({ init: false, read: true, write: false }));
    expect(isUnused(v)).toBe(false);
  });

  test("returns true when a non-init Write-only reference is present (re-assignment without any read; #45)", () => {
    const v = new VariableImpl("x", fakeScope);
    v.references.push(fakeRef({ init: true, read: false, write: true }));
    v.references.push(fakeRef({ init: false, read: false, write: true }));
    expect(isUnused(v)).toBe(true);
  });

  test("returns false when a ReadWrite reference is present (e.g. `x += 1`)", () => {
    const v = new VariableImpl("x", fakeScope);
    v.references.push(fakeRef({ init: true, read: false, write: true }));
    v.references.push(fakeRef({ init: false, read: true, write: true }));
    expect(isUnused(v)).toBe(false);
  });

  // Recursive / self-resolving Read references currently count as a Read and
  // therefore keep the variable not-unused. Whether to exclude self-resolving
  // Read refs is tracked by #68.
  test("returns false when only a self-resolving Read reference exists (recursive call; #68)", () => {
    const v = new VariableImpl("foo", fakeScope);
    v.references.push(fakeRef({ init: false, read: true, write: false }));
    expect(isUnused(v)).toBe(false);
  });
});
