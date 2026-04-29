# integration/fixtures/jsx-component/input.tsx

## Input

```tsx
import { Fragment } from "react";

interface Props {
  label: string;
}

const Hello = (props: Props) => (
  <Fragment>
    <span className="greeting">{props.label}</span>
  </Fragment>
);

const App = () => <Hello label="hi" />;

const root = App;
```

## Mermaid

```mermaid
flowchart RL
  n_scope_0_Fragment_9["import Fragment<br/>L1"]
  n_scope_0_root_237["root<br/>L15"]
  n_scope_0_span_125["global span<br/>L9"]
  subgraph wrap_s_scope_1[" "]
    direction TB
    n_scope_0_Hello_79["Hello()<br/>L7"]
    subgraph s_scope_1["Hello()<br/>L7"]
      direction RL
      return_scope_0_Hello_79((return))
      n_scope_1_props_88["props<br/>L7"]
    end
  end
  subgraph wrap_s_scope_2[" "]
    direction TB
    n_scope_0_App_196["App()<br/>L13"]
    subgraph s_scope_2["App()<br/>L13"]
      direction RL
      return_scope_0_App_196((return))
    end
  end
  n_scope_0_Fragment_9 -->|read| return_scope_0_Hello_79
  n_scope_0_span_125 -->|read| return_scope_0_Hello_79
  n_scope_1_props_88 -->|read| return_scope_0_Hello_79
  n_scope_0_Hello_79 -->|read| return_scope_0_App_196
  n_scope_0_App_196 -->|read| n_scope_0_root_237
  mod_react["module react<br/>L1"]
  mod_react -->|read| n_scope_0_Fragment_9
  classDef fnWrap fill:#1a2030,stroke:#5a7d99;
  class wrap_s_scope_1 fnWrap;
  class wrap_s_scope_2 fnWrap;
  classDef unused stroke-dasharray: 5 5;
  class n_scope_0_root_237 unused;
```
