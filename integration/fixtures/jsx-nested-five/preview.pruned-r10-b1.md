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
-r 10 -A 0 -B 1
```

## Mermaid

```mermaid
%%{init: {"flowchart": {"defaultRenderer": "elk"}}}%%
flowchart RL
  %% pruning roots 10=10 ancestors=1 descendants=0
  n_scope_0_A_9["import A<br/>L1"]
  n_scope_0_B_12["import B<br/>L1"]
  n_scope_0_C_15["import C<br/>L1"]
  n_scope_0_D_18["import D<br/>L1"]
  n_scope_0_E_21["import E<br/>L1"]
  subgraph s_scope_1["Main()<br/>L3-25"]
    direction RL
    n_scope_1_v_71["v<br/>L4"]
    n_scope_1_w_88["w<br/>L5"]
    n_scope_1_x_105["x<br/>L6"]
    n_scope_1_y_122["y<br/>L7"]
    n_scope_1_z_139["z<br/>L8"]
    subgraph s_return_scope_0_Main_54_151_331["return L10-24"]
      direction RL
      ret_use_ref_0["&lt;A&gt;<br/>L11-23"]
      ret_use_ref_1["v<br/>L12"]
      ret_use_ref_2["&lt;B&gt;<br/>L13-22"]
      ret_use_ref_3["w<br/>L14"]
      ret_use_ref_4["&lt;C&gt;<br/>L15-21"]
      ret_use_ref_5["x<br/>L16"]
      ret_use_ref_6["&lt;D&gt;<br/>L17-20"]
      ret_use_ref_7["y<br/>L18"]
      ret_use_ref_8["&lt;E&gt;<br/>L19"]
      ret_use_ref_9["z<br/>L19"]
    end
  end
  n_scope_0_A_9 -->|read| ret_use_ref_0
  n_scope_1_v_71 -->|read| ret_use_ref_1
  n_scope_0_B_12 -->|read| ret_use_ref_2
  n_scope_1_w_88 -->|read| ret_use_ref_3
  n_scope_0_C_15 -->|read| ret_use_ref_4
  n_scope_1_x_105 -->|read| ret_use_ref_5
  n_scope_0_D_18 -->|read| ret_use_ref_6
  n_scope_1_y_122 -->|read| ret_use_ref_7
  n_scope_0_E_21 -->|read| ret_use_ref_8
  n_scope_1_z_139 -->|read| ret_use_ref_9
  boundary_stub_1((...))
  boundary_stub_1 -.->|read| n_scope_0_A_9
  boundary_stub_2((...))
  boundary_stub_2 -.->|read| n_scope_0_B_12
  boundary_stub_3((...))
  boundary_stub_3 -.->|read| n_scope_0_C_15
  boundary_stub_4((...))
  boundary_stub_4 -.->|read| n_scope_0_D_18
  boundary_stub_5((...))
  boundary_stub_5 -.->|read| n_scope_0_E_21
  classDef boundaryStub fill:transparent,stroke:#888,stroke-dasharray:3 3,color:#888;
  class boundary_stub_1 boundaryStub;
  class boundary_stub_2 boundaryStub;
  class boundary_stub_3 boundaryStub;
  class boundary_stub_4 boundaryStub;
  class boundary_stub_5 boundaryStub;
```
