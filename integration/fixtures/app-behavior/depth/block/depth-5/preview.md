# integration/fixtures/app-behavior/depth/block/input.ts

## Input

```ts
{
  const v1 = 1;
  {
    const v2 = v1;
    {
      const v3 = v2;
      {
        const v4 = v3;
        {
          const v5 = v4;
          {
            const v6 = v5;
            console.log(v1, v2, v3, v4, v5, v6);
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
  n_scope_0_console_185["global console"]
  subgraph s_scope_1["block L1-19"]
    direction RL
    n_scope_1_v1_10["v1<br/>L2"]
    subgraph s_scope_2["block L3-18"]
      direction RL
      n_scope_2_v2_32["v2<br/>L4"]
      subgraph s_scope_3["block L5-17"]
        direction RL
        n_scope_3_v3_59["v3<br/>L6"]
        subgraph s_scope_4["block L7-16"]
          direction RL
          n_scope_4_v4_90["v4<br/>L8"]
          subgraph s_scope_5["block L9-15"]
            direction RL
            n_scope_5_v5_125["v5<br/>L10"]
            beyond_depth_s_scope_5((...))
          end
        end
      end
    end
  end
  n_scope_1_v1_10 -->|read| n_scope_2_v2_32
  n_scope_2_v2_32 -->|read| n_scope_3_v3_59
  n_scope_3_v3_59 -->|read| n_scope_4_v4_90
  n_scope_4_v4_90 -->|read| n_scope_5_v5_125
  n_scope_5_v5_125 -.->|read| beyond_depth_s_scope_5
  n_scope_0_console_185 -.->|read| beyond_depth_s_scope_5
  n_scope_1_v1_10 -.->|read| beyond_depth_s_scope_5
  n_scope_2_v2_32 -.->|read| beyond_depth_s_scope_5
  n_scope_3_v3_59 -.->|read| beyond_depth_s_scope_5
  n_scope_4_v4_90 -.->|read| beyond_depth_s_scope_5
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
