# integration/fixtures/class/expression/instance-field-in-return/input.ts

## Input

```ts
function makeClass(seed: number) {
  return class {
    x = seed;
  };
}

const C = makeClass(0);
const c = new C();
```

## Mermaid

```mermaid
%%{init: {"flowchart": {"defaultRenderer": "elk"}}}%%
flowchart RL
  n_scope_0_C_80["C<br/>L7"]
  n_scope_0_c_104["unused c<br/>L8"]
  subgraph wrap_s_scope_1[" "]
    direction TB
    n_scope_0_makeClass_9["makeClass()<br/>L1"]
    subgraph s_scope_1["makeClass()<br/>L1-5"]
      direction RL
      n_scope_1_seed_19["seed<br/>L1"]
      subgraph s_scope_2["class (anonymous)<br/>L2-4"]
        direction RL
      end
    end
  end
  n_scope_1_seed_19 -->|read| module_root
  n_scope_0_makeClass_9 -->|read,call| n_scope_0_C_80
  n_scope_0_C_80 -->|read,call| n_scope_0_c_104
  module_root((module))
  classDef nestL1 fill:#11192a,stroke:transparent;
  class wrap_s_scope_1 nestL1;
  classDef nestL2 fill:#1a2538,stroke:transparent;
  class s_scope_1 nestL2;
  classDef nestL3 fill:#243047,stroke:transparent;
  class s_scope_2 nestL3;
```
