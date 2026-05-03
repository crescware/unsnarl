# integration/fixtures/lprefix-exact-match/input.ts

## Notice

```
uns: 'L12' is ambiguous.
  An exact identifier match was found; interpreting as identifier.
  To disambiguate, use '-r 12'.
```

## Input

```ts
const L12 = 1;
const beta = L12 + 2;
const gamma = beta * 3;
```

## Query

```sh
-r L12 -C 1
```

## Mermaid

```mermaid
%%{init: {"flowchart": {"defaultRenderer": "elk"}}}%%
flowchart RL
  %% pruning roots L12=1 ancestors=1 descendants=1
  n_scope_0_L12_6["L12<br/>L1"]
  n_scope_0_beta_21["beta<br/>L2"]
  n_scope_0_L12_6 -->|read| n_scope_0_beta_21
  boundary_stub_1((...))
  n_scope_0_beta_21 -.-> boundary_stub_1
  classDef boundaryStub fill:transparent,stroke:#888,stroke-dasharray:3 3,color:#888;
  class boundary_stub_1 boundaryStub;
```
