# integration/fixtures/const-chain-five/input.ts

## Input

```ts
const a = 1;
const b = a;
const c = b;
const d = c;
const e = d;
```

## Query

```sh
-r e -C 1
```

## Mermaid

```mermaid
%%{init: {"flowchart": {"defaultRenderer": "elk"}}}%%
flowchart RL
  %% pruning: roots=[e(1)] ancestors=1 descendants=1
  n_scope_0_d_45["d<br/>L4"]
  n_scope_0_e_58["unused e<br/>L5"]
  n_scope_0_d_45 -->|read| n_scope_0_e_58
  boundary_stub_1((…))
  boundary_stub_1 -.->|read| n_scope_0_d_45
  classDef boundaryStub fill:transparent,stroke:#888,stroke-dasharray:3 3,color:#888;
  class boundary_stub_1 boundaryStub;
```
