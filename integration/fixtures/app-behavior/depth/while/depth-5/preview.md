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
--depth 5
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
      subgraph s_scope_3["while L12-23"]
        direction RL
        while_test_scope_2_131["while ()<br/>L12"]
        wr_ref_11(["let n3<br/>L13"])
        subgraph s_scope_4["while L14-22"]
          direction RL
          while_test_scope_3_166["while ()<br/>L14"]
          wr_ref_13(["let n4<br/>L15"])
          subgraph s_scope_5["while L16-21"]
            direction RL
            while_test_scope_4_205["while ()<br/>L16"]
            wr_ref_15(["let n5<br/>L17"])
            beyond_depth_s_scope_5((...))
          end
        end
      end
    end
  end
  n_scope_0_n1_4 -->|set| wr_ref_7
  n_scope_0_n2_16 -->|set| wr_ref_9
  n_scope_0_n3_28 -->|set| wr_ref_11
  n_scope_0_n4_40 -->|set| wr_ref_13
  n_scope_0_n5_52 -->|set| wr_ref_15
  n_scope_0_n6_64 -.->|set| beyond_depth_s_scope_5
  n_scope_0_n1_4 -->|read| while_test_scope_0_73
  n_scope_0_n2_16 -->|read| while_test_scope_1_100
  n_scope_0_n3_28 -->|read| while_test_scope_2_131
  n_scope_0_n4_40 -->|read| while_test_scope_3_166
  n_scope_0_n5_52 -->|read| while_test_scope_4_205
  n_scope_0_n6_64 -.->|read| beyond_depth_s_scope_5
  classDef boundaryStub fill:transparent,stroke:#888,stroke-dasharray:3 3,color:#888;
  class beyond_depth_s_scope_5 boundaryStub;
  classDef nestL1 fill:#1e2738,stroke:#3d4a63;
  class s_scope_1 nestL1;
  class s_scope_5 nestL1;
  classDef nestL2 fill:#233045,stroke:#475670;
  class s_scope_2 nestL2;
  classDef nestL3 fill:#283952,stroke:#51637d;
  class s_scope_3 nestL3;
  classDef nestL4 fill:#2d425f,stroke:#5b708a;
  class s_scope_4 nestL4;
```
