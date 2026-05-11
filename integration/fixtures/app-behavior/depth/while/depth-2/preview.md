# integration/fixtures/app-behavior/depth/while/input.ts

## Input

```ts
let n1 = 1;
let n2 = 1;
let n3 = 1;
let n4 = 1;
let n5 = 1;
let n6 = 1;

while (n1 > 0) {
  n1--;
  while (n2 > 0) {
    n2--;
    while (n3 > 0) {
      n3--;
      while (n4 > 0) {
        n4--;
        while (n5 > 0) {
          n5--;
          while (n6 > 0) {
            n6--;
          }
        }
      }
    }
  }
}
```

## Query

```sh
--depth 2
```

## Mermaid

```mermaid
%%{init: {"flowchart": {"defaultRenderer": "elk"}}}%%
flowchart RL
  n_scope_0_n1_4["let n1<br/>L1"]
  n_scope_0_n2_16["let n2<br/>L2"]
  n_scope_0_n3_28["let n3<br/>L3"]
  n_scope_0_n4_40["let n4<br/>L4"]
  n_scope_0_n5_52["let n5<br/>L5"]
  n_scope_0_n6_64["let n6<br/>L6"]
  subgraph s_scope_1["while L8-25"]
    direction RL
    while_test_scope_0_73["while ()<br/>L8"]
    wr_ref_7(["let n1<br/>L9"])
    subgraph s_scope_2["while L10-24"]
      direction RL
      while_test_scope_1_100["while ()<br/>L10"]
      wr_ref_9(["let n2<br/>L11"])
      beyond_depth_s_scope_2((...))
    end
  end
  n_scope_0_n1_4 -->|set| wr_ref_7
  n_scope_0_n2_16 -->|set| wr_ref_9
  n_scope_0_n3_28 -.->|set| beyond_depth_s_scope_2
  n_scope_0_n4_40 -.->|set| beyond_depth_s_scope_2
  n_scope_0_n5_52 -.->|set| beyond_depth_s_scope_2
  n_scope_0_n6_64 -.->|set| beyond_depth_s_scope_2
  n_scope_0_n1_4 -->|read| while_test_scope_0_73
  n_scope_0_n2_16 -->|read| while_test_scope_1_100
  n_scope_0_n3_28 -.->|read| beyond_depth_s_scope_2
  n_scope_0_n4_40 -.->|read| beyond_depth_s_scope_2
  n_scope_0_n5_52 -.->|read| beyond_depth_s_scope_2
  n_scope_0_n6_64 -.->|read| beyond_depth_s_scope_2
  classDef boundaryStub fill:transparent,stroke:#888,stroke-dasharray:3 3,color:#888;
  class beyond_depth_s_scope_2 boundaryStub;
  classDef nestL1 fill:#1e2738,stroke:#3d4a63;
  class s_scope_1 nestL1;
  classDef nestL2 fill:#233045,stroke:#475670;
  class s_scope_2 nestL2;
```
