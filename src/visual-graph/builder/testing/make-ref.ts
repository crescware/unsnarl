import type { SerializedReference } from "../../../ir/model.js";
import { span } from "./span.js";

export function baseRef(): SerializedReference {
  return {
    id: "r",
    identifier: { name: "x", span: span() },
    from: "s",
    resolved: null,
    owners: [],
    init: false,
    flags: { read: false, write: false, call: false, receiver: false },
    predicateContainer: null,
    returnContainer: null,
    jsxElement: null,
    expressionStatementContainer: null,
  };
}
