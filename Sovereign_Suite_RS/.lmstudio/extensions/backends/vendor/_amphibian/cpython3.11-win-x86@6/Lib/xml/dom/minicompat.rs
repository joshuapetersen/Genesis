//! minicompat.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::xml;

pub const __all__: &str = ["NodeList" ,"EmptyNodeList" ,"StringTypes" ,"defproperty" ];
pub const StringTypes: f64 = ( str , );
pub struct NodeList {
}

impl NodeList {
}

pub struct EmptyNodeList {
}

impl EmptyNodeList {
}

pub fn defproperty(klass: &str, name: &str, doc: &str) {
        get = getattr ( klass , ( "_get_" + name ) );
        pub fn set ( &self, value , name = name )  {
        panic!("xml . dom . NoModificationAllowedErr (");
        "attempt to modify read-only attribute " + repr ( name ) );
        assert !hasattr ( klass , "_set_" + name ) , \;
        "expected !to find _set_" + name;
        prop = property ( get , set , doc = doc );
        setattr ( klass , name , prop );
}

