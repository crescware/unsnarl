import { parse } from "valibot";

import { AnnotationsImpl } from "../../analyzer/annotations-impl.js";
import { blockContextOf } from "../../analyzer/block-context-of.js";
import { caseExitsFunction } from "../../analyzer/case-exits-function.js";
import { caseFallsThrough } from "../../analyzer/case-falls-through.js";
import { findExpressionStatementContainer } from "../../analyzer/expression-statement-container.js";
import { findCompletion } from "../../analyzer/find-completion.js";
import { formatCaseTest } from "../../analyzer/format-case-test.js";
import { isAstNode } from "../../analyzer/is-ast-node.js";
import { isUnused } from "../../analyzer/is-unused.js";
import { findJsxElementSpan } from "../../analyzer/jsx-element-span.js";
import { findReferenceOwners } from "../../analyzer/owner/find-reference-owners.js";
import { findPredicateContainer } from "../../analyzer/predicate.js";
import { referenceCallReceiverFlags } from "../../analyzer/reference-call-receiver.js";
import { SCOPE_TYPE } from "../../analyzer/scope-type.js";
import type { AnalysisVisitor } from "../../boundary/eslint-scope/visitor.js";
import type { Annotations } from "../../ir/annotations/annotations.js";
import type { NestingDepths } from "../../ir/annotations/scope-annotation.js";
import type { Diagnostic } from "../../ir/diagnostic/diagnostic.js";
import type { AstNode } from "../../ir/primitive/ast-node.js";
import { caseClause$ } from "../../ir/scope/block-context-kind.js";
import {
  blockContext$,
  type BlockContext,
} from "../../ir/scope/block-context.js";
import type { Scope } from "../../ir/scope/scope.js";
import { asAstType, AST_TYPE } from "../../parser/ast-type.js";
import { NESTING_KIND } from "../../serializer/nesting-kind.js";

type AnalysisCapture = Readonly<{
  annotations: Annotations;
  diagnostics: readonly Diagnostic[];
}>;

type CapturingVisitor = Readonly<{
  visitor: AnalysisVisitor;
  capture(globalScope: Scope): AnalysisCapture;
}>;

const ZERO_DEPTHS: NestingDepths = {
  [NESTING_KIND.Function]: 0,
  [NESTING_KIND.If]: 0,
  [NESTING_KIND.For]: 0,
  [NESTING_KIND.While]: 0,
  [NESTING_KIND.Switch]: 0,
  [NESTING_KIND.TryCatchFinally]: 0,
  [NESTING_KIND.Block]: 0,
};

export function buildAnalysisVisitor(
  raw: string,
  nestingDepthsByOffset: ReadonlyMap<number, NestingDepths>,
): CapturingVisitor {
  const annotations = new AnnotationsImpl();
  const diagnostics: /* mutable */ Diagnostic[] = [];

  const visitor: AnalysisVisitor = {
    onReference(input) {
      annotations.setReference(input.ref, {
        owners: findReferenceOwners(input.path, input.scope),
        flags: referenceCallReceiverFlags(
          input.parent as AstNode | null,
          input.key,
        ),
        predicateContainer: findPredicateContainer(
          input.parent,
          input.key,
          input.path,
        ),
        completion: findCompletion(input.path),
        jsxElement: findJsxElementSpan(input.path),
        expressionStatementContainer: findExpressionStatementContainer(
          input.path,
        ),
      });
    },
    onScope(input) {
      const block = input.scope.block;
      const isSwitchCase =
        input.scope.type === SCOPE_TYPE.Block &&
        block.type === AST_TYPE.SwitchCase;
      let blockContext: BlockContext | null;
      let fallsThrough = false;
      let exitsFunction = false;
      if (isSwitchCase) {
        const test = (block as { test?: unknown }).test;
        const caseTest = isAstNode(test) ? formatCaseTest(test, raw) : null;
        blockContext =
          input.parent && input.key !== null
            ? parse(blockContext$, {
                kind: caseClause$.literal,
                parentType: asAstType(input.parent.type),
                key: input.key,
                parentSpanOffset: input.parent.start ?? 0,
                caseTest,
              })
            : null;
        const consequent = (block as { consequent?: unknown }).consequent;
        if (Array.isArray(consequent)) {
          fallsThrough = caseFallsThrough(consequent);
          exitsFunction = caseExitsFunction(consequent);
        } else {
          fallsThrough = true;
        }
      } else if (input.scope.type === SCOPE_TYPE.Function) {
        blockContext = null;
      } else {
        blockContext = blockContextOf(
          input.parent as AstNode | null,
          input.key,
          input.path,
        );
      }
      const blockStart = input.scope.block.start;
      const nestingDepths =
        blockStart !== undefined
          ? (nestingDepthsByOffset.get(blockStart) ?? ZERO_DEPTHS)
          : ZERO_DEPTHS;
      annotations.setScope(input.scope, {
        blockContext,
        fallsThrough,
        exitsFunction,
        nestingDepths,
      });
    },
    onDiagnostic(diag) {
      diagnostics.push(diag);
    },
  };

  return {
    visitor,
    capture: (globalScope) => {
      populateVariableAnnotations(globalScope, annotations);
      return { annotations, diagnostics };
    },
  };
}

function populateVariableAnnotations(
  globalScope: Scope,
  annotations: AnnotationsImpl,
): void {
  const stack: /* mutable */ Scope[] = [globalScope];
  while (stack.length > 0) {
    const scope = stack.pop() as Scope;
    for (const variable of scope.variables) {
      annotations.setVariable(variable, { isUnused: isUnused(variable) });
    }
    for (const child of scope.childScopes) {
      stack.push(child);
    }
  }
}
