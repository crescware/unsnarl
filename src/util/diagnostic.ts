import type { DiagnosticKind } from "../analyzer/diagnostic-kind.js";
import type { Diagnostic, Span } from "../ir/model.js";

export class DiagnosticCollector {
  private readonly items: /* mutable */ Diagnostic[] = [];

  add(kind: DiagnosticKind, message: string, span: Span): void {
    this.items.push({ kind, message, span });
  }

  list(): readonly Diagnostic[] {
    return this.items;
  }
}
