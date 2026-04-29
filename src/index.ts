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
