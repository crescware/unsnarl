# integration/fixtures/class/expression/computed-key-in-return/input.ts

## Input

```ts
function makeClass(key: string) {
  return class {
    [key] = 0;
  };
}

const C = makeClass("x");
```

## Mermaid

```mermaid
%%{init: {"flowchart": {"defaultRenderer": "elk"}}}%%
flowchart RL
  n_scope_0_C_80["unused C<br/>L7"]
  subgraph wrap_s_scope_1[" "]
    direction TB
    n_scope_0_makeClass_9["makeClass()<br/>L1"]
    subgraph s_scope_1["makeClass()<br/>L1-5"]
      direction RL
      n_scope_1_key_19["key<br/>L1"]
      subgraph s_scope_2["class (anonymous)<br/>L2-4"]
        direction RL
        elk_empty_s_scope_2["No nodes"]
      end
    end
  end
  n_scope_1_key_19 -->|read| module_root
  n_scope_0_makeClass_9 -->|read,call| n_scope_0_C_80
  module_root((module))
  classDef nestL1 fill:#11192a,stroke:transparent;
  class wrap_s_scope_1 nestL1;
  classDef nestL2 fill:#1a2538,stroke:transparent;
  class s_scope_1 nestL2;
  classDef nestL3 fill:#243047,stroke:transparent;
  class s_scope_2 nestL3;
  classDef elkEmptyPlaceholder fill:transparent,stroke:transparent;
  class elk_empty_s_scope_2 elkEmptyPlaceholder;
```
