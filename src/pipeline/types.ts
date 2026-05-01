import type { ParsedRootQuery } from "../cli/root-query/parsed-root-query.js";
import type { Diagnostic, Language, Scope, SerializedIR } from "../ir/model.js";
import type { VisualGraph } from "../visual-graph/model.js";

export type ParseOptions = {
  language: Language;
  sourcePath: string;
};

export type ParsedSource = {
  ast: unknown;
  language: Language;
  sourcePath: string;
  raw: string;
};

export type Parser = {
  readonly id: string;
  parse(code: string, opts: ParseOptions): ParsedSource;
};

export type AnalyzedSource = {
  readonly rootScope: Scope;
  readonly diagnostics: readonly Diagnostic[];
  readonly raw: string;
};

export type ScopeAnalyzer = {
  readonly id: string;
  analyze(parsed: ParsedSource): AnalyzedSource;
};

export type SourceMeta = {
  path: string;
  language: Language;
};

export type SerializeContext = {
  readonly rootScope: Scope;
  readonly source: SourceMeta;
  readonly diagnostics: readonly Diagnostic[];
  readonly raw: string;
};

export type IRSerializer = {
  readonly id: string;
  serialize(ctx: SerializeContext): SerializedIR;
};

export type EmitOptions = {
  pretty?: boolean;
  prunedGraph?: VisualGraph;
};

export type Emitter = {
  readonly format: string;
  readonly contentType: string;
  // File extension without leading dot, used by `--out-dir` to derive
  // output filenames. Multiple emitters may share an extension (e.g. ir
  // and json both write JSON).
  readonly extension: string;
  emit(ir: SerializedIR, opts: EmitOptions): string;
};

export type EmitterRegistry = {
  register(emitter: Emitter): void;
  get(format: string): Emitter | undefined;
  list(): readonly string[];
};

export type PruningRunOptions = {
  readonly roots: readonly ParsedRootQuery[];
  readonly descendants: number;
  readonly ancestors: number;
};

export type PipelineRunOptions = ParseOptions & {
  format: string;
  emit?: EmitOptions;
  pruning?: PruningRunOptions;
};

export type PipelineRunDetails = {
  readonly text: string;
  readonly pruning: ReadonlyArray<{
    readonly query: string;
    readonly matched: number;
  }> | null;
};

export type Pipeline = {
  run(code: string, opts: PipelineRunOptions): string;
  runDetailed(code: string, opts: PipelineRunOptions): PipelineRunDetails;
};

export type PipelineConfig = {
  parser: Parser;
  analyzer: ScopeAnalyzer;
  serializer: IRSerializer;
  emitters: EmitterRegistry;
};
