# integration/fixtures/jsx/children-expression/input.tsx

## Input

```tsx
import { Sub } from "./sub";

export function Main() {
  const message = "hello";
  return <Sub>{message}</Sub>;
}
```

## Mermaid

```mermaid
%%{init: {"flowchart": {"defaultRenderer": "elk"}}}%%
flowchart RL
  subgraph wrap_s_scope_1[" "]
    direction TB
    n_scope_0_Main_46["unused Main()<br/>L3"]
    subgraph s_scope_1["Main()<br/>L3-6"]
      direction RL
      n_scope_1_message_63["message<br/>L4"]
      subgraph s_return_scope_0_Main_46_84_112["return L5"]
        direction RL
        ret_use_ref_1["&lt;Sub&gt;<br/>L5"]
        ret_use_ref_2["message<br/>L5"]
      end
    end
  end
  subgraph sg___sub["module ./sub"]
    direction RL
    n_scope_0_Sub_9["import Sub<br/>L1"]
  end
  n_scope_0_Sub_9 -->|read| ret_use_ref_1
  n_scope_1_message_63 -->|read| ret_use_ref_2
  classDef nestL1 fill:#11192a,stroke:transparent;
  class wrap_s_scope_1 nestL1;
  class sg___sub nestL1;
  classDef nestL2 fill:#1a2538,stroke:transparent;
  class s_scope_1 nestL2;
  classDef nestL3 fill:#243047,stroke:transparent;
  class s_return_scope_0_Main_46_84_112 nestL3;
```
