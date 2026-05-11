import type { VisualNode } from "../../visual-graph/visual-node.js";
import type { MermaidStrategy } from "./strategy/strategy.js";
import type { ColorTheme } from "./theme/color-theme.js";

export type RenderState = Readonly<{
  lines: /* mutable */ string[];
  nodeMap: ReadonlyMap<string, VisualNode>;
  wrappedOwnerIds: ReadonlySet<string>;
  edgeEndpointIds: ReadonlySet<string>;
  placeholderIds: /* mutable */ string[];
  // Subgraph ids grouped by 0-based palette slot. Filled as subgraphs
  // (including function wrappers) are emitted with their depth; consumed
  // by renderClassDefs to emit the per-level `classDef nestL<n>` /
  // `class ... nestL<n>` rows.
  nestClassMap: /* mutable */ Map<number, /* mutable */ string[]>;
  strategy: MermaidStrategy;
  theme: ColorTheme;
  debug: boolean;
}>;
