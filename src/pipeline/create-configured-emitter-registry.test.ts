import { beforeEach, describe, expect, test, vi } from "vitest";

import { CLI_MERMAID_RENDERER } from "../cli-mermaid-renderer.js";
import { MermaidEmitter } from "../emitter/mermaid/mermaid.js";
import { dagreStrategy } from "../emitter/mermaid/strategy/dagre-strategy.js";
import { elkStrategy } from "../emitter/mermaid/strategy/elk-strategy.js";
import { createConfiguredEmitterRegistry } from "./create-configured-emitter-registry.js";

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

describe("createConfiguredEmitterRegistry", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  test("passes the elk strategy when mermaidRenderer is elk", () => {
    createConfiguredEmitterRegistry({
      mermaidRenderer: CLI_MERMAID_RENDERER.Elk,
    });
    expect(MermaidEmitter).toHaveBeenCalledWith({ strategy: elkStrategy });
  });

  test("passes the dagre strategy when mermaidRenderer is dagre", () => {
    createConfiguredEmitterRegistry({
      mermaidRenderer: CLI_MERMAID_RENDERER.Dagre,
    });
    expect(MermaidEmitter).toHaveBeenCalledWith({ strategy: dagreStrategy });
  });
});
