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
  PipelineRunDetails,
  PipelineRunOptions,
  PruningRunOptions,
  ScopeAnalyzer,
  SerializeContext,
  SourceMeta,
} from "./pipeline/types.js";

export type {
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

export { EslintCompatAnalyzer } from "./analyzer/eslint-compat/eslint-compat.js";

export { FlatSerializer } from "./serializer/flat/flat-serializer.js";

export { IrEmitter } from "./emitter/ir/ir.js";
export { JsonEmitter } from "./emitter/json/json.js";
export { buildVisualGraph } from "./visual-graph/builder.js";
export type {
  Direction,
  NodeKind,
  SubgraphKind,
  VisualBoundaryEdge,
  VisualEdge,
  VisualGraph,
  VisualGraphPruning,
  VisualNode,
  VisualSubgraph,
} from "./visual-graph/model.js";
export { pruneVisualGraph } from "./visual-graph/prune/prune-visual-graph.js";
export type {
  PruneOptions,
  PruneResult,
} from "./visual-graph/prune/prune-options.js";
export { MarkdownEmitter } from "./emitter/markdown/markdown.js";
export { MermaidEmitter } from "./emitter/mermaid/mermaid.js";
export type {
  MermaidEmitterOptions,
  MermaidRenderer,
} from "./emitter/mermaid/mermaid.js";
export { StatsEmitter } from "./emitter/stats/stats.js";
export { DefaultEmitterRegistry } from "./emitter/registry/registry.js";

export {
  createDefaultEmitterRegistry,
  createDefaultPipeline,
} from "./pipeline/default.js";
export type { DefaultRegistryOptions } from "./pipeline/default.js";

export { parseCliArgs } from "./cli/args/parse-cli-args.js";
export { usage as cliUsage } from "./cli/args/usage.js";
export type { CliArgs } from "./cli/args/cli-args.js";
export type { CliLanguage } from "./cli/args/cli-language.js";
export type { CliMermaidRenderer } from "./cli/args/cli-mermaid-renderer.js";
export type {
  CliParseFailure,
  CliParseResult,
  CliParseSuccess,
} from "./cli/args/cli-parse-result.js";
export { parseRootQuery } from "./cli/root-query/parse-root-query.js";
export { parseRootQueries } from "./cli/root-query/parse-root-queries.js";
export type {
  RootQueryParseFailure,
  RootQueryParseResult,
  RootQueryParseSuccess,
} from "./cli/root-query/parse-root-queries.js";
export type { ParsedRootQuery } from "./cli/root-query/parsed-root-query.js";
export { runCli } from "./cli/main/run-cli.js";
export { readSourceFile, readStdin } from "./cli/io.js";

export { walk } from "./analyzer/walk/walk.js";
export type { PathEntry, WalkVisitor } from "./analyzer/walk/walk.js";

export { ReferenceImpl, ScopeImpl, VariableImpl } from "./analyzer/scope.js";
export { ScopeManager } from "./analyzer/manager.js";

export { collectBindingIdentifiers } from "./analyzer/declare/collect-binding-identifiers.js";
export { declareVariable } from "./analyzer/declare/declare-variable.js";
export { hoistDeclarations } from "./analyzer/hoisting/hoist-declarations.js";

export { classifyIdentifier } from "./analyzer/classify/classify-identifier.js";
export type { ClassifyResult } from "./analyzer/classify/classify-result.js";

export { bindReference, resolveInScopeChain } from "./analyzer/resolve.js";

export { findReferenceOwners } from "./analyzer/owner/find-reference-owners.js";

export { findPredicateContainer } from "./analyzer/predicate.js";

export { DiagnosticCollector } from "./util/diagnostic.js";
export { spanFromOffset } from "./util/span.js";

export { makeReferenceId, makeScopeId, makeVariableId } from "./ir/id.js";
