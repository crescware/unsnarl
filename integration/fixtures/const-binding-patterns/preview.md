# integration/fixtures/const-binding-patterns/input.ts

## Input

```ts
// basic
const a = 0;

// multiple declarators
const b = 1,
  c = 2;

// array pattern + default + hole + rest
const [d = 100, , e, ...f] = [3, , 5, 6, 7];

// object pattern + rename + default + rest
const { g, h: renamed = 200, ...others } = { g: 8, h: 9, x: 10, y: 11 };

// non-identifier property name
const { "kebab-case": kebab, 0: zeroth } = { "kebab-case": 13, 0: 14 };

// nested (array inside object) + default
const { nested: [p = 0, q] = [] } = { nested: [15, 16] };

// nested (object inside array)
const [{ r, s = 0 }, [t, u]] = [{ r: 1, s: 2 }, [3, 4]];

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
  n_scope_0_a_15["a<br/>L2"]
  n_scope_0_b_53["b<br/>L5"]
  n_scope_0_c_62["c<br/>L6"]
  n_scope_0_d_118["d<br/>L9"]
  n_scope_0_e_129["e<br/>L9"]
  n_scope_0_f_135["f<br/>L9"]
  n_scope_0_g_209["g<br/>L12"]
  n_scope_0_renamed_215["renamed<br/>L12"]
  n_scope_0_others_233["others<br/>L12"]
  n_scope_0_kebab_329["kebab<br/>L15"]
  n_scope_0_zeroth_339["zeroth<br/>L15"]
  n_scope_0_p_439["p<br/>L18"]
  n_scope_0_q_446["q<br/>L18"]
  n_scope_0_r_522["r<br/>L21"]
  n_scope_0_s_525["s<br/>L21"]
  n_scope_0_t_535["t<br/>L21"]
  n_scope_0_u_538["u<br/>L21"]
  n_scope_0_console_571["global console"]
  n_scope_0_console_571 -->|read| expr_stmt_571
  n_scope_0_a_15 -->|read| expr_stmt_571
  n_scope_0_b_53 -->|read| expr_stmt_571
  n_scope_0_c_62 -->|read| expr_stmt_571
  n_scope_0_d_118 -->|read| expr_stmt_571
  n_scope_0_e_129 -->|read| expr_stmt_571
  n_scope_0_f_135 -->|read| expr_stmt_571
  n_scope_0_g_209 -->|read| expr_stmt_571
  n_scope_0_renamed_215 -->|read| expr_stmt_571
  n_scope_0_others_233 -->|read| expr_stmt_571
  n_scope_0_kebab_329 -->|read| expr_stmt_571
  n_scope_0_zeroth_339 -->|read| expr_stmt_571
  n_scope_0_p_439 -->|read| expr_stmt_571
  n_scope_0_q_446 -->|read| expr_stmt_571
  n_scope_0_r_522 -->|read| expr_stmt_571
  n_scope_0_s_525 -->|read| expr_stmt_571
  n_scope_0_t_535 -->|read| expr_stmt_571
  n_scope_0_u_538 -->|read| expr_stmt_571
  expr_stmt_571["console.log()<br/>L23-41"]
```
