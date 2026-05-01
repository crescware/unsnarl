import type { SerializedIR } from "../../ir/model.js";
import type { EmitOptions, Emitter } from "../../pipeline/types.js";
import type { MermaidEmitter } from "../mermaid/mermaid.js";
import { codeFenceLang } from "./code-fence-lang.js";
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
    const lines = [
      `# ${ir.source.path}`,
      "",
      "## Input",
      "",
      `\`\`\`${fence}`,
      raw,
      "```",
      "",
    ] satisfies string[];
    const pruning = opts.prunedGraph?.pruning ?? null;
    if (pruning !== null) {
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
