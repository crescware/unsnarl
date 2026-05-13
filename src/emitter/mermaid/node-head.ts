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
    case NODE_KIND.LegacyClassName:
      return `class ${name}`;
    case NODE_KIND.LegacyImportBinding: {
      const isRenamedNamed =
        n.importKind === IMPORT_KIND.Named && n.importedName !== n.name;
      return isRenamedNamed ? name : `import ${name}`;
    }
    case NODE_KIND.LegacyCatchClause:
      return `catch ${name}`;
    case NODE_KIND.LegacyImplicitGlobalVariable:
      return `global ${name}`;
    case NODE_KIND.LegacyWriteOp:
      return n.declarationKind === VARIABLE_DECLARATION_KIND.Let
        ? `let ${name}`
        : name;
    case NODE_KIND.LegacyModuleSource:
      return `module ${name}`;
    case NODE_KIND.LegacyImportIntermediate:
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
    case NODE_KIND.LegacyIfTest:
    case NODE_KIND.LegacySwitchDiscriminant:
    case NODE_KIND.LegacyWhileTest:
    case NODE_KIND.LegacyDoWhileTest:
    case NODE_KIND.LegacyForTest:
    case NODE_KIND.LegacyModuleSink:
    case NODE_KIND.LegacyExpressionStatement:
      return name;
    case NODE_KIND.LegacyBeyondDepth:
      // ASCII "..." rather than U+2026; some Mermaid renderers stumble on
      // multibyte glyphs inside a node-shape label.
      return "...";
  }
}
