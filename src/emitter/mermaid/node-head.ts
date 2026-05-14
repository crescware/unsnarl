import { VARIABLE_DECLARATION_KIND } from "../../serializer/variable-declaration-kind.js";
import { NODE_KIND } from "../../visual-graph/node-kind.js";
import type { VisualNode } from "../../visual-graph/visual-node.js";
import { escape } from "./escape.js";

export function nodeHead(n: VisualNode): string {
  const name = escape(n.name);
  if (n.isJsxElement) {
    // Mermaid `["..."]` labels require HTML-escaped angle brackets so the
    // parser does not mistake them for syntax; the renderer surfaces them
    // as literal `<` / `>` in the output.
    return `&lt;${name}&gt;`;
  }
  switch (n.kind) {
    case NODE_KIND.FunctionDeclaration:
      return `${name}()`;
    case NODE_KIND.ClassDeclaration:
      return `class ${name}`;
    case NODE_KIND.NamedImportBinding:
      return n.importedName !== n.name ? name : `import ${name}`;
    case NODE_KIND.DefaultImportBinding:
    case NODE_KIND.NamespaceImportBinding:
      return `import ${name}`;
    case NODE_KIND.CatchParameter:
      return `catch ${name}`;
    case NODE_KIND.SyntheticImplicitGlobal:
      return `global ${name}`;
    case NODE_KIND.WriteReference:
      return n.declarationKind === VARIABLE_DECLARATION_KIND.Let
        ? `let ${name}`
        : name;
    case NODE_KIND.SyntheticModuleSource:
      return `module ${name}`;
    case NODE_KIND.SyntheticImportIntermediate:
      return `import ${name}`;
    case NODE_KIND.VarBinding:
      return n.initIsFunction ? `${name}()` : `var ${name}`;
    case NODE_KIND.ConstBinding:
      return n.initIsFunction ? `${name}()` : name;
    case NODE_KIND.LetBinding:
      return n.initIsFunction ? `${name}()` : `let ${name}`;
    case NODE_KIND.FormalParameter:
    case NODE_KIND.ReturnArgumentReference:
    case NODE_KIND.SyntheticIfStatementTest:
    case NODE_KIND.SyntheticSwitchStatementDiscriminant:
    case NODE_KIND.SyntheticWhileStatementTest:
    case NODE_KIND.SyntheticDoWhileStatementTest:
    case NODE_KIND.SyntheticForStatementHeader:
    case NODE_KIND.SyntheticForInStatementHeader:
    case NODE_KIND.SyntheticForOfStatementHeader:
    case NODE_KIND.SyntheticModuleSink:
    case NODE_KIND.SyntheticExpressionStatement:
      return name;
    case NODE_KIND.SyntheticBeyondDepth:
      // ASCII "..." rather than U+2026; some Mermaid renderers stumble on
      // multibyte glyphs inside a node-shape label.
      return "...";
  }
}
