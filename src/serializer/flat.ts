import { makeReferenceId, makeScopeId, makeVariableId } from "../ir/id.js";
import type {
  AstNode,
  Reference,
  Scope,
  SerializedDefinition,
  SerializedIR,
  SerializedReference,
  SerializedScope,
  SerializedVariable,
  Span,
  Variable,
} from "../ir/model.js";
import type { IRSerializer, SerializeContext } from "../pipeline/types.js";
import { spanFromOffset } from "../util/span.js";

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
      scopes: serializedScopes,
      variables: serializedVariables,
      references: serializedReferences,
      unusedVariableIds,
      diagnostics: [...ctx.diagnostics],
    };
  }
}

function collectScopesInOrder(root: Scope): Scope[] {
  const out: Scope[] = [];
  function visit(s: Scope) {
    out.push(s);
    for (const c of s.childScopes) {
      visit(c);
    }
  }
  visit(root);
  return out;
}

function pickVariableOffset(v: Variable): number {
  const head = v.identifiers[0];
  if (head !== undefined) {
    return head.start ?? 0;
  }
  const def = v.defs[0];
  if (def !== undefined) {
    return def.name.start ?? 0;
  }
  return 0;
}

function offsetOf(node: AstNode): number {
  return node.start ?? 0;
}

function spanOf(node: AstNode, raw: string): Span {
  return spanFromOffset(raw, node.start ?? 0);
}

function hasDeclaringDef(v: Variable): boolean {
  return v.defs.some((d) => d.type !== "ImplicitGlobalVariable");
}

function serializeScope(
  scope: Scope,
  scopeIds: Map<Scope, string>,
  variableIds: Map<Variable, string>,
  referenceIds: Map<Reference, string>,
  raw: string,
): SerializedScope {
  const id = scopeIds.get(scope);
  if (id === undefined) {
    throw new Error("Scope id not found");
  }
  return {
    id,
    type: scope.type,
    isStrict: scope.isStrict,
    upper: scope.upper ? (scopeIds.get(scope.upper) ?? null) : null,
    childScopes: scope.childScopes
      .map((c) => scopeIds.get(c))
      .filter((x): x is string => x !== undefined),
    variableScope: scopeIds.get(scope.variableScope) ?? id,
    block: { type: scope.block.type, span: spanOf(scope.block, raw) },
    variables: scope.variables
      .map((v) => variableIds.get(v))
      .filter((x): x is string => x !== undefined),
    references: scope.references
      .map((r) => referenceIds.get(r))
      .filter((x): x is string => x !== undefined),
    through: scope.through
      .map((r) => referenceIds.get(r))
      .filter((x): x is string => x !== undefined),
    functionExpressionScope: scope.functionExpressionScope,
  };
}

function serializeVariable(
  v: Variable,
  scopeIds: Map<Scope, string>,
  variableIds: Map<Variable, string>,
  referenceIds: Map<Reference, string>,
  raw: string,
): SerializedVariable {
  const id = variableIds.get(v);
  if (id === undefined) {
    throw new Error("Variable id not found");
  }
  return {
    id,
    name: v.name,
    scope: scopeIds.get(v.scope) ?? "",
    identifiers: v.identifiers.map((i) => spanOf(i, raw)),
    references: v.references
      .map((r) => referenceIds.get(r))
      .filter((x): x is string => x !== undefined),
    defs: v.defs.map<SerializedDefinition>((d) => ({
      type: d.type,
      name: { name: d.name.name, span: spanOf(d.name, raw) },
      node: { type: d.node.type, span: spanOf(d.node, raw) },
      parent:
        d.parent === null
          ? null
          : { type: d.parent.type, span: spanOf(d.parent, raw) },
    })),
  };
}

function serializeReference(
  r: Reference,
  scopeIds: Map<Scope, string>,
  variableIds: Map<Variable, string>,
  referenceIds: Map<Reference, string>,
  raw: string,
): SerializedReference {
  const id = referenceIds.get(r);
  if (id === undefined) {
    throw new Error("Reference id not found");
  }
  return {
    id,
    identifier: { name: r.identifier.name, span: spanOf(r.identifier, raw) },
    from: scopeIds.get(r.from) ?? "",
    resolved: r.resolved ? (variableIds.get(r.resolved) ?? null) : null,
    owners: (r.unsnarlOwners ?? [])
      .map((o) => variableIds.get(o))
      .filter((x): x is string => x !== undefined),
    writeExpr: r.writeExpr ? spanOf(r.writeExpr, raw) : null,
    init: r.init,
    flags: {
      read: r.isRead(),
      write: r.isWrite(),
      call: r.isCall?.() ?? false,
    },
  };
}
