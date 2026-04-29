import type {
  ImportKind,
  Language,
  VariableDeclarationKind,
} from "../ir/model.js";

export type Direction = "RL" | "LR" | "TB" | "BT";

export type NodeKind =
  | "Variable"
  | "FunctionName"
  | "ClassName"
  | "Parameter"
  | "CatchClause"
  | "ImportBinding"
  | "ImplicitGlobalVariable"
  | "WriteOp"
  | "ReturnSink"
  | "ModuleSink"
  | "ModuleSource"
  | "ImportIntermediate";

export type SubgraphKind =
  | "function"
  | "switch"
  | "case"
  | "if"
  | "else"
  | "if-else-container"
  | "try"
  | "catch"
  | "finally"
  | "for";

export interface VisualNode {
  id: string;
  kind: NodeKind;
  name: string;
  line: number;
  parent: string | null;
  unused?: boolean;
  declarationKind?: VariableDeclarationKind;
  initIsFunction?: boolean;
  importKind?: ImportKind;
  importedName?: string | null;
  importSource?: string;
}

export interface VisualSubgraph {
  id: string;
  kind: SubgraphKind;
  line: number;
  parent: string | null;
  direction: Direction;
  caseTest?: string | null;
  hasElse?: boolean;
  ownerNodeId?: string;
}

export interface VisualEdge {
  from: string;
  to: string;
  label: string;
}

export interface VisualGraph {
  version: 1;
  source: { path: string; language: Language };
  direction: Direction;
  nodes: VisualNode[];
  subgraphs: VisualSubgraph[];
  edges: VisualEdge[];
}
