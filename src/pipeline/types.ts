import type { Diagnostic, Language, Scope, SerializedIR } from "../ir/model.js";

export interface ParseOptions {
  language: Language;
  sourcePath: string;
}

export interface ParsedSource {
  ast: unknown;
  language: Language;
  sourcePath: string;
  raw: string;
}

export interface Parser {
  readonly id: string;
  parse(code: string, opts: ParseOptions): ParsedSource;
}

export interface AnalyzedSource {
  readonly rootScope: Scope;
  readonly diagnostics: readonly Diagnostic[];
  readonly raw: string;
}

export interface ScopeAnalyzer {
  readonly id: string;
  analyze(parsed: ParsedSource): AnalyzedSource;
}

export interface SourceMeta {
  path: string;
  language: Language;
}

export interface SerializeContext {
  readonly rootScope: Scope;
  readonly source: SourceMeta;
  readonly diagnostics: readonly Diagnostic[];
  readonly raw: string;
}

export interface IRSerializer {
  readonly id: string;
  serialize(ctx: SerializeContext): SerializedIR;
}

export interface EmitOptions {
  pretty?: boolean;
}

export interface Emitter {
  readonly format: string;
  readonly contentType: string;
  emit(ir: SerializedIR, opts: EmitOptions): string;
}

export interface EmitterRegistry {
  register(emitter: Emitter): void;
  get(format: string): Emitter | undefined;
  list(): readonly string[];
}

export interface PipelineRunOptions extends ParseOptions {
  format: string;
  emit?: EmitOptions;
}

export interface Pipeline {
  run(code: string, opts: PipelineRunOptions): string;
}

export interface PipelineConfig {
  parser: Parser;
  analyzer: ScopeAnalyzer;
  serializer: IRSerializer;
  emitters: EmitterRegistry;
}
