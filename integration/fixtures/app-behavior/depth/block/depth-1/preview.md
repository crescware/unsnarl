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
  n_scope_0_console_185["global console"]
  subgraph s_scope_1["block L1-19"]
    direction RL
    n_scope_1_v1_10["v1<br/>L2"]
    beyond_depth_s_scope_1((...))
  end
  n_scope_1_v1_10 -.->|read| beyond_depth_s_scope_1
  n_scope_0_console_185 -.->|read| beyond_depth_s_scope_1
  classDef boundaryStub fill:transparent,stroke:#888,stroke-dasharray:3 3,color:#888;
  class beyond_depth_s_scope_1 boundaryStub;
```
