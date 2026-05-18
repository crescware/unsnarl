# Visual graph `NODE_KIND`

This directory defines `NODE_KIND` — the discriminator that labels
every node in the visual graph. Each `NODE_KIND` value belongs to
exactly one of four categories. This README documents the categories
and the naming rule for each.

Read this before adding, renaming, or removing a `NODE_KIND` value.

## Naming philosophy

`NODE_KIND` aligns with ECMAScript spec grammar. Where the spec
distinguishes concepts (`var` / `let` / `const`, `ForStatement` /
`ForInStatement` / `ForOfStatement`, named / default / namespace
imports), `NODE_KIND` makes the same distinction at the kind level.
The kind value alone identifies what the node represents; no
companion field further refines its meaning.

When a value has no ECMAScript spec correspondence, the name marks
it as unsnarl-specific. The marking takes one of two forms, chosen by
whether the visual graph node has an underlying AST node:

- `Synthetic` prefix — the node has no underlying AST node. It either
  represents an AST position (a property slot such as
  `IfStatement.test`) or is a pure graph construct with no AST
  counterpart at all.
- `Reference` suffix — the node corresponds to an actual AST
  Identifier that the visual graph has selected for its own node,
  based on some criterion (the Identifier's role, or the position it
  occupies).

Spec-aligned and unsnarl-specific values are therefore distinguishable
from the name alone.

The convention is "spec-aligned vocabulary", not "literal spec
production names". Names like `VarBinding` / `LetBinding` /
`ConstBinding` track the spec's semantic distinctions even where the
spec's literal production names (`VariableDeclaration`,
`LexicalBinding`) are not reused.

## Categories and naming rules

### 1. Spec-derived binding

Represents an Identifier in a binding position defined by the
ECMAScript spec.

**Rule.** Use the spec parent production name when the spec gives one
binding name per production. When one spec interface aggregates
multiple semantic forms via a kind/operator field, each form becomes a
distinct `NODE_KIND` value.

Examples:

- `FunctionDeclaration`, `ClassDeclaration`, `FormalParameter`,
  `CatchParameter` — spec production names kept as-is.
- `VarBinding`, `LetBinding`, `ConstBinding` — one kind per
  `VariableDeclaration.kind` value.
- `NamedImportBinding`, `DefaultImportBinding`,
  `NamespaceImportBinding` — one kind per import specifier shape.

### 2. Identifier reference

Represents an AST Identifier that the visual graph promotes to its
own node by a selection criterion. The criterion may be the
Identifier's role (read, write) or the position it occupies.

**Rule.** Name describing the selection criterion (role name or
position name) + `Reference` suffix. No `Synthetic` prefix — the
underlying node is an actual AST Identifier.

Examples:

- `WriteReference` — Identifier written by
  `AssignmentExpression.left`, `UpdateExpression.argument`, or a
  pattern-Lhs in an `assign` context. Role-defined capture; the name
  is the role.
- `ReturnArgumentReference` — Identifier read inside
  `ReturnStatement.argument`. Position-defined capture; the name is
  the position.

The capture scope is intentionally asymmetric between the two
existing values. `WriteReference` promotes every Identifier playing
the write role — the role itself is the entire criterion, and that
criterion alone defines what gets nodified. `ReturnArgumentReference`
promotes Identifiers only when read inside a return statement's
argument; other reads do not get a dedicated node (they are absorbed
by the surrounding statement anchor), so the criterion is positional
rather than role-defined. Each name carries the form of its
criterion: a role name when the role is the criterion, a position
name when the position is the criterion.

### 3. Synthetic anchor

Represents an AST _position_ that has no underlying AST Identifier;
the visual graph emits a virtual node to anchor edges and reference
aggregations at that position. The position may be a specific
property slot (e.g. `IfStatement.test`), a region of the parent that
aggregates multiple slots (e.g. the `ForStatement` header), or the
parent node taken as a whole (e.g. `ExpressionStatement`).

**Rule.** `Synthetic` prefix + spec parent node name. Append a
role/position suffix when the anchor narrows to a specific slot or
region of the parent; omit the suffix when the anchor covers the
parent node in its entirety.

Examples:

- `SyntheticIfStatementTest` — narrows to the `IfStatement.test`
  slot. Suffix `Test` names the slot.
- `SyntheticSwitchStatementDiscriminant` — narrows to the
  `SwitchStatement.discriminant` slot.
- `SyntheticWhileStatementTest`, `SyntheticDoWhileStatementTest` —
  analogous `test` slots.
- `SyntheticForStatementHeader`, `SyntheticForInStatementHeader`,
  `SyntheticForOfStatementHeader` — narrows to the header region
  (`init`/`test`/`update` aggregated). One anchor per For production.
  `Header` names the region.
- `SyntheticExpressionStatement` — covers the entire
  `ExpressionStatement` node, with no narrower slot to name, so no
  suffix is appended. The `Synthetic` prefix distinguishes the anchor
  from the AST node of the same root name.

### 4. Synthetic graph-only

Has no AST counterpart at all. Exists because the visual graph needs
nodes for module sinks, module sources, import bridging, and depth
limits.

**Rule.** `Synthetic` prefix + descriptive name.

Examples:

- `SyntheticModuleSink` — visual-graph root for the analyzed module.
  Import edges terminate here.
- `SyntheticModuleSource` — one node per distinct
  `ImportDeclaration.source` string.
- `SyntheticImportIntermediate` — bridges `SyntheticModuleSource` and
  a `NamedImportBinding` when the imported name differs from the local
  name.
- `SyntheticImplicitGlobal` — synthetic binding for references that
  escape scope resolution. Has no source location (`line: 0`).
- `SyntheticBeyondDepth` — boundary stub at the depth limit, anchoring
  edges that cross a collapsed subtree.

## Adding a new `NODE_KIND`

Work through these questions in order. The first answer that fits
selects the category and the naming rule.

1. **Does the new kind correspond to an ECMAScript spec production?**
   - If yes and the spec gives one binding name per production: use
     the production name. Category 1.
   - If yes but one spec interface aggregates multiple semantic forms
     via a kind/operator field: introduce one `NODE_KIND` value per
     form. Category 1.
2. **Is the new kind an AST Identifier that the visual graph promotes
   to its own node by a selection criterion (role or position)?**
   - If yes: name describing the criterion + `Reference` suffix.
     Category 2.
3. **Does the new kind correspond to an AST position with no
   underlying Identifier — a property slot, a region of the parent,
   or the parent node taken as a whole?**
   - If yes: `Synthetic` + spec parent node name + (role/position
     suffix when narrowing to a slot or region). Category 3.
4. **Does the new kind have no AST counterpart at all?**
   - Then: `Synthetic` + descriptive name. Category 4.

If none of the four fits, the construct may be miscategorized;
reconsider before naming. When introducing a category 2, 3, or 4
value, also add an entry to "Why each unsnarl-only kind exists" below
explaining the reason.

## Relationship with `DEFINITION_TYPE`

`src/analyzer/definition-type.ts` defines `DEFINITION_TYPE` — the
values that eslint-scope's `Definition.type` field exposes to
downstream tools. `DEFINITION_TYPE` is part of the eslint-scope
compatibility contract (see `docs/architecture.md` and the "Boundary
Invariants" section of the project `CLAUDE.md`) and is fixed.

The mapping from `DEFINITION_TYPE` to `NODE_KIND` lives in
`src/visual-graph/builder/make-variable-node.ts`:

| `DEFINITION_TYPE` value  | `NODE_KIND` value                                                        | Dispatched by         |
| ------------------------ | ------------------------------------------------------------------------ | --------------------- |
| `Variable`               | `VarBinding` / `LetBinding` / `ConstBinding`                             | `def.declarationKind` |
| `FunctionName`           | `FunctionDeclaration`                                                    | —                     |
| `ClassName`              | `ClassDeclaration`                                                       | —                     |
| `Parameter`              | `FormalParameter`                                                        | —                     |
| `CatchClause`            | `CatchParameter`                                                         | —                     |
| `ImportBinding`          | `NamedImportBinding` / `DefaultImportBinding` / `NamespaceImportBinding` | `def.importKind`      |
| `ImplicitGlobalVariable` | `SyntheticImplicitGlobal`                                                | —                     |

## Why each unsnarl-only kind exists

For kinds in categories 2, 3, and 4 — those without a spec
correspondence — the reason for their existence is recorded here.
Adding a new unsnarl-only kind requires adding an entry.

- **`WriteReference`** — spec has no single production for "write".
  The relevant constructs (`AssignmentExpression`,
  `UpdateExpression`, pattern-Lhs in `assign` context) are aggregated
  into one box so the visual graph can render "this Identifier is
  mutated at this site" with a single node.
- **`ReturnArgumentReference`** — the visual graph surfaces returned
  reads so the dataflow into the return value is visible. Other reads
  are absorbed by their enclosing statement anchor and do not get a
  dedicated node.
- **`SyntheticIfStatementTest`, `SyntheticSwitchStatementDiscriminant`,
  `SyntheticWhileStatementTest`, `SyntheticDoWhileStatementTest`,
  `SyntheticForStatementHeader`, `SyntheticForInStatementHeader`,
  `SyntheticForOfStatementHeader`** — anchors for control-flow
  predicates. Each is a position in the parent statement; the visual
  graph nodifies it so that references participating in the predicate
  attach to a stable target.
- **`SyntheticExpressionStatement`** — aggregates references that
  share an `ExpressionStatement` parent into one box so a single
  statement does not produce a fan of disconnected nodes.
- **`SyntheticModuleSink`** — module-level root. Edges from import
  bindings terminate here so the import structure has a single sink
  per module.
- **`SyntheticModuleSource`** — one node per distinct external module
  string, so multiple imports from the same source share a node.
- **`SyntheticImportIntermediate`** — emitted only when a
  `NamedImportBinding`'s imported name differs from its local name,
  surfacing the alias as a visible intermediate.
- **`SyntheticImplicitGlobal`** — represents references that escape
  scope resolution. The analyzer creates a synthetic binding so the
  reference still has a target; the visual graph carries it as a
  graph-only node with no source location.
- **`SyntheticBeyondDepth`** — when a subtree is collapsed by the
  depth limit, edges crossing the boundary attach to this stub. One
  stub per collapsed subtree.
