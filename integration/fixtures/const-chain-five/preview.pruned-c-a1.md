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
-r c -A 1 -B 0
```

## Mermaid

```mermaid
%%{init: {"flowchart": {"defaultRenderer": "elk"}}}%%
flowchart RL
  %% pruning: roots=[c(1)] ancestors=0 descendants=1
  n_scope_0_c_32["c<br/>L3"]
  n_scope_0_d_45["d<br/>L4"]
  n_scope_0_c_32 -->|read| n_scope_0_d_45
```
