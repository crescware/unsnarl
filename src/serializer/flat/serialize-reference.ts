import { parse, type InferOutput } from "valibot";

import type { Annotations } from "../../ir/annotations/annotations.js";
import type { return$, throw$ } from "../../ir/reference/completion-kind.js";
import { normal$ } from "../../ir/reference/completion-kind.js";
import type { AbruptCompletion } from "../../ir/reference/completion.js";
import type { Reference } from "../../ir/reference/reference.js";
import type { Scope } from "../../ir/scope/scope.js";
import type { Variable } from "../../ir/scope/variable.js";
import type { ReferenceId } from "../../ir/serialized/reference-id.js";
import type { ScopeId } from "../../ir/serialized/scope-id.js";
import {
  serializedReference$,
  type SerializedReference,
} from "../../ir/serialized/serialized-reference.js";
import type { VariableId } from "../../ir/serialized/variable-id.js";
import { spanFromOffset } from "../../util/span.js";
import { serializeHeadExpression } from "./serialize-expression-statement-head.js";
import { spanOf } from "./span-of.js";

export function serializeReference(
  r: Reference,
  scopeIds: Map<Scope, ScopeId>,
  variableIds: Map<Variable, VariableId>,
  referenceIds: Map<Reference, ReferenceId>,
  annotations: Annotations,
  raw: string,
): SerializedReference {
  const id = referenceIds.get(r) ?? null;
  if (id === null) {
    throw new Error("Reference id not found");
  }
  const from = scopeIds.get(r.from) ?? null;
  if (from === null) {
    throw new Error(`Scope id not found for reference ${r.identifier.name}`);
  }
  const ann = annotations.ofReference(r);
  return parse(serializedReference$, {
    id,
    identifier: { name: r.identifier.name, span: spanOf(r.identifier, raw) },
    from,
    resolved: r.resolved ? (variableIds.get(r.resolved) ?? null) : null,
    owners: ann.owners
      .map((v) => variableIds.get(v) ?? null)
      .filter((v): v is VariableId => v !== null),
    init: r.init,
    flags: {
      read: r.isRead(),
      write: r.isWrite(),
      call: ann.flags.call,
      receiver: ann.flags.receiver,
    },
    predicateContainer: ann.predicateContainer,
    completion:
      ann.completion.kind === normal$.literal
        ? { kind: normal$.literal }
        : serializeAbruptCompletion(ann.completion, raw),
    jsxElement: ann.jsxElement
      ? {
          startSpan: spanFromOffset(raw, ann.jsxElement.startOffset),
          endSpan: spanFromOffset(raw, ann.jsxElement.endOffset),
        }
      : null,
    expressionStatementContainer: ann.expressionStatementContainer
      ? {
          startSpan: spanFromOffset(
            raw,
            ann.expressionStatementContainer.startOffset,
          ),
          endSpan: spanFromOffset(
            raw,
            ann.expressionStatementContainer.endOffset,
          ),
          head: serializeHeadExpression(
            ann.expressionStatementContainer.head,
            raw,
          ),
        }
      : null,
  });
}

function serializeAbruptCompletion(
  c: AbruptCompletion,
  raw: string,
): Readonly<{
  kind: InferOutput<typeof return$> | InferOutput<typeof throw$>;
  startSpan: ReturnType<typeof spanFromOffset>;
  endSpan: ReturnType<typeof spanFromOffset>;
}> {
  return {
    kind: c.kind,
    startSpan: spanFromOffset(raw, c.startOffset),
    endSpan: spanFromOffset(raw, c.endOffset),
  };
}
