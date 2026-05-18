# integration/fixtures/app-behavior/ast-type-coverage/accessor-property/input.ts

## Input

```ts
class C {
  accessor x = 1;
}
```

## Mermaid

```mermaid
%%{init: {"flowchart": {"defaultRenderer": "elk"}}}%%
flowchart RL
  n_scope_0_C_6["unused class C<br/>L1"]
  subgraph s_scope_1["class C<br/>L1-3"]
    direction RL
    n_scope_1_C_6["unused class C<br/>L1"]
  end
  classDef nestL1 fill:#11192a,stroke:transparent;
  class s_scope_1 nestL1;
```
