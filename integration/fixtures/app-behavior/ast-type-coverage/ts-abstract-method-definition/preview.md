# integration/fixtures/app-behavior/ast-type-coverage/ts-abstract-method-definition/input.ts

## Input

```ts
abstract class C {
  abstract m(): void;
}
```

## Mermaid

```mermaid
%%{init: {"flowchart": {"defaultRenderer": "elk"}}}%%
flowchart RL
  n_scope_0_C_15["unused class C<br/>L1"]
  subgraph s_scope_1["class C<br/>L1-3"]
    direction RL
    n_scope_1_C_15["unused class C<br/>L1"]
  end
  classDef nestL1 fill:#11192a,stroke:transparent;
  class s_scope_1 nestL1;
```
