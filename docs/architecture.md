# Architecture: the eslint-scope boundary

unsnarl reuses eslint-scope's public shape for `Scope` / `Variable` /
`Reference` / `Definition` so that downstream tools can swap analyzers
without code changes. To keep that promise honest while still allowing
unsnarl-specific analysis (predicates, return containers, JSX spans,
nesting depths, ownership, …), the source tree is split into three
regions with different rules.

## The three regions

### `src/boundary/eslint-scope/`

The compatibility region. Its job is to behave exactly like
eslint-scope. Everything here either:

- mirrors eslint-scope's traversal and binding algorithm
  (`analyze.ts`, `enter-*.ts`, `handle-*.ts`, `hoist-into.ts`,
  `declare-for-left.ts`, `declare-function-params.ts`, the
  `walk/`, `declare/`, `hoisting/`, `classify/` subtrees), or
- implements the eslint-scope IR classes that the algorithm produces
  (`scope-impl.ts`, `variable-impl.ts`, `reference-impl.ts`,
  `manager.ts`, `resolve.ts`), or
- pins the public shape at the type level (`contract/`).

If parity with eslint-scope says "do X here", code in this region
does X here, even when X looks suboptimal. That is the point.

### `src/analyzer/`

unsnarl-specific producers. They consume the IR that the boundary
already built and compute information that eslint-scope does not have:

- `predicate.ts`, `predicate-container-type.ts` — predicate detection
- `return-container.ts`, `block-context-of.ts` — control-flow shape
- `jsx-element-span.ts`, `expression-statement-container.ts` — span
  extraction
- `if-chain-root-offset.ts`, `is-control-exit.ts`,
  `is-function-exit.ts`, `case-*.ts`, `format-case-test.ts` — branch
  classification helpers
- `is-unused.ts` — replacement for the previously-method-based
  `Variable.unsnarlIsUnused()`
- `compute-nesting-depths.ts` — nesting metric
- `owner/` — reference-owner resolution
- `annotations-impl.ts` — concrete `Annotations` side-table
- shared vocabulary used by both regions (`scope-type.ts`,
  `definition-type.ts`, `diagnostic-kind.ts`)

These files do not change the eslint-scope IR. They read it and emit
unsnarl-only data.

### `src/ir/annotations/`

The side-table. It is the only channel through which unsnarl-specific
information attaches to a `Scope` or `Reference`:

- `Annotations.ofScope(scope)` returns a `ScopeAnnotation`.
- `Annotations.ofReference(ref)` returns a `ReferenceAnnotation`.

Missing entries return zero-value defaults so callers never need to
special-case absence. The implementation lives in
`src/analyzer/annotations-impl.ts`.

## Where to put a change

When deciding which region a change belongs in, ask:

- **Does it affect what `Scope` / `Variable` / `Reference` /
  `Definition` look like, or what eslint-scope's algorithm would
  produce for a given input?**
  → `src/boundary/eslint-scope/`. Parity tests will catch drift.
- **Does it compute extra information for unsnarl, leaving the
  eslint-scope IR untouched?**
  → `src/analyzer/`, plumbing the result through
  `src/ir/annotations/` and `annotations-impl.ts`.
- **Does it add a new field to the IR itself?**
  → almost certainly the wrong question. Add an annotation field
  and route it through the side-table instead. Mutating the compat
  IR shape is a parity-breaking change.

## The contract

`src/boundary/eslint-scope/contract/` holds type-level mirrors of
eslint-scope's public API:

- `compat-scope.ts`, `compat-variable.ts`, `compat-reference.ts`,
  `compat-definition.ts` — minimal types that capture the
  non-deprecated public shape.
- `contract-assertion.ts` — compile-time assertions that the unsnarl
  IR types satisfy the contract types. If any assertion stops
  type-checking, the IR has drifted from eslint-scope and parity
  will (or already does) regress.

The assertion file has no runtime importer, so `knip.ts` lists it
under `entry` to keep its imports walked. See
`src/boundary/eslint-scope/contract/contract-assertion.ts:1` and
`knip.ts:11` for the registered entry.

Removing a field from the contract because eslint-scope still exposes
it (as long as it is not deprecated upstream) is itself a parity
break. Fields are deliberately excluded only when eslint-scope marks
them deprecated; the comment block in each `compat-*.ts` documents
those exclusions.

## Invariants enforced at the directory level

- `src/boundary/eslint-scope/` does not import from
  `src/ir/annotations/` or from any unsnarl-only producer in
  `src/analyzer/` (predicate, return-container, jsx-element-span,
  block-context-of, owner/, …). The compat region does not see the
  side-table or the producers; it only emits the IR that they will
  later read.
- The reverse direction is allowed: `src/analyzer/` and the rest of
  the pipeline import from `src/boundary/eslint-scope/` freely.
- The shared vocabulary files (`scope-type.ts`, `definition-type.ts`,
  `diagnostic-kind.ts`) live in `src/analyzer/` and are imported by
  both regions. They are enums/constants, not algorithms.

## Related documentation

- `src/visual-graph/README.md` — naming philosophy, categories, and
  decision rules for `NODE_KIND` values. Read this before adding or
  renaming a `NODE_KIND`. The relationship between `NODE_KIND` and
  `DEFINITION_TYPE` (which `DEFINITION_TYPE` must preserve as part of
  the eslint-scope contract) is documented there.
