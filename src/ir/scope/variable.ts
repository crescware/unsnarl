import type { AstIdentifier } from "../primitive/ast-identifier.js";
import type { Reference } from "../reference/reference.js";
import type { Definition } from "./definition.js";
import type { Scope } from "./scope.js";

// Mutable: declareVariable pushes into identifiers/defs and bindReference
// pushes into references during analysis.
export type Variable = {
  name: string;
  scope: Scope;
  identifiers: /* mutable */ AstIdentifier[];
  references: /* mutable */ Reference[];
  defs: /* mutable */ Definition[];
  unsnarlIsUnused(): boolean;
};
