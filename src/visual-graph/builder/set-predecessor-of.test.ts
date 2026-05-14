import { describe, expect, test } from "vitest";

import { asScopeId } from "../../ir/serialized/scope-id.js";
import type { SerializedScope } from "../../ir/serialized/serialized-scope.js";
import { setPredecessorOf } from "./set-predecessor-of.js";
import { baseScope } from "./testing/make-scope.js";
import { baseWriteOp } from "./testing/make-write-op.js";

describe("setPredecessorOf", () => {
  test("returns variable node id when there are no prior ops", () => {
    const scope = { ...baseScope(), id: asScopeId("s") };
    const map = new Map<string, SerializedScope>([[scope.id, scope]]);
    const op = { ...baseWriteOp(), refId: "r1", scopeId: "s" };
    expect(setPredecessorOf(op, [op], map)).toEqual("n_v");
  });

  test("returns prior op's writeOp node id when prior op is in the same scope", () => {
    const scope = { ...baseScope(), id: asScopeId("s") };
    const map = new Map<string, SerializedScope>([[scope.id, scope]]);
    const prev = { ...baseWriteOp(), refId: "rPrev", scopeId: "s" };
    const op = { ...baseWriteOp(), refId: "rCur", scopeId: "s" };
    expect(setPredecessorOf(op, [prev, op], map)).toEqual("wr_rPrev");
  });

  test("returns prior op's writeOp node id when prior op is in an ancestor scope", () => {
    const root = { ...baseScope(), id: asScopeId("root") };
    const child = {
      ...baseScope(),
      id: asScopeId("child"),
      upper: asScopeId("root"),
    };
    const map = new Map<string, SerializedScope>([
      [root.id, root],
      [child.id, child],
    ]);
    const prev = { ...baseWriteOp(), refId: "rPrev", scopeId: "root" };
    const op = { ...baseWriteOp(), refId: "rCur", scopeId: "child" };
    expect(setPredecessorOf(op, [prev, op], map)).toEqual("wr_rPrev");
  });

  test("returns variable node id when prior op is in a sibling scope", () => {
    const root = { ...baseScope(), id: asScopeId("root") };
    const sibA = {
      ...baseScope(),
      id: asScopeId("a"),
      upper: asScopeId("root"),
    };
    const sibB = {
      ...baseScope(),
      id: asScopeId("b"),
      upper: asScopeId("root"),
    };
    const map = new Map<string, SerializedScope>([
      [root.id, root],
      [sibA.id, sibA],
      [sibB.id, sibB],
    ]);
    const prev = { ...baseWriteOp(), refId: "rPrev", scopeId: "a" };
    const op = { ...baseWriteOp(), refId: "rCur", scopeId: "b" };
    expect(setPredecessorOf(op, [prev, op], map)).toEqual("n_v");
  });

  test("returns variable node id when op is missing from ops list", () => {
    const scope = { ...baseScope(), id: asScopeId("s") };
    const map = new Map<string, SerializedScope>([[scope.id, scope]]);
    const op = { ...baseWriteOp(), refId: "rCur", scopeId: "s" };
    expect(setPredecessorOf(op, [], map)).toEqual("n_v");
  });
});
