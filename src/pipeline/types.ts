import type { ParsedRootQuery } from "../cli/root-query/parsed-root-query.js";
import type { Diagnostic, Language, Scope, SerializedIR } from "../ir/model.js";
import type { VisualGraph } from "../visual-graph/model.js";

export type ParseOptions = Readonly<{
  language: Language;
  sourcePath: string;
}>;

export type ParsedSource = Readonly<{
  ast: unknown;
  language: Language;
  sourcePath: string;
  raw: string;
}>;

export type Parser = Readonly<{
  id: string;
  parse(code: string, opts: ParseOptions): ParsedSource;
}>;

export type AnalyzedSource = Readonly<{
  rootScope: Scope;
  diagnostics: readonly Diagnostic[];
  raw: string;
}>;

export type ScopeAnalyzer = Readonly<{
  id: string;
  analyze(parsed: ParsedSource): AnalyzedSource;
}>;

type SourceMeta = Readonly<{
  path: string;
  language: Language;
}>;

export type SerializeContext = Readonly<{
  rootScope: Scope;
  source: SourceMeta;
  diagnostics: readonly Diagnostic[];
  raw: string;
}>;

export type IRSerializer = Readonly<{
  id: string;
  serialize(ctx: SerializeContext): SerializedIR;
}>;

export type EmitOptions = Readonly<{
  prettyJson: boolean;
  prunedGraph: VisualGraph | null;
}>;

export type Emitter = Readonly<{
  format: string;
  contentType: string;
  // File extension without leading dot, used by `--out-dir` to derive
  // output filenames. Multiple emitters may share an extension (e.g. ir
  // and json both write JSON).
  extension: string;
  emit(ir: SerializedIR, opts: EmitOptions): string;
}>;

export type EmitterRegistry = Readonly<{
  register(emitter: Emitter): void;
  get(format: string): Emitter | undefined;
  list(): readonly string[];
}>;

export type PruningRunOptions = Readonly<{
  roots: readonly ParsedRootQuery[];
  descendants: number;
  ancestors: number;
}>;

export type PipelineRunOptions = ParseOptions &
  Readonly<{
    format: string;
    emit: EmitOptions;
    pruning: PruningRunOptions | null;
  }>;

export type PipelineRunDetails = Readonly<{
  text: string;
  pruning:
    | readonly Readonly<{
        query: string;
        matched: number;
      }>[]
    | null;
}>;

export type Pipeline = Readonly<{
  runDetailed(code: string, opts: PipelineRunOptions): PipelineRunDetails;
}>;

export type PipelineConfig = Readonly<{
  parser: Parser;
  analyzer: ScopeAnalyzer;
  serializer: IRSerializer;
  emitters: EmitterRegistry;
}>;
