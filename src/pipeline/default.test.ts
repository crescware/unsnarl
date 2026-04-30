import { beforeEach, describe, expect, test, vi } from "vitest";

import { IrEmitter } from "../emitter/ir.js";
import { JsonEmitter } from "../emitter/json.js";
import { MarkdownEmitter } from "../emitter/markdown.js";
import { MermaidEmitter } from "../emitter/mermaid/mermaid.js";
import { dagreStrategy } from "../emitter/mermaid/strategy/dagre-strategy.js";
import { elkStrategy } from "../emitter/mermaid/strategy/elk-strategy.js";
import { createDefaultEmitterRegistry } from "./default.js";

vi.mock("../emitter/ir.js", () => ({
  IrEmitter: vi.fn(function IrEmitter() {
    return {
      format: "ir",
      contentType: "application/json",
      extension: "json",
      emit: vi.fn(),
    };
  }),
}));
vi.mock("../emitter/json.js", () => ({
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
vi.mock("../emitter/markdown.js", () => ({
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
    createDefaultEmitterRegistry({ mermaidRenderer: "elk" });
    expect(IrEmitter).toHaveBeenCalledTimes(1);
    expect(JsonEmitter).toHaveBeenCalledTimes(1);
    expect(MermaidEmitter).toHaveBeenCalledTimes(1);
    expect(MarkdownEmitter).toHaveBeenCalledTimes(1);
  });

  test("forwards the mermaid renderer choice into MermaidEmitter", () => {
    createDefaultEmitterRegistry({ mermaidRenderer: "elk" });
    expect(MermaidEmitter).toHaveBeenCalledWith({ strategy: elkStrategy });

    vi.clearAllMocks();
    createDefaultEmitterRegistry({ mermaidRenderer: "dagre" });
    expect(MermaidEmitter).toHaveBeenCalledWith({ strategy: dagreStrategy });
  });

  test("MarkdownEmitter receives the SAME MermaidEmitter instance, not a fresh one", () => {
    createDefaultEmitterRegistry({ mermaidRenderer: "elk" });
    const mermaidConstructor = vi.mocked(MermaidEmitter);
    const markdownConstructor = vi.mocked(MarkdownEmitter);
    expect(mermaidConstructor.mock.results).toHaveLength(1);
    const mermaidInstance = mermaidConstructor.mock.results[0]?.value as object;
    expect(markdownConstructor).toHaveBeenCalledWith(mermaidInstance);
  });
});
