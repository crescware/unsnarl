# integration/fixtures/jsx/nested-five/input.tsx

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
-r 19 -A 1 -B 0
```

## Mermaid

```mermaid
%%{init: {"flowchart": {"defaultRenderer": "elk"}}}%%
flowchart RL
  %% pruning roots 19=6 ancestors=0 descendants=1
  subgraph s_scope_1["Main()<br/>L3-25"]
    direction RL
    subgraph s_return_scope_0_Main_54_151_331["return L10-24"]
      direction RL
      ret_use_ref_5["&lt;A&gt;<br/>L11-23"]
      ret_use_ref_7["&lt;B&gt;<br/>L13-22"]
      ret_use_ref_9["&lt;C&gt;<br/>L15-21"]
      ret_use_ref_11["&lt;D&gt;<br/>L17-20"]
      ret_use_ref_13["&lt;E&gt;<br/>L19"]
      ret_use_ref_14["z<br/>L19"]
    end
  end
  classDef nestL1 fill:#11192a,stroke:transparent;
  class s_scope_1 nestL1;
  classDef nestL2 fill:#1a2538,stroke:transparent;
  class s_return_scope_0_Main_54_151_331 nestL2;
```
