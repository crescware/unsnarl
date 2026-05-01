import type { VisualNode } from "../../visual-graph/model.js";
import type { MermaidStrategy } from "./strategy/strategy.js";

export interface RenderState {
  lines: string[];
  nodeMap: ReadonlyMap<string, VisualNode>;
  wrappedOwnerIds: ReadonlySet<string>;
  edgeEndpointIds: ReadonlySet<string>;
  placeholderIds: string[];
  wrapperIds: string[];
  strategy: MermaidStrategy;
}
