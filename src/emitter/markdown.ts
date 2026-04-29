import type { Language, SerializedIR } from "../ir/model.js";
import type { EmitOptions, Emitter } from "../pipeline/types.js";
import type { MermaidEmitter } from "./mermaid.js";

export class MarkdownEmitter implements Emitter {
  readonly format = "markdown";
  readonly contentType = "text/markdown";

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
      "## Mermaid",
      "",
      "```mermaid",
      mermaid,
      "```",
      "",
    ];
    return `${lines.join("\n")}`;
  }
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
