import { AST_TYPE } from "../../parser/ast-type.js";
import type { NodeLike } from "./node-like.js";

const CASE_TEST_MAX_LENGTH = 32;

export function formatCaseTest(node: NodeLike, raw: string): string {
  const start = node.start;
  const end = node.end;
  if (
    typeof start === "number" &&
    typeof end === "number" &&
    end > start &&
    end <= raw.length &&
    end - start <= CASE_TEST_MAX_LENGTH
  ) {
    return raw.slice(start, end);
  }
  switch (node.type) {
    case AST_TYPE.Identifier: {
      const name = node["name"];
      return typeof name === "string" ? name : "<expr>";
    }
    case AST_TYPE.NullLiteral:
      return "null";
    case AST_TYPE.BooleanLiteral:
    case AST_TYPE.NumericLiteral:
    case AST_TYPE.StringLiteral: {
      const value = node["value"];
      if (typeof value === "string") {
        return JSON.stringify(value);
      }
      if (
        typeof value === "number" ||
        typeof value === "boolean" ||
        value === null
      ) {
        return String(value);
      }
      return "<expr>";
    }
    default:
      return "<expr>";
  }
}
