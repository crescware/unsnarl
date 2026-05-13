import { DEFINITION_TYPE } from "../../analyzer/definition-type.js";
import type { Span } from "../../ir/primitive/span.js";
import type { ReferenceId } from "../../ir/serialized/reference-id.js";
import type { SerializedIR } from "../../ir/serialized/serialized-ir.js";
import type { SerializedScope } from "../../ir/serialized/serialized-scope.js";
import type { SerializedVariable } from "../../ir/serialized/serialized-variable.js";
import type { VariableId } from "../../ir/serialized/variable-id.js";
import { AST_TYPE, type AstType } from "../../parser/ast-type.js";
import type { UnsnarlPlugin } from "../../pipeline/plugin/unsnarl-plugin.js";
import { IMPORT_KIND } from "../../serializer/import-kind.js";

const REACT_MODULE = "react";
const HOOK_USE_CALLBACK = "useCallback";
const HOOK_USE_MEMO = "useMemo";

type HookKind = typeof HOOK_USE_CALLBACK | typeof HOOK_USE_MEMO;

const HOOK_NAMES = new Set<string>([HOOK_USE_CALLBACK, HOOK_USE_MEMO]);

function asHookKind(name: string): HookKind | null {
  return HOOK_NAMES.has(name) ? (name as HookKind) : null;
}

type InitReplacement = Readonly<{ type: AstType; span: Span }>;

const plugin: UnsnarlPlugin = {
  meta: { name: "unsnarl-plugin-react" },
  transform(ir: SerializedIR): SerializedIR {
    const hookImports = collectHookImports(ir);
    if (hookImports.size === 0) {
      return ir;
    }

    const childScopesByUpper = groupChildScopes(ir);
    const { initReplacements, wrappedVarIds } = collectInitTargets(
      ir,
      hookImports,
      childScopesByUpper,
    );
    if (wrappedVarIds.size === 0) {
      return ir;
    }

    const refsToRemove = collectRefsToRemove(ir, wrappedVarIds);
    const refsRetainedByVar = countRetainedRefsByResolved(ir, refsToRemove);
    const varsToRemove = collectVarsToRemove(hookImports, refsRetainedByVar);

    return rebuildIr(ir, {
      refsToRemove,
      varsToRemove,
      initReplacements,
    });
  },
};

export default plugin;

function collectHookImports(ir: SerializedIR): Map<VariableId, HookKind> {
  const out = new Map<VariableId, HookKind>();
  for (const v of ir.variables) {
    const def = v.defs[0];
    if (!def || def.type !== DEFINITION_TYPE.ImportBinding) {
      continue;
    }
    if (def.importKind !== IMPORT_KIND.Named) {
      continue;
    }
    if (def.importSource !== REACT_MODULE) {
      continue;
    }
    const kind = asHookKind(def.importedName);
    if (kind === null) {
      continue;
    }
    out.set(v.id, kind);
  }
  return out;
}

function groupChildScopes(
  ir: SerializedIR,
): Map<string, readonly SerializedScope[]> {
  const out = new Map<string, /* mutable */ SerializedScope[]>();
  for (const s of ir.scopes) {
    if (s.upper === null) {
      continue;
    }
    const arr = out.get(s.upper) ?? [];
    arr.push(s);
    out.set(s.upper, arr);
  }
  return out;
}

function collectInitTargets(
  ir: SerializedIR,
  hookImports: ReadonlyMap<VariableId, HookKind>,
  childScopesByUpper: ReadonlyMap<string, readonly SerializedScope[]>,
): Readonly<{
  initReplacements: ReadonlyMap<VariableId, InitReplacement>;
  wrappedVarIds: ReadonlySet<VariableId>;
}> {
  const initReplacements = new Map<VariableId, InitReplacement>();
  const wrappedVarIds = new Set<VariableId>();

  for (const v of ir.variables) {
    const def = v.defs[0];
    if (!def || def.type !== DEFINITION_TYPE.Variable) {
      continue;
    }
    if (def.init === null || def.init.type !== AST_TYPE.CallExpression) {
      continue;
    }
    const kind = findHookCalleeKind(ir, hookImports, v.id, def.init.span);
    if (kind === null) {
      continue;
    }
    const inner = findInnerFunctionScope(
      childScopesByUpper.get(v.scope) ?? [],
      def.init.span.offset,
    );
    if (inner === null) {
      continue;
    }
    wrappedVarIds.add(v.id);
    // useCallback: peel the wrapper so the variable's init points at the
    // inner function, matching how a plain `const x = () => ...` reads.
    // useMemo: keep the init as the original CallExpression so the IR
    // reads as an IIFE-style invocation of the inner function.
    if (kind === HOOK_USE_CALLBACK) {
      initReplacements.set(v.id, {
        type: inner.block.type,
        span: inner.block.span,
      });
    }
  }
  return { initReplacements, wrappedVarIds };
}

function findHookCalleeKind(
  ir: SerializedIR,
  hookImports: ReadonlyMap<VariableId, HookKind>,
  ownerVarId: VariableId,
  initSpan: Span,
): HookKind | null {
  for (const r of ir.references) {
    if (r.init) {
      continue;
    }
    if (!r.flags.call) {
      continue;
    }
    if (r.resolved === null) {
      continue;
    }
    const kind = hookImports.get(r.resolved);
    if (kind === undefined) {
      continue;
    }
    if (!r.owners.includes(ownerVarId)) {
      continue;
    }
    if (r.identifier.span.offset !== initSpan.offset) {
      continue;
    }
    return kind;
  }
  return null;
}

function findInnerFunctionScope(
  siblings: readonly SerializedScope[],
  callOffset: number,
): SerializedScope | null {
  let best: SerializedScope | null = null;
  for (const s of siblings) {
    if (
      s.block.type !== AST_TYPE.ArrowFunctionExpression &&
      s.block.type !== AST_TYPE.FunctionExpression
    ) {
      continue;
    }
    if (s.block.span.offset <= callOffset) {
      continue;
    }
    if (best === null || s.block.span.offset < best.block.span.offset) {
      best = s;
    }
  }
  return best;
}

function collectRefsToRemove(
  ir: SerializedIR,
  wrappedVarIds: ReadonlySet<VariableId>,
): Set<ReferenceId> {
  const out = new Set<ReferenceId>();
  for (const r of ir.references) {
    if (r.init) {
      continue;
    }
    for (const o of r.owners) {
      if (wrappedVarIds.has(o)) {
        out.add(r.id);
        break;
      }
    }
  }
  return out;
}

function countRetainedRefsByResolved(
  ir: SerializedIR,
  refsToRemove: ReadonlySet<ReferenceId>,
): Map<VariableId, number> {
  const out = new Map<VariableId, number>();
  for (const r of ir.references) {
    if (refsToRemove.has(r.id)) {
      continue;
    }
    if (r.resolved === null) {
      continue;
    }
    out.set(r.resolved, (out.get(r.resolved) ?? 0) + 1);
  }
  return out;
}

function collectVarsToRemove(
  hookImports: ReadonlyMap<VariableId, HookKind>,
  refsRetainedByVar: ReadonlyMap<VariableId, number>,
): Set<VariableId> {
  const out = new Set<VariableId>();
  for (const id of hookImports.keys()) {
    if (!refsRetainedByVar.has(id)) {
      out.add(id);
    }
  }
  return out;
}

function rebuildIr(
  ir: SerializedIR,
  changes: Readonly<{
    refsToRemove: ReadonlySet<ReferenceId>;
    varsToRemove: ReadonlySet<VariableId>;
    initReplacements: ReadonlyMap<VariableId, InitReplacement>;
  }>,
): SerializedIR {
  const newReferences = ir.references.filter(
    (v) => !changes.refsToRemove.has(v.id),
  );
  const newVariables: readonly SerializedVariable[] = ir.variables
    .filter((v) => !changes.varsToRemove.has(v.id))
    .map((v) => rebuildVariable(v, changes));
  const newScopes: readonly SerializedScope[] = ir.scopes.map((scope) => ({
    ...scope,
    variables: scope.variables.filter((vid) => !changes.varsToRemove.has(vid)),
    references: scope.references.filter(
      (rid) => !changes.refsToRemove.has(rid),
    ),
    through: scope.through.filter((rid) => !changes.refsToRemove.has(rid)),
  }));
  const newUnusedVariableIds = ir.unusedVariableIds.filter(
    (id) => !changes.varsToRemove.has(id),
  );
  return {
    ...ir,
    scopes: newScopes,
    variables: newVariables,
    references: newReferences,
    unusedVariableIds: newUnusedVariableIds,
  };
}

function rebuildVariable(
  v: SerializedVariable,
  changes: Readonly<{
    refsToRemove: ReadonlySet<ReferenceId>;
    initReplacements: ReadonlyMap<VariableId, InitReplacement>;
  }>,
): SerializedVariable {
  const updatedRefs = v.references.filter(
    (rid) => !changes.refsToRemove.has(rid),
  );
  const replacement = changes.initReplacements.get(v.id);
  const def0 = v.defs[0];
  if (
    replacement !== undefined &&
    def0 !== undefined &&
    def0.type === DEFINITION_TYPE.Variable
  ) {
    return {
      ...v,
      references: updatedRefs,
      defs: [{ ...def0, init: replacement }, ...v.defs.slice(1)],
    };
  }
  return { ...v, references: updatedRefs };
}
