import { existsSync, mkdtempSync, readFileSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { afterEach, beforeEach, describe, expect, test, vi } from "vitest";

import { writeOutput } from "./write-output.js";

describe("writeOutput", () => {
  describe("when outputPath is null", () => {
    let stdoutSpy: ReturnType<typeof vi.spyOn>;
    let written: /* mutable */ string[];

    beforeEach(() => {
      written = [];
      stdoutSpy = vi
        .spyOn(process.stdout, "write")
        .mockImplementation((chunk: unknown) => {
          written.push(typeof chunk === "string" ? chunk : String(chunk));
          return true;
        });
    });

    afterEach(() => {
      stdoutSpy.mockRestore();
    });

    test("writes the text to stdout", () => {
      writeOutput(null, "hello\n");
      expect(written).toEqual(["hello\n"]);
    });

    test("writes empty text to stdout", () => {
      writeOutput(null, "");
      expect(written).toEqual([""]);
    });
  });

  describe("when outputPath is given", () => {
    let stdoutSpy: ReturnType<typeof vi.spyOn>;
    let tmpDir: string;

    beforeEach(() => {
      stdoutSpy = vi.spyOn(process.stdout, "write").mockImplementation(() => {
        return true;
      });
      tmpDir = mkdtempSync(join(tmpdir(), "unsnarl-write-output-"));
    });

    afterEach(() => {
      stdoutSpy.mockRestore();
      rmSync(tmpDir, { recursive: true, force: true });
    });

    test("writes the text to the given path and does not touch stdout", () => {
      const outputPath = join(tmpDir, "out.txt");

      writeOutput(outputPath, "file body\n");

      expect(readFileSync(outputPath, "utf8")).toBe("file body\n");
      expect(stdoutSpy).not.toHaveBeenCalled();
    });

    test("creates intermediate directories that do not yet exist", () => {
      const outputPath = join(tmpDir, "deep", "nested", "out.txt");

      writeOutput(outputPath, "nested\n");

      expect(existsSync(join(tmpDir, "deep", "nested"))).toBe(true);
      expect(readFileSync(outputPath, "utf8")).toBe("nested\n");
    });

    test("overwrites an existing file at the path", () => {
      const outputPath = join(tmpDir, "out.txt");

      writeOutput(outputPath, "first\n");
      writeOutput(outputPath, "second\n");

      expect(readFileSync(outputPath, "utf8")).toBe("second\n");
    });
  });
});
