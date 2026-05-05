import { describe, expect, test } from "vitest";

import {
  doWhileTestNodeId,
  forTestNodeId,
  whileTestNodeId,
} from "./loop-test-node-id.js";

describe("whileTestNodeId", () => {
  test("composes 'while_test_<scope>_<offset>'", () => {
    expect(whileTestNodeId("scope_0", 33)).toBe("while_test_scope_0_33");
  });

  test("sanitises non-alphanumeric characters in the parent scope id", () => {
    expect(whileTestNodeId("scope.0:nested", 7)).toBe(
      "while_test_scope_0_nested_7",
    );
  });

  test("accepts an empty parent scope id", () => {
    expect(whileTestNodeId("", 12)).toBe("while_test__12");
  });
});

describe("doWhileTestNodeId", () => {
  test("composes 'do_while_test_<scope>_<offset>'", () => {
    expect(doWhileTestNodeId("scope_0", 41)).toBe("do_while_test_scope_0_41");
  });

  test("sanitises non-alphanumeric characters in the parent scope id", () => {
    expect(doWhileTestNodeId("a-b", 5)).toBe("do_while_test_a_b_5");
  });
});

describe("forTestNodeId", () => {
  test("composes 'for_test_<scope>_<offset>'", () => {
    expect(forTestNodeId("scope_1", 99)).toBe("for_test_scope_1_99");
  });

  test("sanitises non-alphanumeric characters in the parent scope id", () => {
    expect(forTestNodeId("scope#2!", 0)).toBe("for_test_scope_2__0");
  });
});
