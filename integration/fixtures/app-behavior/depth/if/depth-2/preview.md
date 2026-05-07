# integration/fixtures/app-behavior/depth/if/input.ts

## Input

```ts
const a = true;
const b = true;
const c = true;
const d = true;
const e = true;
const f = true;

if (a) {
  const v1 = 1;
  if (b) {
    const v2 = v1;
    if (c) {
      const v3 = v2;
      if (d) {
        const v4 = v3;
        if (e) {
          const v5 = v4;
          if (f) {
            const v6 = v5;
            console.log(v6);
          }
        }
      }
    }
  }
}
```

## Mermaid

```mermaid
%%{init: {"flowchart": {"defaultRenderer": "elk"}}}%%
flowchart RL
  n_scope_0_a_6["a<br/>L1"]
  n_scope_0_b_22["b<br/>L2"]
  n_scope_0_c_38["c<br/>L3"]
  n_scope_0_d_54["d<br/>L4"]
  n_scope_0_e_70["e<br/>L5"]
  n_scope_0_f_86["f<br/>L6"]
  n_scope_0_console_324["global console"]
  subgraph s_scope_1["if L8-26"]
    direction RL
    if_test_scope_0_97{"if ()<br/>L8"}
    n_scope_1_v1_114["v1<br/>L9"]
    subgraph s_scope_2["if L10-25"]
      direction RL
      if_test_scope_1_124{"if ()<br/>L10"}
      n_scope_2_v2_143["v2<br/>L11"]
      beyond_depth_s_scope_2((...))
    end
  end
  n_scope_0_a_6 -->|read| if_test_scope_0_97
  n_scope_0_b_22 -->|read| if_test_scope_1_124
  n_scope_1_v1_114 -->|read| n_scope_2_v2_143
  n_scope_0_c_38 -.->|read| beyond_depth_s_scope_2
  n_scope_2_v2_143 -.->|read| beyond_depth_s_scope_2
  n_scope_0_d_54 -.->|read| beyond_depth_s_scope_2
  n_scope_0_e_70 -.->|read| beyond_depth_s_scope_2
  n_scope_0_f_86 -.->|read| beyond_depth_s_scope_2
  n_scope_0_console_324 -.->|read| beyond_depth_s_scope_2
  classDef boundaryStub fill:transparent,stroke:#888,stroke-dasharray:3 3,color:#888;
  class beyond_depth_s_scope_2 boundaryStub;
```
