export type CliLanguage = "ts" | "tsx" | "js" | "jsx";

export const LANGUAGES: ReadonlySet<string> = new Set([
  "ts",
  "tsx",
  "js",
  "jsx",
] satisfies CliLanguage[]);
