import type { BlockContext } from "../../../ir/model.js";

export function makeBlockContext(
  parentType: string,
  key: string,
  parentSpanOffset = 0,
  caseTest?: string | null,
): BlockContext {
  const ctx: BlockContext = { parentType, key, parentSpanOffset };
  if (caseTest !== undefined) {
    ctx.caseTest = caseTest;
  }
  return ctx;
}
