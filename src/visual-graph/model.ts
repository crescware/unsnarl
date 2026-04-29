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
  type: "node";
  id: string;
  kind: NodeKind;
  name: string;
  line: number;
  unused?: boolean;
  declarationKind?: VariableDeclarationKind;
  initIsFunction?: boolean;
  importKind?: ImportKind;
  importedName?: string | null;
  importSource?: string;
}

export interface VisualSubgraph {
  type: "subgraph";
  id: string;
  kind: SubgraphKind;
  line: number;
  direction: Direction;
  caseTest?: string | null;
  hasElse?: boolean;
  ownerNodeId?: string;
  elements: VisualElement[];
}

export type VisualElement = VisualNode | VisualSubgraph;

export interface VisualEdge {
  from: string;
  to: string;
  label: string;
}

export interface VisualGraph {
  version: 1;
  source: { path: string; language: Language };
  direction: Direction;
  elements: VisualElement[];
  edges: VisualEdge[];
}
