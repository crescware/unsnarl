//! `oxc_semantic::Scoping` references → `IrArena.references`.
//!
//! Phase 2 stub. The real mapping iterates resolved + root-unresolved
//! reference ids, builds [`unsnarl_ir::ReferenceData`] (identifier,
//! `from` scope, optional `resolved` variable, init flag, flags from
//! `oxc_semantic::ReferenceFlags`), and cross-links them onto
//! `VariableData.references`. The per-fixture decision on `with`-body
//! reference treatment (pinned by
//! `oxc_semantic_probe_test::with_body_identifier_resolves_to_outer_binding_diverging_from_eslint_scope`)
//! lands here once the parity harness has been run.
//!
//! TODO(phase-2): fill in the actual mapping.
