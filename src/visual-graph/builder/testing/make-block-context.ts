import type { BlockContext } from "../../../ir/model.js";

export function makeBlockContext(
  parentType: string,
  key: string,
  parentSpanOffset = 0,
  caseTest?: string | null,
): BlockContext {
  return caseTest === undefined
    ? { parentType, key, parentSpanOffset }
    : { parentType, key, parentSpanOffset, caseTest };
}
