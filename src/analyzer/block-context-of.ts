import { parse } from "valibot";

import type { PathEntry } from "../boundary/eslint-scope/walk/path-entry.js";
import type { AstNode } from "../ir/primitive/ast-node.js";
import { other$ } from "../ir/scope/block-context-kind.js";
import { blockContext$, type BlockContext } from "../ir/scope/block-context.js";
import { asAstType } from "../parser/ast-type.js";
import { ifChainRootOffset } from "./if-chain-root-offset.js";

export function blockContextOf(
  parent: AstNode | null,
  key: string | null,
  path: readonly PathEntry[],
): BlockContext | null {
  if (!parent || key === null) {
    return null;
  }
  const chainRoot = ifChainRootOffset(parent, key, path);
  return parse(blockContext$, {
    kind: other$.literal,
    parentType: asAstType(parent.type),
    key,
    parentSpanOffset: parent.start ?? 0,
    ifChainRootOffset: chainRoot,
  });
}
