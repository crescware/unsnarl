import { sanitize } from "./sanitize.js";

export function whileTestNodeId(parentScopeId: string, offset: number): string {
  return `while_test_${sanitize(parentScopeId)}_${offset}`;
}

export function doWhileTestNodeId(
  parentScopeId: string,
  offset: number,
): string {
  return `do_while_test_${sanitize(parentScopeId)}_${offset}`;
}

export function forTestNodeId(parentScopeId: string, offset: number): string {
  return `for_test_${sanitize(parentScopeId)}_${offset}`;
}
