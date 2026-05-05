# integration/fixtures/return-subgraph-misclassification/input.ts

## Input

```ts
const arr = [1];
function f() {
  for (const x of arr) {
    console.log(x);
  }
}
```

## Mermaid

```mermaid
%%{init: {"flowchart": {"defaultRenderer": "elk"}}}%%
flowchart RL
  n_scope_0_arr_6["arr<br/>L1"]
  n_scope_0_console_61["global console"]
  subgraph wrap_s_scope_1[" "]
    direction TB
    n_scope_0_f_26["unused f()<br/>L2"]
    subgraph s_scope_1["f()<br/>L2-6"]
      direction RL
      subgraph s_scope_2["for L3-5"]
        direction RL
        n_scope_2_x_45["x<br/>L3"]
        subgraph s_return_scope_0_f_26_implicit["return L3"]
          direction RL
          ret_use_ref_0["arr<br/>L3"]
          ret_use_ref_1["console<br/>L4"]
          ret_use_ref_2["x<br/>L4"]
        end
      end
    end
  end
  n_scope_0_arr_6 -->|read| ret_use_ref_0
  n_scope_0_console_61 -->|read| ret_use_ref_1
  n_scope_2_x_45 -->|read| ret_use_ref_2
  classDef fnWrap fill:#1a2030,stroke:#5a7d99;
  class wrap_s_scope_1 fnWrap;
```
