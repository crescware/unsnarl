import type { CliLanguage } from "../args/cli-language.js";

export function detectLanguage(path: string): CliLanguage {
  if (path.endsWith(".tsx")) {
    return "tsx";
  }
  if (path.endsWith(".jsx")) {
    return "jsx";
  }
  if (
    path.endsWith(".js") ||
    path.endsWith(".mjs") ||
    path.endsWith(".cjs") ||
    path.endsWith(".ejs")
  ) {
    return "js";
  }
  if (path.endsWith(".mts") || path.endsWith(".cts")) {
    return "ts";
  }
  return "ts";
}
