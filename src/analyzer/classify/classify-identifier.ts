import type { AstNode } from "../../ir/primitive/ast-node.js";
import { ReferenceFlags } from "../../ir/reference/reference-flags.js";
import type { PathEntry } from "../walk/walk.js";
import { classifyOrdinaryReference } from "./classify-ordinary-reference.js";
import type { ClassifyResult } from "./classify-result.js";
import { findBindingRootContext } from "./find-binding-root-context.js";
import { isDirectBinding } from "./is-direct-binding.js";
import { isPatternStep } from "./is-pattern-step.js";
import { isSkipContext } from "./is-skip-context.js";
import { reference } from "./reference.js";

export function classifyIdentifier(
  parent: AstNode | null,
  key: string | null,
  path: readonly PathEntry[],
): ClassifyResult {
  if (!parent) {
    return reference(ReferenceFlags.Read, false);
  }

  const t = parent.type;

  if (isSkipContext(t, key, parent)) {
    return { kind: "skip" };
  }

  if (isDirectBinding(t, key)) {
    return { kind: "binding" };
  }

  if (isPatternStep(parent, path, path.length - 1)) {
    const root = findBindingRootContext(parent, key, path);
    if (root === "var" || root === "param" || root === "catch") {
      return { kind: "binding" };
    }
    if (root === "assign") {
      return reference(ReferenceFlags.Write, false);
    }
  }

  return classifyOrdinaryReference(t, key, parent);
}
