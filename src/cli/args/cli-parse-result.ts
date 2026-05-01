import type { CliArgs } from "./cli-args.js";

export type CliParseSuccess = Readonly<{
  ok: true;
  args: CliArgs;
}>;

export type CliParseFailure = Readonly<{
  ok: false;
  error: string;
}>;

export type CliParseResult = CliParseSuccess | CliParseFailure;
