# integration/fixtures/imports/default-only/input.ts

## Input

```ts
import def from "m";

const x = def;
```

## Mermaid

```mermaid
%%{init: {"flowchart": {"defaultRenderer": "elk"}}}%%
flowchart RL
  n_scope_0_x_28["unused x<br/>L3"]
  subgraph sg_m["module m"]
    direction RL
    n_scope_0_def_7["import def<br/>L1"]
  end
  n_scope_0_def_7 -->|read| n_scope_0_x_28
  classDef nestL1 fill:#11192a,stroke:transparent;
  class sg_m nestL1;
```
