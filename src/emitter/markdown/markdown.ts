import { DIAGNOSTIC_KIND } from "../../analyzer/diagnostic-kind.js";
import { formatVarDiagnostic } from "../../analyzer/format-var-diagnostic.js";
import type { SerializedIR } from "../../ir/serialized/serialized-ir.js";
import type { EmitOptions } from "../../pipeline/emit/emit-options.js";
import type { Emitter } from "../../pipeline/emit/emitter.js";
import { formatResolutionNotice } from "../../visual-graph/prune/format-resolution-notice.js";
import type { MermaidEmitter } from "../mermaid/mermaid.js";
import { codeFenceLang } from "./code-fence-lang.js";
import { formatDepthQuery } from "./format-depth-query.js";
import { formatHighlightQuery } from "./format-highlight-query.js";
import { formatPruningQuery } from "./format-pruning-query.js";

export class MarkdownEmitter implements Emitter {
  readonly format = "markdown";
  readonly contentType = "text/markdown";
  readonly extension = "md";

  private readonly mermaid: MermaidEmitter;

  constructor(mermaid: MermaidEmitter) {
    this.mermaid = mermaid;
  }

  emit(ir: SerializedIR, opts: EmitOptions): string {
    const mermaid = this.mermaid.emit(ir, opts).replace(/\n+$/, "");
    const raw = ir.raw.replace(/\n+$/, "");
    const fence = codeFenceLang(ir.source.language);
    const lines: string[] = [`# ${ir.source.path}`, ""];
    const resolutions = opts.resolutions ?? [];
    const varDiagnostics = ir.diagnostics.filter(
      (diagnostic) => diagnostic.kind === DIAGNOSTIC_KIND.VarDetected,
    );
    if (resolutions.length > 0 || varDiagnostics.length > 0) {
      lines.push("## Notice", "", "```");
      for (const resolution of resolutions) {
        lines.push(...formatResolutionNotice(resolution));
      }
      for (const diagnostic of varDiagnostics) {
        lines.push(...formatVarDiagnostic(diagnostic));
      }
      lines.push("```", "");
    }
    lines.push("## Input", "", `\`\`\`${fence}`, raw, "```", "");
    const pruning = opts.prunedGraph?.pruning ?? null;
    const depthQuery = formatDepthQuery(opts.depths);
    const highlight = opts.highlight;
    if (pruning !== null || depthQuery !== null || highlight !== null) {
      const parts: string[] = [];
      if (pruning !== null) {
        parts.push(formatPruningQuery(pruning));
      }
      if (depthQuery !== null) {
        parts.push(depthQuery);
      }
      if (highlight !== null) {
        parts.push(formatHighlightQuery(highlight));
      }
      lines.push("## Query", "", "```sh", parts.join(" "), "```", "");
    }
    lines.push("## Mermaid", "", "```mermaid", mermaid, "```", "");
    return `${lines.join("\n")}`;
  }
}
