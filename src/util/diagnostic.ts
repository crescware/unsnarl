import type { Diagnostic, DiagnosticKind, Span } from "../ir/model.js";

export class DiagnosticCollector {
  private readonly items: /* mutable */ Diagnostic[] = [];

  add(kind: DiagnosticKind, message: string, span?: Span): void {
    if (span === undefined) {
      this.items.push({ kind, message });
    } else {
      this.items.push({ kind, message, span });
    }
  }

  list(): readonly Diagnostic[] {
    return this.items;
  }
}
