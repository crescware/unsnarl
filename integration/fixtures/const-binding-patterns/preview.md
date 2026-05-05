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

// computed property name
const key = "dynamic";
const { [key]: dynamicValue } = { dynamic: 12 };

// computed property name (Symbol) + rename + default
const sym = Symbol("id");
const { [sym]: id = "default" } = { [sym]: "abc" };

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
  dynamicValue,
  id,
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
  n_scope_0_key_307["unused key<br/>L15"]
  n_scope_0_dynamicValue_339["dynamicValue<br/>L16"]
  n_scope_0_sym_434["sym<br/>L19"]
  n_scope_0_id_469["id<br/>L20"]
  n_scope_0_kebab_561["kebab<br/>L23"]
  n_scope_0_zeroth_571["zeroth<br/>L23"]
  n_scope_0_p_671["p<br/>L26"]
  n_scope_0_q_678["q<br/>L26"]
  n_scope_0_r_754["r<br/>L29"]
  n_scope_0_s_757["s<br/>L29"]
  n_scope_0_t_767["t<br/>L29"]
  n_scope_0_u_770["u<br/>L29"]
  n_scope_0_Symbol_440["global Symbol"]
  n_scope_0_console_803["global console"]
  n_scope_0_Symbol_440 -->|read,call| n_scope_0_sym_434
  n_scope_0_sym_434 -->|read| n_scope_0_id_469
  n_scope_0_console_803 -->|read| expr_stmt_803
  n_scope_0_a_15 -->|read| expr_stmt_803
  n_scope_0_b_53 -->|read| expr_stmt_803
  n_scope_0_c_62 -->|read| expr_stmt_803
  n_scope_0_d_118 -->|read| expr_stmt_803
  n_scope_0_e_129 -->|read| expr_stmt_803
  n_scope_0_f_135 -->|read| expr_stmt_803
  n_scope_0_g_209 -->|read| expr_stmt_803
  n_scope_0_renamed_215 -->|read| expr_stmt_803
  n_scope_0_others_233 -->|read| expr_stmt_803
  n_scope_0_dynamicValue_339 -->|read| expr_stmt_803
  n_scope_0_id_469 -->|read| expr_stmt_803
  n_scope_0_kebab_561 -->|read| expr_stmt_803
  n_scope_0_zeroth_571 -->|read| expr_stmt_803
  n_scope_0_p_671 -->|read| expr_stmt_803
  n_scope_0_q_678 -->|read| expr_stmt_803
  n_scope_0_r_754 -->|read| expr_stmt_803
  n_scope_0_s_757 -->|read| expr_stmt_803
  n_scope_0_t_767 -->|read| expr_stmt_803
  n_scope_0_u_770 -->|read| expr_stmt_803
  expr_stmt_803["console.log()<br/>L31-51"]
```
