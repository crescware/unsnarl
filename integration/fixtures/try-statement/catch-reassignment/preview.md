# integration/fixtures/try-statement/catch-reassignment/input.ts

## Input

```ts
function g(): unknown {
  try {
    throw new Error("oops");
  } catch (e) {
    e = "rewritten";
    return e;
  }
}
```

## Mermaid

```mermaid
%%{init: {"flowchart": {"defaultRenderer": "elk"}}}%%
flowchart RL
  n_scope_0_Error_46["global Error"]
  subgraph wrap_s_scope_1[" "]
    direction TB
    n_scope_0_g_9["unused g()<br/>L1"]
    subgraph s_scope_1["g()<br/>L1-8"]
      direction RL
      subgraph s_scope_2["try L2-4"]
        direction RL
      end
      subgraph s_scope_3["catch L4-7"]
        direction RL
        n_scope_3_e_72["catch e<br/>L4"]
        wr_ref_1(["e<br/>L5"])
        subgraph s_return_scope_0_g_9_102_111["return L6"]
          direction RL
          ret_use_ref_2["e<br/>L6"]
        end
      end
    end
  end
  n_scope_3_e_72 -->|set| wr_ref_1
  n_scope_0_Error_46 -->|read,call| module_root
  wr_ref_1 -->|read| ret_use_ref_2
  module_root((module))
  classDef fnWrap fill:#1a2030,stroke:#5a7d99;
  class wrap_s_scope_1 fnWrap;
  classDef nestL1 fill:#1e2738,stroke:#3d4a63;
  class s_scope_1 nestL1;
  classDef nestL2 fill:#233045,stroke:#475670;
  class s_scope_2 nestL2;
  class s_scope_3 nestL2;
  classDef nestL3 fill:#283952,stroke:#51637d;
  class s_return_scope_0_g_9_102_111 nestL3;
```
