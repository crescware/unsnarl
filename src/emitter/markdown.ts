import type { Language, SerializedIR } from "../ir/model.js";
import type { EmitOptions, Emitter } from "../pipeline/types.js";
import type { VisualGraphPruning } from "../visual-graph/model.js";
import type { MermaidEmitter } from "./mermaid.js";

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
    const lines = [
      `# ${ir.source.path}`,
      "",
      "## Input",
      "",
      `\`\`\`${fence}`,
      raw,
      "```",
      "",
    ];
    const pruning = opts.prunedGraph?.pruning;
    if (pruning !== undefined) {
      lines.push(
        "## Query",
        "",
        "```sh",
        formatPruningQuery(pruning),
        "```",
        "",
      );
    }
    lines.push("## Mermaid", "", "```mermaid", mermaid, "```", "");
    return `${lines.join("\n")}`;
  }
}

function formatPruningQuery(pruning: VisualGraphPruning): string {
  const roots = pruning.roots.map((r) => r.query).join(",");
  if (pruning.descendants === pruning.ancestors) {
    return `-r ${roots} -C ${pruning.descendants}`;
  }
  return `-r ${roots} -A ${pruning.descendants} -B ${pruning.ancestors}`;
}

function codeFenceLang(language: Language): string {
  switch (language) {
    case "tsx":
      return "tsx";
    case "jsx":
      return "jsx";
    case "js":
      return "js";
    default:
      return "ts";
  }
}
