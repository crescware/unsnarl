import type { AstIdentifier } from "../ir/primitive/ast-identifier.js";
import type { Reference } from "../ir/reference/reference.js";
import type { Definition } from "../ir/scope/definition.js";
import type { Scope } from "../ir/scope/scope.js";
import type { Variable } from "../ir/scope/variable.js";

export class VariableImpl implements Variable {
  readonly name: string;
  readonly scope: Scope;
  readonly identifiers: /* mutable */ AstIdentifier[] = [];
  readonly references: /* mutable */ Reference[] = [];
  readonly defs: /* mutable */ Definition[] = [];

  constructor(name: string, scope: Scope) {
    this.name = name;
    this.scope = scope;
  }

  unsnarlIsUnused(): boolean {
    return this.references.length === 0;
  }
}
