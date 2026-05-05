# integration/fixtures/for/classic-for/labeled-break-continue/input.ts

## Input

```ts
outer_loop: for (let r = 0; r < 3; r++) {
  for (let s = 0; s < 3; s++) {
    if (s === 1) continue outer_loop;
    if (r === 2) break outer_loop;
    console.log(r, s);
  }
}
```

## Mermaid

```mermaid
%%{init: {"flowchart": {"defaultRenderer": "elk"}}}%%
flowchart RL
  n_scope_0_console_151["global console"]
  subgraph s_scope_1["for L1-7"]
    direction RL
    for_test_scope_0_12["for ()<br/>L1"]
    n_scope_1_r_21["let r<br/>L1"]
    wr_ref_1(["let r<br/>L1"])
    subgraph s_scope_3["for L2-6"]
      direction RL
      for_test_scope_2_44["for ()<br/>L2"]
      n_scope_3_s_53["let s<br/>L2"]
      wr_ref_3(["let s<br/>L2"])
      expr_stmt_151["console.log()<br/>L5"]
    end
  end
  n_scope_1_r_21 -->|set| wr_ref_1
  n_scope_3_s_53 -->|set| wr_ref_3
  n_scope_1_r_21 -->|read| for_test_scope_0_12
  n_scope_3_s_53 -->|read| for_test_scope_2_44
  wr_ref_3 -->|read| module_root
  wr_ref_1 -->|read| module_root
  n_scope_0_console_151 -->|read| expr_stmt_151
  wr_ref_1 -->|read| expr_stmt_151
  wr_ref_3 -->|read| expr_stmt_151
  module_root((module))
```
