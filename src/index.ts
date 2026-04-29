export { createPipeline } from "./pipeline/pipeline.js";

export type {
  EmitOptions,
  Emitter,
  EmitterRegistry,
  IRSerializer,
  ParseOptions,
  ParsedSource,
  Parser,
  Pipeline,
  PipelineConfig,
  PipelineRunOptions,
  ScopeAnalyzer,
  SourceMeta,
} from "./pipeline/types.js";

export type {
  AstExpression,
  AstIdentifier,
  AstNode,
  Definition,
  DefinitionType,
  Diagnostic,
  DiagnosticKind,
  Language,
  Reference,
  ReferenceFlagBits,
  ReferenceId,
  Scope,
  ScopeId,
  ScopeType,
  SerializedDefinition,
  SerializedIR,
  SerializedReference,
  SerializedScope,
  SerializedVariable,
  Span,
  Variable,
  VariableId,
} from "./ir/model.js";

export { ReferenceFlags } from "./ir/model.js";

export { OxcParser, ParseError } from "./parser/oxc.js";
export type { ParseErrorDetail } from "./parser/oxc.js";

export { EslintCompatAnalyzer } from "./analyzer/eslint-compat.js";
export type { AnalysisResult } from "./analyzer/eslint-compat.js";

export { walk } from "./analyzer/walk.js";
export type { PathEntry, WalkVisitor } from "./analyzer/walk.js";

export { ReferenceImpl, ScopeImpl, VariableImpl } from "./analyzer/scope.js";
export { ScopeManager } from "./analyzer/manager.js";

export {
  collectBindingIdentifiers,
  declareVariable,
} from "./analyzer/declare.js";
export { hoistDeclarations } from "./analyzer/hoisting.js";

export { classifyIdentifier } from "./analyzer/classify.js";
export type { ClassifyResult } from "./analyzer/classify.js";

export { bindReference, resolveInScopeChain } from "./analyzer/resolve.js";

export { DiagnosticCollector } from "./util/diagnostic.js";
export { spanFromOffset } from "./util/span.js";

export { makeReferenceId, makeScopeId, makeVariableId } from "./ir/id.js";
