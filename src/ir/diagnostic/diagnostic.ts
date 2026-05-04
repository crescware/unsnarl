import type { DiagnosticKind } from "../../analyzer/diagnostic-kind.js";
import type { Span } from "../primitive/span.js";

export type Diagnostic = Readonly<{
  kind: DiagnosticKind;
  message: string;
  span: Span;
}>;
