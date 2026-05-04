import type { Language } from "../../cli/language.js";

export function codeFenceLang(language: Language): string {
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
