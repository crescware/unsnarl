import { beforeEach, describe, expect, test, vi } from "vitest";

import { IrEmitter } from "../emitter/ir/ir.js";
import { JsonEmitter } from "../emitter/json/json.js";
import { MarkdownEmitter } from "../emitter/markdown/markdown.js";
import { MermaidEmitter } from "../emitter/mermaid/mermaid.js";
import { createDefaultEmitterRegistry } from "./create-default-emitter-registry.js";

vi.mock("../emitter/ir/ir.js", () => ({
  IrEmitter: vi.fn(function IrEmitter() {
    return {
      format: "ir",
      contentType: "application/json",
      extension: "json",
      emit: vi.fn(),
    };
  }),
}));

vi.mock("../emitter/json/json.js", () => ({
  JsonEmitter: vi.fn(function JsonEmitter() {
    return {
      format: "json",
      contentType: "application/json",
      extension: "json",
      emit: vi.fn(),
    };
  }),
}));

vi.mock("../emitter/mermaid/mermaid.js", () => ({
  MermaidEmitter: vi.fn(function MermaidEmitter() {
    return {
      format: "mermaid",
      contentType: "text/vnd.mermaid",
      extension: "mmd",
      emit: vi.fn(),
    };
  }),
}));

vi.mock("../emitter/markdown/markdown.js", () => ({
  MarkdownEmitter: vi.fn(function MarkdownEmitter() {
    return {
      format: "markdown",
      contentType: "text/markdown",
      extension: "md",
      emit: vi.fn(),
    };
  }),
}));

describe("createDefaultEmitterRegistry", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  test("constructs each of the four emitters exactly once", () => {
    createDefaultEmitterRegistry();

    expect(IrEmitter).toHaveBeenCalledTimes(1);
    expect(JsonEmitter).toHaveBeenCalledTimes(1);
    expect(MermaidEmitter).toHaveBeenCalledTimes(1);
    expect(MarkdownEmitter).toHaveBeenCalledTimes(1);
  });

  test("MarkdownEmitter receives the SAME MermaidEmitter instance, not a fresh one", () => {
    createDefaultEmitterRegistry();
    const mermaidConstructor = vi.mocked(MermaidEmitter);
    const markdownConstructor = vi.mocked(MarkdownEmitter);
    const mermaidInstance = mermaidConstructor.mock.results[0]?.value as object;

    expect(mermaidConstructor.mock.results).toHaveLength(1);
    expect(markdownConstructor).toHaveBeenCalledWith(mermaidInstance);
  });
});
