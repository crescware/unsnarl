# jsx-component

## Input (`input.tsx`)

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
  n_scope_0_Hello_79["Hello<br/>L7"]
  n_scope_0_App_196["App<br/>L13"]
  n_scope_0_root_237["root<br/>L15"]
  n_scope_0_span_125["(unresolved:span)"]
  n_scope_1_props_88["param props<br/>L7"]
  n_scope_1_props_88 -->|read| module_root
  n_scope_0_Fragment_9 -->|read| module_root
  n_scope_0_span_125 -->|read| module_root
  n_scope_1_props_88 -->|read| module_root
  n_scope_0_Hello_79 -->|read| module_root
  n_scope_0_App_196 -->|read| n_scope_0_root_237
  module_root((module))
  classDef unused stroke-dasharray: 5 5;
  class n_scope_0_root_237 unused;
```
