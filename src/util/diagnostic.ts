import type { Diagnostic, DiagnosticKind, Span } from "../ir/model.js";

export class DiagnosticCollector {
  private readonly items: /* mutable */ Diagnostic[] = [];

  add(kind: DiagnosticKind, message: string, span: Span | null): void {
    this.items.push({ kind, message, span });
  }

  list(): readonly Diagnostic[] {
    return this.items;
  }
}
