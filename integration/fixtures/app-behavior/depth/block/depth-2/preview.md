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
  n_scope_1_v1_10["v1<br/>L2"]
  n_scope_2_v2_32["v2<br/>L4"]
  collapsed_scope_3["[hidden]<br/>L5-17"]
  n_scope_1_v1_10 -->|read| n_scope_2_v2_32
```
