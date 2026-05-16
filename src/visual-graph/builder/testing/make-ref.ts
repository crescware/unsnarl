import { parse } from "valibot";

import {
  serializedReference$,
  type SerializedReference,
} from "../../../ir/serialized/serialized-reference.js";
import { normalCompletion } from "./completion.js";
import { span } from "./span.js";

export function baseRef(): SerializedReference {
  return parse(serializedReference$, {
    id: "r",
    identifier: { name: "x", span: span() },
    from: "s",
    resolved: null,
    owners: [],
    init: false,
    flags: { read: false, write: false, call: false, receiver: false },
    predicateContainer: null,
    completion: normalCompletion(),
    jsxElement: null,
    expressionStatementContainer: null,
  });
}
