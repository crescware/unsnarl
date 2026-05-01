import type { CliArgs } from "./cli-args.js";

export interface CliParseSuccess {
  ok: true;
  args: CliArgs;
}

export interface CliParseFailure {
  ok: false;
  error: string;
}

export type CliParseResult = CliParseSuccess | CliParseFailure;
