# integration/fixtures/app-behavior/ast-type-coverage/ts-type-predicate/input.ts

## Input

```ts
function isStr(x: unknown): x is string {
  return typeof x === "string";
}
```

## Mermaid

```mermaid
%%{init: {"flowchart": {"defaultRenderer": "elk"}}}%%
flowchart RL
  subgraph wrap_s_scope_1[" "]
    direction TB
    n_scope_0_isStr_9["unused isStr()<br/>L1"]
    subgraph s_scope_1["isStr()<br/>L1-3"]
      direction RL
      n_scope_1_x_15["x<br/>L1"]
      subgraph s_return_scope_0_isStr_9_44_73["return L2"]
        direction RL
        ret_use_ref_0["x<br/>L2"]
      end
    end
  end
  n_scope_1_x_15 -->|read| ret_use_ref_0
  classDef nestL1 fill:#11192a,stroke:transparent;
  class wrap_s_scope_1 nestL1;
  classDef nestL2 fill:#1a2538,stroke:transparent;
  class s_scope_1 nestL2;
  classDef nestL3 fill:#243047,stroke:transparent;
  class s_return_scope_0_isStr_9_44_73 nestL3;
```
