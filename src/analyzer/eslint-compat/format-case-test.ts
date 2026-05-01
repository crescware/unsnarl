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
    case "Identifier": {
      const name = node["name"];
      return typeof name === "string" ? name : "<expr>";
    }
    case "NullLiteral":
      return "null";
    case "BooleanLiteral":
    case "NumericLiteral":
    case "StringLiteral": {
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
