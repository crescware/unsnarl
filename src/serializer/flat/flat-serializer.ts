import { makeReferenceId, makeScopeId, makeVariableId } from "../../ir/id.js";
import type { Reference } from "../../ir/reference/reference.js";
import type { Scope } from "../../ir/scope/scope.js";
import type { Variable } from "../../ir/scope/variable.js";
import type { SerializedIR } from "../../ir/serialized/serialized-ir.js";
import type { SerializedReference } from "../../ir/serialized/serialized-reference.js";
import type { SerializedScope } from "../../ir/serialized/serialized-scope.js";
import type { SerializedVariable } from "../../ir/serialized/serialized-variable.js";
import type { IRSerializer } from "../../pipeline/serialize/ir-serializer.js";
import type { SerializeContext } from "../../pipeline/serialize/serialize-context.js";
import { SERIALIZED_IR_VERSION } from "../serialized-ir-version.js";
import { collectScopesInOrder } from "./collect-scopes-in-order.js";
import { hasDeclaringDef } from "./has-declaring-def.js";
import { offsetOf } from "./offset-of.js";
import { pickVariableOffset } from "./pick-variable-offset.js";
import { serializeReference } from "./serialize-reference.js";
import { serializeScope } from "./serialize-scope.js";
import { serializeVariable } from "./serialize-variable.js";

export class FlatSerializer implements IRSerializer {
  readonly id = "flat";

  serialize(ctx: SerializeContext): SerializedIR {
    const scopes = collectScopesInOrder(ctx.rootScope);
    const scopeIds = new Map<Scope, string>();
    scopes.forEach((s, i) => scopeIds.set(s, makeScopeId(i)));

    const variableIds = new Map<Variable, string>();
    const orderedVariables: /* mutable */ Variable[] = [];
    for (const s of scopes) {
      for (const v of s.variables) {
        const sid = scopeIds.get(s);
        if (sid === undefined) {
          continue;
        }
        const offset = pickVariableOffset(v);
        const vid = makeVariableId(sid, v.name, offset);
        variableIds.set(v, vid);
        orderedVariables.push(v);
      }
    }

    const referenceIds = new Map<Reference, string>();
    const allReferences: /* mutable */ Reference[] = [];
    for (const s of scopes) {
      for (const r of s.references) {
        if (referenceIds.has(r)) {
          continue;
        }
        allReferences.push(r);
      }
    }
    allReferences.sort(
      (a, b) => offsetOf(a.identifier) - offsetOf(b.identifier),
    );
    allReferences.forEach((r, i) => referenceIds.set(r, makeReferenceId(i)));

    const serializedScopes: readonly SerializedScope[] = scopes.map((s) =>
      serializeScope(s, scopeIds, variableIds, referenceIds, ctx.raw),
    );

    const serializedVariables: readonly SerializedVariable[] =
      orderedVariables.map((v) =>
        serializeVariable(v, scopeIds, variableIds, referenceIds, ctx.raw),
      );

    const serializedReferences: readonly SerializedReference[] =
      allReferences.map((r) =>
        serializeReference(r, scopeIds, variableIds, referenceIds, ctx.raw),
      );

    const unusedVariableIds: /* mutable */ string[] = [];
    for (const v of orderedVariables) {
      if (v.references.length === 0 && hasDeclaringDef(v)) {
        const id = variableIds.get(v);
        if (id !== undefined) {
          unusedVariableIds.push(id);
        }
      }
    }

    return {
      version: SERIALIZED_IR_VERSION,
      source: { path: ctx.source.path, language: ctx.source.language },
      raw: ctx.raw,
      scopes: serializedScopes,
      variables: serializedVariables,
      references: serializedReferences,
      unusedVariableIds,
      diagnostics: [...ctx.diagnostics],
    };
  }
}
