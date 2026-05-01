// Source language tag, shared between the IR (`Language`) and the CLI
// (`CliLanguage`). They alias the same value set so callers on either
// side don't import across domains.
export const LANGUAGE = {
  Ts: "ts",
  Tsx: "tsx",
  Js: "js",
  Jsx: "jsx",
} as const;
export type Language = (typeof LANGUAGE)[keyof typeof LANGUAGE];
export type CliLanguage = Language;
