//! grammar.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::pickle;
// use crate::.::{token};
// use crate::pprint::{pprint};

pub struct Grammar {
    pub symbol2number: String, // TODO: infer type
    pub number2symbol: String, // TODO: infer type
    pub states: String, // TODO: infer type
    pub dfas: String, // TODO: infer type
    pub labels: String, // TODO: infer type
    pub keywords: String, // TODO: infer type
    pub tokens: String, // TODO: infer type
    pub symbol2label: String, // TODO: infer type
    pub start: String, // TODO: infer type
}

impl Grammar {
}

pub const opmap_raw: &str = "
( LPAR
) RPAR
[ LSQB
] RSQB
: COLON
, COMMA
; SEMI
+ PLUS
- MINUS
* STAR
/ SLASH
| VBAR
& AMPER
< LESS
> GREATER
= EQUAL
. DOT
% PERCENT
` BACKQUOTE
{ LBRACE
} RBRACE
@ AT
@= ATEQUAL
== EQEQUAL
!= NOTEQUAL
<> NOTEQUAL
<= LESSEQUAL
>= GREATEREQUAL
~ TILDE
^ CIRCUMFLEX
<< LEFTSHIFT
>> RIGHTSHIFT
** DOUBLESTAR
+= PLUSEQUAL
-= MINEQUAL
*= STAREQUAL
/= SLASHEQUAL
%= PERCENTEQUAL
&= AMPEREQUAL
|= VBAREQUAL
^= CIRCUMFLEXEQUAL
<<= LEFTSHIFTEQUAL
>>= RIGHTSHIFTEQUAL
**= DOUBLESTAREQUAL
// DOUBLESLASH
//= DOUBLESLASHEQUAL
-> RARROW
:= COLONEQUAL
";
pub const opmap: f64 = { };
