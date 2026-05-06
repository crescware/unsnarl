# integration/fixtures/declaration/var/binding-patterns/input.ts

## Notice

```
uns: warning: L2:0: var declaration detected; rendered as node only (no edges).
uns: warning: L5:0: var declaration detected; rendered as node only (no edges).
uns: warning: L9:0: var declaration detected; rendered as node only (no edges).
uns: warning: L12:0: var declaration detected; rendered as node only (no edges).
uns: warning: L15:0: var declaration detected; rendered as node only (no edges).
uns: warning: L18:0: var declaration detected; rendered as node only (no edges).
uns: warning: L21:0: var declaration detected; rendered as node only (no edges).
```

## Input

```ts
// basic
var a = 0;

// multiple declarators
var b = 1,
  c = 2;

// array pattern + default + hole + rest
var [d = 100, , e, ...f] = [3, , 5, 6, 7];

// object pattern + rename + default + rest
var { g, h: renamed = 200, ...others } = { g: 8, h: 9, x: 10, y: 11 };

// non-identifier property name
var { "kebab-case": kebab, 0: zeroth } = { "kebab-case": 13, 0: 14 };

// nested (array inside object) + default
var { nested: [p = 0, q] = [] } = { nested: [15, 16] };

// nested (object inside array)
var [{ r, s = 0 }, [t, u]] = [{ r: 1, s: 2 }, [3, 4]];

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
  n_scope_0_a_13["a<br/>L2"]
  n_scope_0_b_49["b<br/>L5"]
  n_scope_0_c_58["c<br/>L6"]
  n_scope_0_d_112["d<br/>L9"]
  n_scope_0_e_123["e<br/>L9"]
  n_scope_0_f_129["f<br/>L9"]
  n_scope_0_g_201["g<br/>L12"]
  n_scope_0_renamed_207["renamed<br/>L12"]
  n_scope_0_others_225["others<br/>L12"]
  n_scope_0_kebab_319["kebab<br/>L15"]
  n_scope_0_zeroth_329["zeroth<br/>L15"]
  n_scope_0_p_427["p<br/>L18"]
  n_scope_0_q_434["q<br/>L18"]
  n_scope_0_r_508["r<br/>L21"]
  n_scope_0_s_511["s<br/>L21"]
  n_scope_0_t_521["t<br/>L21"]
  n_scope_0_u_524["u<br/>L21"]
  n_scope_0_console_557["global console"]
  n_scope_0_console_557 -->|read| expr_stmt_557
  expr_stmt_557["console.log()<br/>L23-41"]
```
