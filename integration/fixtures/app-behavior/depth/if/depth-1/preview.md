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
  end
  n_scope_0_a_6 -->|read| if_test_scope_0_97
  n_scope_0_b_22 -->|read| s_scope_1
  n_scope_1_v1_114 -->|read| s_scope_1
  n_scope_0_c_38 -->|read| s_scope_1
  n_scope_0_d_54 -->|read| s_scope_1
  n_scope_0_e_70 -->|read| s_scope_1
  n_scope_0_f_86 -->|read| s_scope_1
  n_scope_0_console_324 -->|read| s_scope_1
```
