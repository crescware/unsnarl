# integration/fixtures/declaration/var/with-implicit-global/input.ts

## Notice

```
uns: warning: L1:0: var declaration detected; rendered as node only (no edges).
```

## Input

```ts
var a = 0;
global = 1;
```

## Mermaid

```mermaid
%%{init: {"flowchart": {"defaultRenderer": "elk"}}}%%
flowchart RL
  n_scope_0_a_4["var a<br/>L1"]
  n_scope_0_global_11["global global"]
  wr_ref_1(["global<br/>L2"])
  n_scope_0_global_11 -->|set| wr_ref_1
  classDef varNode stroke-dasharray:5 5;
  class n_scope_0_a_4 varNode;
```
