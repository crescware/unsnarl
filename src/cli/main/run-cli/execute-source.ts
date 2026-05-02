import type { CliLanguage } from "../../args/cli-language.js";

export type ExecuteSource =
  | Readonly<{
      stdin: true;
      text: string;
      lang: CliLanguage;
    }>
  | Readonly<{
      stdin: false;
      path: string;
    }>;
