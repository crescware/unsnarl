import type { Reference } from "../../ir/reference/reference.js";
import type { Definition } from "../../ir/scope/definition.js";
import type { Scope } from "../../ir/scope/scope.js";
import type { Variable } from "../../ir/scope/variable.js";
import type { CompatDefinition } from "./compat-definition.js";
import type { CompatReference } from "./compat-reference.js";
import type { CompatScope } from "./compat-scope.js";
import type { CompatVariable } from "./compat-variable.js";

// Compile-time assertion that unsnarl's IR types satisfy the eslint-scope
// contract. If any of these aliases ever fail to type-check, the contract
// has been violated and parity will regress. Fix the IR type rather than
// relaxing the contract.
type AssertCompatReference<T extends CompatReference> = T;
type AssertCompatScope<T extends CompatScope> = T;
type AssertCompatVariable<T extends CompatVariable> = T;
type AssertCompatDefinition<T extends CompatDefinition> = T;

export type CheckReferenceContract = AssertCompatReference<Reference>;
export type CheckScopeContract = AssertCompatScope<Scope>;
export type CheckVariableContract = AssertCompatVariable<Variable>;
export type CheckDefinitionContract = AssertCompatDefinition<Definition>;
