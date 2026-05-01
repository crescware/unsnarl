import type { VisualNode } from "../../visual-graph/model.js";
import type { MermaidStrategy } from "./strategy/strategy.js";

export type RenderState = Readonly<{
  lines: /* mutable */ string[];
  nodeMap: ReadonlyMap<string, VisualNode>;
  wrappedOwnerIds: ReadonlySet<string>;
  edgeEndpointIds: ReadonlySet<string>;
  placeholderIds: /* mutable */ string[];
  wrapperIds: /* mutable */ string[];
  strategy: MermaidStrategy;
}>;
