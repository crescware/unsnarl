import type { Reference } from "../../ir/reference/reference.js";
import type { Scope } from "../../ir/scope/scope.js";
import type { Variable } from "../../ir/scope/variable.js";
import {
  asReferenceId,
  type ReferenceId,
} from "../../ir/serialized/reference-id.js";
import { asScopeId, type ScopeId } from "../../ir/serialized/scope-id.js";
import type { SerializedIR } from "../../ir/serialized/serialized-ir.js";
import type { SerializedReference } from "../../ir/serialized/serialized-reference.js";
import type { SerializedScope } from "../../ir/serialized/serialized-scope.js";
import type { SerializedVariable } from "../../ir/serialized/serialized-variable.js";
import {
  asVariableId,
  type VariableId,
} from "../../ir/serialized/variable-id.js";
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
    const scopeIds = new Map<Scope, ScopeId>();
    scopes.forEach((v, i) => scopeIds.set(v, asScopeId(`scope#${i}`)));

    const variableIds = new Map<Variable, VariableId>();
    const orderedVariables: /* mutable */ Variable[] = [];
    for (const s of scopes) {
      for (const v of s.variables) {
        const sid = scopeIds.get(s) ?? null;
        if (sid === null) {
          continue;
        }
        // Implicit bindings such as `arguments` (FunctionDeclarationInstantiation,
        // ES spec 9.2.13) carry no source-level identifier or definition;
        // they exist only to satisfy resolution for in-source references and
        // have nothing observable to serialize. Exclude them at the boundary
        // so SerializedVariable can guarantee defs.length >= 1.
        if (v.defs.length === 0) {
          continue;
        }
        const offset = pickVariableOffset(v);
        const vid = asVariableId(`${sid}:${v.name}@${offset}`);
        variableIds.set(v, vid);
        orderedVariables.push(v);
      }
    }

    const referenceIds = new Map<Reference, ReferenceId>();
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
    allReferences.forEach((v, i) =>
      referenceIds.set(v, asReferenceId(`ref#${i}`)),
    );

    const serializedScopes: readonly SerializedScope[] = scopes.map((v) =>
      serializeScope(
        v,
        scopeIds,
        variableIds,
        referenceIds,
        ctx.annotations,
        ctx.raw,
      ),
    );

    const serializedVariables: readonly SerializedVariable[] =
      orderedVariables.map((v) =>
        serializeVariable(v, scopeIds, variableIds, referenceIds, ctx.raw),
      );

    const serializedReferences: readonly SerializedReference[] =
      allReferences.map((v) =>
        serializeReference(
          v,
          scopeIds,
          variableIds,
          referenceIds,
          ctx.annotations,
          ctx.raw,
        ),
      );

    const unusedVariableIds: /* mutable */ VariableId[] = [];
    for (const v of orderedVariables) {
      if (ctx.annotations.ofVariable(v).isUnused && hasDeclaringDef(v)) {
        const id = variableIds.get(v) ?? null;
        if (id !== null) {
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
