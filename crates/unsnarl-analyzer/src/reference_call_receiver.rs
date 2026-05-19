//! Compute `call` / `receiver` flags for a reference.
//!
//! Mirrors `ts/src/analyzer/reference-call-receiver.ts`. The flags
//! report whether a reference appears as the callee of a call /
//! `new`, or as the object of a member expression.

use unsnarl_annotations::ReferenceAnnotationFlags;
use unsnarl_oxc_parity::AstType;

pub fn reference_call_receiver_flags(
    parent_type: Option<&AstType>,
    key: Option<&str>,
) -> ReferenceAnnotationFlags {
    let Some(parent_type) = parent_type else {
        return ReferenceAnnotationFlags {
            call: false,
            receiver: false,
        };
    };
    match (parent_type, key) {
        (AstType::CallExpression, Some("callee")) | (AstType::NewExpression, Some("callee")) => {
            ReferenceAnnotationFlags {
                call: true,
                receiver: false,
            }
        }
        (AstType::MemberExpression, Some("object")) => ReferenceAnnotationFlags {
            call: false,
            receiver: true,
        },
        _ => ReferenceAnnotationFlags {
            call: false,
            receiver: false,
        },
    }
}

#[cfg(test)]
#[path = "reference_call_receiver_test.rs"]
mod reference_call_receiver_test;
