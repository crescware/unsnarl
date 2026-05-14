import { IMPORT_KIND } from "../../serializer/import-kind.js";
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
    case NODE_KIND.LegacyFunctionName:
      return `${name}()`;
    case NODE_KIND.ClassDeclaration:
      return `class ${name}`;
    case NODE_KIND.LegacyImportBinding: {
      const isRenamedNamed =
        n.importKind === IMPORT_KIND.Named && n.importedName !== n.name;
      return isRenamedNamed ? name : `import ${name}`;
    }
    case NODE_KIND.LegacyCatchClause:
      return `catch ${name}`;
    case NODE_KIND.SyntheticImplicitGlobal:
      return `global ${name}`;
    case NODE_KIND.LegacyWriteOp:
      return n.declarationKind === VARIABLE_DECLARATION_KIND.Let
        ? `let ${name}`
        : name;
    case NODE_KIND.SyntheticModuleSource:
      return `module ${name}`;
    case NODE_KIND.SyntheticImportIntermediate:
      return `import ${name}`;
    case NODE_KIND.LegacyVariable:
      if (n.initIsFunction) {
        return `${name}()`;
      }
      if (n.declarationKind === VARIABLE_DECLARATION_KIND.Let) {
        return `let ${name}`;
      }
      if (n.declarationKind === VARIABLE_DECLARATION_KIND.Var) {
        return `var ${name}`;
      }
      return name;
    case NODE_KIND.LegacyParameter:
    case NODE_KIND.LegacyReturnUse:
    case NODE_KIND.SyntheticIfStatementTest:
    case NODE_KIND.SyntheticSwitchStatementDiscriminant:
    case NODE_KIND.SyntheticWhileStatementTest:
    case NODE_KIND.SyntheticDoWhileStatementTest:
    case NODE_KIND.LegacyForTest:
    case NODE_KIND.SyntheticModuleSink:
    case NODE_KIND.SyntheticExpressionStatement:
      return name;
    case NODE_KIND.SyntheticBeyondDepth:
      // ASCII "..." rather than U+2026; some Mermaid renderers stumble on
      // multibyte glyphs inside a node-shape label.
      return "...";
  }
}
