//! IR side-table contract: per-entity annotations produced by the
//! analyzer and consumed by the serializers.
//!
//! `Annotations` is the lookup trait, keyed by arena IDs against
//! `IrArena`. The row types are `ScopeAnnotation` /
//! `ReferenceAnnotation` / `VariableAnnotation`.
//!
//! `NestingDepths` deliberately stays in `unsnarl-ir::nesting_kind`
//! rather than being redefined here, because `SerializedScope`
//! embeds it directly and `unsnarl-ir` is below this crate in the
//! dependency graph. `ScopeAnnotation` imports it.
//!
//! `Serialize` is derived only where the in-memory shape matches the
//! pipeline output shape: `ScopeAnnotation`, `VariableAnnotation`,
//! and the `ReferenceAnnotationFlags` inner struct. `ReferenceAnnotation`
//! itself does not derive `Serialize` because three of its fields
//! (`completion`, `jsx_element`, `expression_statement_container`)
//! reference `unsnarl-ir` types whose in-memory shape carries
//! `Utf16CodeUnitOffset` while the pipeline-emitted shape carries `Span`;
//! adding `Serialize` would produce a second JSON form no pipeline
//! path emits. Field order is preserved by struct declaration order;
//! see `reference_annotation`'s module doc for the deviation rationale.

pub mod annotations;
pub mod reference_annotation;
pub mod scope_annotation;
pub mod variable_annotation;

pub use annotations::Annotations;
pub use reference_annotation::{ReferenceAnnotation, ReferenceAnnotationFlags};
pub use scope_annotation::ScopeAnnotation;
pub use variable_annotation::VariableAnnotation;
