# integration/fixtures/jsx-nested-five/input.tsx

## Input

```tsx
import { A, B, C, D, E } from "components";

function Main() {
  const v = "v";
  const w = "w";
  const x = "x";
  const y = "y";
  const z = "z";

  return (
    <A>
      {v}
      <B>
        {w}
        <C>
          {x}
          <D>
            {y}
            <E>{z}</E>
          </D>
        </C>
      </B>
    </A>
  );
}
```

## Query

```sh
-r 10-12 -A 0 -B 1
```

## Mermaid

```mermaid
%%{init: {"flowchart": {"defaultRenderer": "elk"}}}%%
flowchart RL
  %% pruning roots 10-12=2 ancestors=1 descendants=0
  n_scope_0_A_9["import A<br/>L1"]
  subgraph s_scope_1["Main()<br/>L3-25"]
    direction RL
    n_scope_1_v_71["v<br/>L4"]
    subgraph s_return_scope_0_Main_54_151_331["return L10-24"]
      direction RL
      ret_use_ref_0["&lt;A&gt;<br/>L11-23"]
      ret_use_ref_1["v<br/>L12"]
    end
  end
  n_scope_0_A_9 -->|read| ret_use_ref_0
  n_scope_1_v_71 -->|read| ret_use_ref_1
  boundary_stub_1((...))
  boundary_stub_1 -.->|read| n_scope_0_A_9
  classDef boundaryStub fill:transparent,stroke:#888,stroke-dasharray:3 3,color:#888;
  class boundary_stub_1 boundaryStub;
```
