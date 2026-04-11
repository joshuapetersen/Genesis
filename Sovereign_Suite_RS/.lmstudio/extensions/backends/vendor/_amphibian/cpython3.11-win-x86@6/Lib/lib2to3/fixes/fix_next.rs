//! fix_next.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::.::{token};

pub const bind_warning: &str = "Calls to builtin next() possibly shadowed by global binding";
pub struct FixNext {
    pub shadowed_next: String, // TODO: infer type
}

impl FixNext {
}

pub fn is_assign_target(node: &str) {
        assign = find_assign ( node );
        if assign is None /* Option */ {
        return  false;
        for child in assign . children .iter() {
        if child . type == token . EQUAL {
        return  false;
        } else if is_subtree ( child , node ) {
        return  true;
        return  false;
        pub fn find_assign ( node )  {
        if node . type == syms . expr_stmt {
        return  node;
        if node . type == syms . simple_stmt || node . parent is None /* Option */ {
        return;
        return  find_assign ( node . parent );
        pub fn is_subtree ( root , node )  {
        if root == node {
        return  true;
        return  any ( is_subtree ( c , node ) for c in root . children );
}

