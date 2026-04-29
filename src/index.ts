export { createPipeline } from "./pipeline/pipeline.js";

export type {
  AnalyzedSource,
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
  SerializeContext,
  SourceMeta,
} from "./pipeline/types.js";

export type {
  AstExpression,
  AstIdentifier,
  AstNode,
  BlockContext,
  Definition,
  DefinitionType,
  Diagnostic,
  DiagnosticKind,
  ImportKind,
  Language,
  PredicateContainer,
  PredicateContainerType,
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
  VariableDeclarationKind,
  VariableId,
} from "./ir/model.js";

export { ReferenceFlags } from "./ir/model.js";

export { OxcParser, ParseError } from "./parser/oxc.js";
export type { ParseErrorDetail } from "./parser/oxc.js";

export { EslintCompatAnalyzer } from "./analyzer/eslint-compat.js";

export { FlatSerializer } from "./serializer/flat.js";

export { JsonEmitter } from "./emitter/json.js";
export { MarkdownEmitter } from "./emitter/markdown.js";
export { MermaidEmitter } from "./emitter/mermaid.js";
export { DefaultEmitterRegistry } from "./emitter/registry.js";

export {
  createDefaultEmitterRegistry,
  createDefaultPipeline,
} from "./pipeline/default.js";

export { parseCliArgs, usage as cliUsage } from "./cli/args.js";
export type {
  CliArgs,
  CliLanguage,
  CliParseFailure,
  CliParseResult,
  CliParseSuccess,
} from "./cli/args.js";
export { runCli } from "./cli/main.js";
export { readSourceFile, readStdin } from "./cli/io.js";

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

export { findReferenceOwners } from "./analyzer/owner.js";

export { findPredicateContainer } from "./analyzer/predicate.js";

export { DiagnosticCollector } from "./util/diagnostic.js";
export { spanFromOffset } from "./util/span.js";

export { makeReferenceId, makeScopeId, makeVariableId } from "./ir/id.js";
