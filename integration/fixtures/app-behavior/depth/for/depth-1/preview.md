# integration/fixtures/app-behavior/depth/for/input.ts

## Input

```ts
for (let i1 = 0; i1 < 1; i1++) {
  for (let i2 = 0; i2 < 1; i2++) {
    for (let i3 = 0; i3 < 1; i3++) {
      for (let i4 = 0; i4 < 1; i4++) {
        for (let i5 = 0; i5 < 1; i5++) {
          for (let i6 = 0; i6 < 1; i6++) {
            console.log(i1, i2, i3, i4, i5, i6);
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
  n_scope_0_console_240["global console"]
  subgraph s_scope_1["for L1-13"]
    direction RL
    for_test_scope_0_0["for ()<br/>L1"]
    n_scope_1_i1_9["let i1<br/>L1"]
    wr_ref_2(["let i1<br/>L1"])
    subgraph s_scope_3["for L2-12"]
      direction RL
      for_test_scope_2_35["for ()<br/>L2"]
      n_scope_3_i2_44["let i2<br/>L2"]
      wr_ref_5(["let i2<br/>L2"])
      beyond_depth_s_scope_3((...))
    end
  end
  n_scope_1_i1_9 -->|set| wr_ref_2
  n_scope_3_i2_44 -->|set| wr_ref_5
  n_scope_1_i1_9 -->|read| for_test_scope_0_0
  n_scope_3_i2_44 -->|read| for_test_scope_2_35
  n_scope_0_console_240 -.->|read| beyond_depth_s_scope_3
  wr_ref_2 -.->|read| beyond_depth_s_scope_3
  wr_ref_5 -.->|read| beyond_depth_s_scope_3
  classDef boundaryStub fill:transparent,stroke:#888,stroke-dasharray:3 3,color:#888;
  class beyond_depth_s_scope_3 boundaryStub;
```
