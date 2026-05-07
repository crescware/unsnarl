import { AnnotationsImpl } from "../../analyzer/annotations-impl.js";
import { blockContextOf } from "../../analyzer/block-context-of.js";
import { caseExitsFunction } from "../../analyzer/case-exits-function.js";
import { caseFallsThrough } from "../../analyzer/case-falls-through.js";
import { findExpressionStatementContainer } from "../../analyzer/expression-statement-container.js";
import { formatCaseTest } from "../../analyzer/format-case-test.js";
import { isAstNode } from "../../analyzer/is-ast-node.js";
import { findJsxElementSpan } from "../../analyzer/jsx-element-span.js";
import { findReferenceOwners } from "../../analyzer/owner/find-reference-owners.js";
import { findPredicateContainer } from "../../analyzer/predicate.js";
import { findReturnContainer } from "../../analyzer/return-container.js";
import { SCOPE_TYPE } from "../../analyzer/scope-type.js";
import type { AnalysisVisitor } from "../../boundary/eslint-scope/visitor.js";
import type { Annotations } from "../../ir/annotations/annotations.js";
import type { CategoryDepths } from "../../ir/annotations/scope-annotation.js";
import type { Diagnostic } from "../../ir/diagnostic/diagnostic.js";
import type { AstNode } from "../../ir/primitive/ast-node.js";
import type { BlockContext } from "../../ir/scope/block-context.js";
import { AST_TYPE } from "../../parser/ast-type.js";
import { CATEGORY } from "../../serializer/category.js";

type AnalysisCapture = Readonly<{
  annotations: Annotations;
  diagnostics: readonly Diagnostic[];
}>;

type CapturingVisitor = Readonly<{
  visitor: AnalysisVisitor;
  capture(): AnalysisCapture;
}>;

const ZERO_DEPTHS: CategoryDepths = {
  [CATEGORY.Function]: 0,
  [CATEGORY.If]: 0,
  [CATEGORY.For]: 0,
  [CATEGORY.While]: 0,
  [CATEGORY.Switch]: 0,
  [CATEGORY.TryCatchFinally]: 0,
  [CATEGORY.Block]: 0,
};

export function buildAnalysisVisitor(
  raw: string,
  categoryDepthsByOffset: ReadonlyMap<number, CategoryDepths>,
): CapturingVisitor {
  const annotations = new AnnotationsImpl();
  const diagnostics: /* mutable */ Diagnostic[] = [];

  const visitor: AnalysisVisitor = {
    onReference(input) {
      annotations.setReference(input.ref, {
        owners: findReferenceOwners(input.path, input.scope),
        predicateContainer: findPredicateContainer(
          input.parent,
          input.key,
          input.path,
        ),
        returnContainer: findReturnContainer(input.path),
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
            ? {
                kind: "case-clause",
                parentType: input.parent.type,
                key: input.key,
                parentSpanOffset: input.parent.start ?? 0,
                caseTest,
              }
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
      const categoryDepths =
        blockStart !== undefined
          ? (categoryDepthsByOffset.get(blockStart) ?? ZERO_DEPTHS)
          : ZERO_DEPTHS;
      annotations.setScope(input.scope, {
        blockContext,
        fallsThrough,
        exitsFunction,
        categoryDepths,
      });
    },
    onDiagnostic(diag) {
      diagnostics.push(diag);
    },
  };

  return {
    visitor,
    capture: () => ({ annotations, diagnostics }),
  };
}
