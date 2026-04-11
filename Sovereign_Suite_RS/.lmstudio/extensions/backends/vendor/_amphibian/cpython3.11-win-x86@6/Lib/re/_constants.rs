//! _constants.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::_sre::{MAXREPEAT, MAXGROUPS};

pub const MAGIC: u64 = 20220615;
pub struct error {
    pub msg: String, // TODO: infer type
    pub pattern: String, // TODO: infer type
    pub pos: String, // TODO: infer type
    pub lineno: String, // TODO: infer type
    pub colno: String, // TODO: infer type
    pub name: String, // TODO: infer type
}

impl error {
}

pub struct _NamedIntConstant {
    pub name: String, // TODO: infer type
}

impl _NamedIntConstant {
    pub fn __new__(&self, value: &str, name: &str) {
        self = super ( _NamedIntConstant , cls ) . __new__ ( cls , value );
        self . name = name;
        return  self;
    }

    pub fn _makecodes(&self, names: &str) {
        items = vec![ _NamedIntConstant ( i , name ).iter().map(|i , name| enumerate ( names ) ).collect();
        globals ( ) . update ( { item . name : item for item in items } );
        return  items;
        OPCODES = _makecodes (;
        "FAILURE" , "SUCCESS" ,;
        "ANY" , "ANY_ALL" ,;
        "ASSERT" , "ASSERT_NOT" ,;
        "AT" ,;
        "BRANCH" ,;
        "CATEGORY" ,;
        "CHARSET" , "BIGCHARSET" ,;
        "GROUPREF" , "GROUPREF_EXISTS" ,;
        "IN" ,;
        "INFO" ,;
        "JUMP" ,;
        "LITERAL" ,;
        "MARK" ,;
        "MAX_UNTIL" ,;
        "MIN_UNTIL" ,;
        "NOT_LITERAL" ,;
        "NEGATE" ,;
        "RANGE" ,;
        "REPEAT" ,;
        "REPEAT_ONE" ,;
        "SUBPATTERN" ,;
        "MIN_REPEAT_ONE" ,;
        "ATOMIC_GROUP" ,;
        "POSSESSIVE_REPEAT" ,;
        "POSSESSIVE_REPEAT_ONE" ,;
        "GROUPREF_IGNORE" ,;
        "IN_IGNORE" ,;
        "LITERAL_IGNORE" ,;
        "NOT_LITERAL_IGNORE" ,;
        "GROUPREF_LOC_IGNORE" ,;
        "IN_LOC_IGNORE" ,;
        "LITERAL_LOC_IGNORE" ,;
        "NOT_LITERAL_LOC_IGNORE" ,;
        "GROUPREF_UNI_IGNORE" ,;
        "IN_UNI_IGNORE" ,;
        "LITERAL_UNI_IGNORE" ,;
        "NOT_LITERAL_UNI_IGNORE" ,;
        "RANGE_UNI_IGNORE" ,;
        "MIN_REPEAT" , "MAX_REPEAT" ,;
        );
        del OPCODES [ -2 : ];
        ATCODES = _makecodes (;
        "AT_BEGINNING" , "AT_BEGINNING_LINE" , "AT_BEGINNING_STRING" ,;
        "AT_BOUNDARY" , "AT_NON_BOUNDARY" ,;
        "AT_END" , "AT_END_LINE" , "AT_END_STRING" ,;
        "AT_LOC_BOUNDARY" , "AT_LOC_NON_BOUNDARY" ,;
        "AT_UNI_BOUNDARY" , "AT_UNI_NON_BOUNDARY" ,;
        );
        CHCODES = _makecodes (;
        "CATEGORY_DIGIT" , "CATEGORY_NOT_DIGIT" ,;
        "CATEGORY_SPACE" , "CATEGORY_NOT_SPACE" ,;
        "CATEGORY_WORD" , "CATEGORY_NOT_WORD" ,;
        "CATEGORY_LINEBREAK" , "CATEGORY_NOT_LINEBREAK" ,;
        "CATEGORY_LOC_WORD" , "CATEGORY_LOC_NOT_WORD" ,;
        "CATEGORY_UNI_DIGIT" , "CATEGORY_UNI_NOT_DIGIT" ,;
        "CATEGORY_UNI_SPACE" , "CATEGORY_UNI_NOT_SPACE" ,;
        "CATEGORY_UNI_WORD" , "CATEGORY_UNI_NOT_WORD" ,;
        "CATEGORY_UNI_LINEBREAK" , "CATEGORY_UNI_NOT_LINEBREAK" ,;
        );
        OP_IGNORE = {;
        LITERAL : LITERAL_IGNORE ,;
        NOT_LITERAL : NOT_LITERAL_IGNORE ,;
        };
        OP_LOCALE_IGNORE = {;
        LITERAL : LITERAL_LOC_IGNORE ,;
        NOT_LITERAL : NOT_LITERAL_LOC_IGNORE ,;
        };
        OP_UNICODE_IGNORE = {;
        LITERAL : LITERAL_UNI_IGNORE ,;
        NOT_LITERAL : NOT_LITERAL_UNI_IGNORE ,;
        };
        AT_MULTILINE = {;
        AT_BEGINNING : AT_BEGINNING_LINE ,;
        AT_END : AT_END_LINE;
        };
        AT_LOCALE = {;
        AT_BOUNDARY : AT_LOC_BOUNDARY ,;
        AT_NON_BOUNDARY : AT_LOC_NON_BOUNDARY;
        };
        AT_UNICODE = {;
        AT_BOUNDARY : AT_UNI_BOUNDARY ,;
        AT_NON_BOUNDARY : AT_UNI_NON_BOUNDARY;
        };
        CH_LOCALE = {;
        CATEGORY_DIGIT : CATEGORY_DIGIT ,;
        CATEGORY_NOT_DIGIT : CATEGORY_NOT_DIGIT ,;
        CATEGORY_SPACE : CATEGORY_SPACE ,;
        CATEGORY_NOT_SPACE : CATEGORY_NOT_SPACE ,;
        CATEGORY_WORD : CATEGORY_LOC_WORD ,;
        CATEGORY_NOT_WORD : CATEGORY_LOC_NOT_WORD ,;
        CATEGORY_LINEBREAK : CATEGORY_LINEBREAK ,;
        CATEGORY_NOT_LINEBREAK : CATEGORY_NOT_LINEBREAK;
        };
        CH_UNICODE = {;
        CATEGORY_DIGIT : CATEGORY_UNI_DIGIT ,;
        CATEGORY_NOT_DIGIT : CATEGORY_UNI_NOT_DIGIT ,;
        CATEGORY_SPACE : CATEGORY_UNI_SPACE ,;
        CATEGORY_NOT_SPACE : CATEGORY_UNI_NOT_SPACE ,;
        CATEGORY_WORD : CATEGORY_UNI_WORD ,;
        CATEGORY_NOT_WORD : CATEGORY_UNI_NOT_WORD ,;
        CATEGORY_LINEBREAK : CATEGORY_UNI_LINEBREAK ,;
        CATEGORY_NOT_LINEBREAK : CATEGORY_UNI_NOT_LINEBREAK;
        };
        SRE_FLAG_TEMPLATE = 1;
        SRE_FLAG_IGNORECASE = 2;
        SRE_FLAG_LOCALE = 4;
        SRE_FLAG_MULTILINE = 8;
        SRE_FLAG_DOTALL = 16;
        SRE_FLAG_UNICODE = 32;
        SRE_FLAG_VERBOSE = 64;
        SRE_FLAG_DEBUG = 128;
        SRE_FLAG_ASCII = 256;
        SRE_INFO_PREFIX = 1;
        SRE_INFO_LITERAL = 2;
        SRE_INFO_CHARSET = 4;
    }

}

