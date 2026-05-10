import fg from "fast-glob";
import { readFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import colors from "yoctocolors";

import { findRepositoryRootPath } from "./find-repository-root-path";
import {
  findToBeMatches,
  getDisplayLabel,
  getErrorMessage,
} from "./find-to-be-matches";

type LintError = Readonly<{
  file: string;
  matches: readonly string[];
}>;

export async function main(): Promise<void> {
  const scriptDirectoryPath = path.dirname(fileURLToPath(import.meta.url));
  const repositoryRootPath = findRepositoryRootPath(scriptDirectoryPath);
  const files = await fg("src/**/*.test.{ts,tsx}", {
    cwd: repositoryRootPath,
  });

  const errors: readonly LintError[] = files.reduce(
    (acc: LintError[], file: string): LintError[] => {
      const filePath = path.join(repositoryRootPath, file);
      const content = readFileSync(filePath, "utf-8");
      const matches = findToBeMatches(content);
      if (matches !== null) {
        acc.push({ file, matches });
      }
      return acc;
    },
    [],
  );

  if (errors.length === 0) {
    console.log(colors.green("No toBe errors found"));
    return;
  }

  errors.forEach(({ file, matches }) => {
    console.error(colors.bold(colors.red(file)));
    matches.forEach((match) => {
      console.error(
        `   ${colors.red(getDisplayLabel(match))} — ${getErrorMessage(match)}`,
      );
    });
  });

  process.exit(1);
}
