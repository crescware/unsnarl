import type { VariableDeclarationKind } from "../serializer/variable-declaration-kind.js";
import type { NODE_KIND } from "./node-kind.js";
import type { VISUAL_ELEMENT_TYPE } from "./visual-element-type.js";

// Common shape across every kind. Mutable: builder.ts and the various
// builder/* helpers may patch endLine / unused after the node is first
// inserted into its container. Wrapping in Readonly would force a
// refactor of every post-construction patch site.
type CommonNodeFields = {
  type: typeof VISUAL_ELEMENT_TYPE.Node;
  id: string;
  name: string;
  line: number;
  // Set when the reference logically extends past its identifier line --
  // currently the JSX element case where <A>...</A> spans line..endLine.
  // Renderers display L{line}-{endLine} and prune treats line queries as
  // matching anywhere within the closed range. null when single-line.
  endLine: number | null;
  // Marks a reference whose identifier names a JSX element opening tag, so
  // renderers can wrap the label as `<Name>` regardless of whether the
  // element happens to be single-line (endLine is null).
  isJsxElement: boolean;
  unused: boolean;
};

export type VisualNode =
  | (CommonNodeFields & { kind: typeof NODE_KIND.FunctionDeclaration })
  | (CommonNodeFields & { kind: typeof NODE_KIND.ClassDeclaration })
  | (CommonNodeFields & { kind: typeof NODE_KIND.FormalParameter })
  | (CommonNodeFields & { kind: typeof NODE_KIND.CatchParameter })
  | (CommonNodeFields & { kind: typeof NODE_KIND.SyntheticImplicitGlobal })
  | (CommonNodeFields & { kind: typeof NODE_KIND.ReturnArgumentReference })
  | (CommonNodeFields & { kind: typeof NODE_KIND.SyntheticIfStatementTest })
  | (CommonNodeFields & {
      kind: typeof NODE_KIND.SyntheticSwitchStatementDiscriminant;
    })
  | (CommonNodeFields & { kind: typeof NODE_KIND.SyntheticWhileStatementTest })
  | (CommonNodeFields & {
      kind: typeof NODE_KIND.SyntheticDoWhileStatementTest;
    })
  | (CommonNodeFields & { kind: typeof NODE_KIND.LegacyForTest })
  | (CommonNodeFields & {
      kind: typeof NODE_KIND.SyntheticForStatementHeader;
    })
  | (CommonNodeFields & { kind: typeof NODE_KIND.SyntheticModuleSink })
  | (CommonNodeFields & { kind: typeof NODE_KIND.SyntheticModuleSource })
  | (CommonNodeFields & { kind: typeof NODE_KIND.SyntheticImportIntermediate })
  | (CommonNodeFields & { kind: typeof NODE_KIND.SyntheticExpressionStatement })
  | (CommonNodeFields & { kind: typeof NODE_KIND.SyntheticBeyondDepth })
  | (CommonNodeFields & {
      kind: typeof NODE_KIND.VarBinding;
      initIsFunction: boolean;
    })
  | (CommonNodeFields & {
      kind: typeof NODE_KIND.ConstBinding;
      initIsFunction: boolean;
    })
  | (CommonNodeFields & {
      kind: typeof NODE_KIND.LetBinding;
      initIsFunction: boolean;
    })
  | (CommonNodeFields & {
      kind: typeof NODE_KIND.WriteReference;
      declarationKind: VariableDeclarationKind | null;
    })
  | (CommonNodeFields & {
      kind: typeof NODE_KIND.NamedImportBinding;
      importedName: string;
    })
  | (CommonNodeFields & {
      kind: typeof NODE_KIND.DefaultImportBinding;
    })
  | (CommonNodeFields & {
      kind: typeof NODE_KIND.NamespaceImportBinding;
    });
