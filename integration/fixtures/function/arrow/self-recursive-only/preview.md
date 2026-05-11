# integration/fixtures/function/arrow/self-recursive-only/input.ts

## Input

```ts
const a = () => a;
```

## Mermaid

```mermaid
%%{init: {"flowchart": {"defaultRenderer": "elk"}}}%%
flowchart RL
  subgraph wrap_s_scope_1[" "]
    direction TB
    n_scope_0_a_6["unused a()<br/>L1"]
    subgraph s_scope_1["a()<br/>L1"]
      direction RL
      subgraph s_return_scope_0_a_6_16_17["return L1"]
        direction RL
        ret_use_ref_1["a<br/>L1"]
      end
    end
  end
  n_scope_0_a_6 -->|read| ret_use_ref_1
  classDef nestL1 fill:#11192a,stroke:transparent;
  class wrap_s_scope_1 nestL1;
  classDef nestL2 fill:#1a2538,stroke:transparent;
  class s_scope_1 nestL2;
  classDef nestL3 fill:#243047,stroke:transparent;
  class s_return_scope_0_a_6_16_17 nestL3;
```
