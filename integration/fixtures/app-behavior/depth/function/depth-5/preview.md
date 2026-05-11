# integration/fixtures/app-behavior/depth/function/input.ts

## Input

```ts
function f1() {
  function f2() {
    function f3() {
      function f4() {
        function f5() {
          function f6() {
            const x = 1;
            return x;
          }
          return f6();
        }
        return f5();
      }
      return f4();
    }
    return f3();
  }
  return f2();
}

f1();
```

## Query

```sh
--depth 5
```

## Mermaid

```mermaid
%%{init: {"flowchart": {"defaultRenderer": "elk"}}}%%
flowchart RL
  subgraph wrap_s_scope_1[" "]
    direction TB
    n_scope_0_f1_9["f1()<br/>L1"]
    subgraph s_scope_1["f1()<br/>L1-19"]
      direction RL
      subgraph wrap_s_scope_2[" "]
        direction TB
        n_scope_1_f2_27["f2()<br/>L2"]
        subgraph s_scope_2["f2()<br/>L2-17"]
          direction RL
          subgraph wrap_s_scope_3[" "]
            direction TB
            n_scope_2_f3_47["f3()<br/>L3"]
            subgraph s_scope_3["f3()<br/>L3-15"]
              direction RL
              subgraph wrap_s_scope_4[" "]
                direction TB
                n_scope_3_f4_69["f4()<br/>L4"]
                subgraph s_scope_4["f4()<br/>L4-13"]
                  direction RL
                  subgraph wrap_s_scope_5[" "]
                    direction TB
                    n_scope_4_f5_93["f5()<br/>L5"]
                    subgraph s_scope_5["f5()<br/>L5-11"]
                      direction RL
                      n_scope_5_f6_119["f6()<br/>L6"]
                      subgraph s_return_scope_4_f5_93_195_207["return L10"]
                        direction RL
                        ret_use_ref_2["f6<br/>L10"]
                      end
                    end
                  end
                  subgraph s_return_scope_3_f4_69_226_238["return L12"]
                    direction RL
                    ret_use_ref_3["f5<br/>L12"]
                  end
                end
              end
              subgraph s_return_scope_2_f3_47_253_265["return L14"]
                direction RL
                ret_use_ref_4["f4<br/>L14"]
              end
            end
          end
          subgraph s_return_scope_1_f2_27_276_288["return L16"]
            direction RL
            ret_use_ref_5["f3<br/>L16"]
          end
        end
      end
      subgraph s_return_scope_0_f1_9_295_307["return L18"]
        direction RL
        ret_use_ref_6["f2<br/>L18"]
      end
    end
  end
  n_scope_5_f6_119 -->|read,call| ret_use_ref_2
  n_scope_4_f5_93 -->|read,call| ret_use_ref_3
  n_scope_3_f4_69 -->|read,call| ret_use_ref_4
  n_scope_2_f3_47 -->|read,call| ret_use_ref_5
  n_scope_1_f2_27 -->|read,call| ret_use_ref_6
  n_scope_0_f1_9 -->|read,call| expr_stmt_311
  expr_stmt_311["f1()<br/>L21"]
  classDef nestL1 fill:#11192a,stroke:transparent;
  class wrap_s_scope_1 nestL1;
  class wrap_s_scope_4 nestL1;
  class s_return_scope_2_f3_47_253_265 nestL1;
  classDef nestL2 fill:#1a2538,stroke:transparent;
  class s_scope_1 nestL2;
  class s_scope_4 nestL2;
  classDef nestL3 fill:#243047,stroke:transparent;
  class wrap_s_scope_2 nestL3;
  class wrap_s_scope_5 nestL3;
  class s_return_scope_3_f4_69_226_238 nestL3;
  class s_return_scope_0_f1_9_295_307 nestL3;
  classDef nestL4 fill:#2d3b57,stroke:transparent;
  class s_scope_2 nestL4;
  class s_scope_5 nestL4;
  classDef nestL5 fill:#364666,stroke:transparent;
  class wrap_s_scope_3 nestL5;
  class s_return_scope_4_f5_93_195_207 nestL5;
  class s_return_scope_1_f2_27_276_288 nestL5;
  classDef nestL6 fill:#3f5175,stroke:transparent;
  class s_scope_3 nestL6;
```
