# integration/fixtures/let-binding-patterns/input.ts

## Input

```ts
// basic
let a = 0;

// multiple declarators
let b = 1,
  c = 2;

// array pattern + default + hole + rest
let [d = 100, , e, ...f] = [3, , 5, 6, 7];

// object pattern + rename + default + rest
let { g, h: renamed = 200, ...others } = { g: 8, h: 9, x: 10, y: 11 };

// non-identifier property name
let { "kebab-case": kebab, 0: zeroth } = { "kebab-case": 13, 0: 14 };

// nested (array inside object) + default
let { nested: [p = 0, q] = [] } = { nested: [15, 16] };

// nested (object inside array)
let [{ r, s = 0 }, [t, u]] = [{ r: 1, s: 2 }, [3, 4]];

console.log(
  a,
  b,
  c,
  d,
  e,
  f,
  g,
  renamed,
  others,
  kebab,
  zeroth,
  p,
  q,
  r,
  s,
  t,
  u,
);
```

## Mermaid

```mermaid
%%{init: {"flowchart": {"defaultRenderer": "elk"}}}%%
flowchart RL
  n_scope_0_a_13["let a<br/>L2"]
  n_scope_0_b_49["let b<br/>L5"]
  n_scope_0_c_58["let c<br/>L6"]
  n_scope_0_d_112["let d<br/>L9"]
  n_scope_0_e_123["let e<br/>L9"]
  n_scope_0_f_129["let f<br/>L9"]
  n_scope_0_g_201["let g<br/>L12"]
  n_scope_0_renamed_207["let renamed<br/>L12"]
  n_scope_0_others_225["let others<br/>L12"]
  n_scope_0_kebab_319["let kebab<br/>L15"]
  n_scope_0_zeroth_329["let zeroth<br/>L15"]
  n_scope_0_p_427["let p<br/>L18"]
  n_scope_0_q_434["let q<br/>L18"]
  n_scope_0_r_508["let r<br/>L21"]
  n_scope_0_s_511["let s<br/>L21"]
  n_scope_0_t_521["let t<br/>L21"]
  n_scope_0_u_524["let u<br/>L21"]
  n_scope_0_console_557["global console"]
  n_scope_0_console_557 -->|read| expr_stmt_557
  n_scope_0_a_13 -->|read| expr_stmt_557
  n_scope_0_b_49 -->|read| expr_stmt_557
  n_scope_0_c_58 -->|read| expr_stmt_557
  n_scope_0_d_112 -->|read| expr_stmt_557
  n_scope_0_e_123 -->|read| expr_stmt_557
  n_scope_0_f_129 -->|read| expr_stmt_557
  n_scope_0_g_201 -->|read| expr_stmt_557
  n_scope_0_renamed_207 -->|read| expr_stmt_557
  n_scope_0_others_225 -->|read| expr_stmt_557
  n_scope_0_kebab_319 -->|read| expr_stmt_557
  n_scope_0_zeroth_329 -->|read| expr_stmt_557
  n_scope_0_p_427 -->|read| expr_stmt_557
  n_scope_0_q_434 -->|read| expr_stmt_557
  n_scope_0_r_508 -->|read| expr_stmt_557
  n_scope_0_s_511 -->|read| expr_stmt_557
  n_scope_0_t_521 -->|read| expr_stmt_557
  n_scope_0_u_524 -->|read| expr_stmt_557
  expr_stmt_557["console.log()<br/>L23-41"]
```
