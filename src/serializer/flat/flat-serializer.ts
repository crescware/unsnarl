import { makeReferenceId, makeScopeId, makeVariableId } from "../../ir/id.js";
import type {
  Reference,
  Scope,
  SerializedIR,
  SerializedReference,
  SerializedScope,
  SerializedVariable,
  Variable,
} from "../../ir/model.js";
import type { IRSerializer, SerializeContext } from "../../pipeline/types.js";
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
    const orderedVariables: Variable[] = [];
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
    const allReferences: Reference[] = [];
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

    const serializedScopes: SerializedScope[] = scopes.map((s) =>
      serializeScope(s, scopeIds, variableIds, referenceIds, ctx.raw),
    );

    const serializedVariables: SerializedVariable[] = orderedVariables.map(
      (v) => serializeVariable(v, scopeIds, variableIds, referenceIds, ctx.raw),
    );

    const serializedReferences: SerializedReference[] = allReferences.map((r) =>
      serializeReference(r, scopeIds, variableIds, referenceIds, ctx.raw),
    );

    const unusedVariableIds: string[] = [];
    for (const v of orderedVariables) {
      if (v.references.length === 0 && hasDeclaringDef(v)) {
        const id = variableIds.get(v);
        if (id !== undefined) {
          unusedVariableIds.push(id);
        }
      }
    }

    return {
      version: 1,
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
