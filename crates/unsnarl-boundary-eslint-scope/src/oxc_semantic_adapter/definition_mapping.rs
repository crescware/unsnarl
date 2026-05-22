//! Declaration sites → `IrArena.definitions`.
//!
//! Phase 2 stub. The real mapping walks each `SymbolId`'s
//! `Scoping::symbol_declaration(id) -> NodeId`, classifies the node's
//! `AstKind` into one of the seven `DefinitionType` variants
//! (`Variable`, `ImportBinding`, `FunctionName`, `ClassName`,
//! `Parameter`, `CatchClause`, `ImplicitGlobalVariable`), then walks
//! parents via `AstNodes::ancestor_kinds` to recover the four
//! serializer-required extras: `init` (for `VariableDeclarator.init`),
//! `declaration_kind` (parent `VariableDeclaration.kind`),
//! `import_source` (parent `ImportDeclaration.source.value`), and
//! `imported_name` (`ImportSpecifier.imported`).
//!
//! TODO(phase-2): fill in the actual mapping.
