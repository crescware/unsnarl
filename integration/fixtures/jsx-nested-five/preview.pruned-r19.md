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
-r 19 -C 10
```

## Mermaid

```mermaid
%%{init: {"flowchart": {"defaultRenderer": "elk"}}}%%
flowchart RL
  %% pruning roots 19=2 ancestors=10 descendants=10
  n_scope_0_E_21["import E<br/>L1"]
  subgraph s_scope_1["Main()<br/>L3-25"]
    direction RL
    n_scope_1_z_139["z<br/>L8"]
    subgraph s_return_scope_0_Main_54_151_331["return L10-24"]
      direction RL
      ret_use_ref_8["E<br/>L19"]
      ret_use_ref_9["z<br/>L19"]
    end
  end
  n_scope_0_E_21 -->|read| ret_use_ref_8
  n_scope_1_z_139 -->|read| ret_use_ref_9
  mod_components["module components<br/>L1"]
  mod_components -->|read| n_scope_0_E_21
  boundary_stub_1((...))
  mod_components -.-> boundary_stub_1
  classDef boundaryStub fill:transparent,stroke:#888,stroke-dasharray:3 3,color:#888;
  class boundary_stub_1 boundaryStub;
```
