import type { CliLanguage } from "../../language.js";

export type ExecuteSource =
  | Readonly<{
      stdin: true;
      text: string;
      stdinLang: CliLanguage;
    }>
  | Readonly<{
      stdin: false;
      path: string;
    }>;
