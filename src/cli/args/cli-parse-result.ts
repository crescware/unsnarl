import type { CliArgs } from "./cli-args.js";

export type CliParseSuccess = {
  ok: true;
  args: CliArgs;
};

export type CliParseFailure = {
  ok: false;
  error: string;
};

export type CliParseResult = CliParseSuccess | CliParseFailure;
