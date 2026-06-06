//! The `Call` / `New` nodes of a head expression's receiver chain.

use unsnarl_ir::serialized::SerializedHeadExpression;

/// The `Call` / `New` nodes of a head expression's receiver chain,
/// outermost first. `arr.map(f).filter(g)` parses as
/// `(((arr.map)(f)).filter)(g)`, so the head is the outer `filter`
/// `Call` whose callee's object is the inner `map` `Call`; descending
/// through each `Call`'s `Member` callee object yields
/// `[filter-call, map-call]`. Only the *receiver* chain is followed
/// (a `Member` object that is itself a `Call`); arguments are not part
/// of the head, so a call nested as an argument (`foo(items.map(cb))`)
/// contributes no inner node here and is left to its single host proxy.
pub fn receiver_call_chain(head: &SerializedHeadExpression) -> Vec<&SerializedHeadExpression> {
    let mut out = Vec::new();
    let mut node = head;
    while let SerializedHeadExpression::Call { callee, .. }
    | SerializedHeadExpression::New { callee, .. } = node
    {
        out.push(node);
        match callee.as_ref() {
            SerializedHeadExpression::Member { object, .. } => node = object,
            _ => break,
        }
    }
    out
}
