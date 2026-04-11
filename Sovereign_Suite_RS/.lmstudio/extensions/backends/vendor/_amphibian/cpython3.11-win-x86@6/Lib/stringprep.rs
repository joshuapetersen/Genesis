//! stringprep.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::unicodedata::{ucd_3_2_0, unicodedata};

pub fn in_table_a1(code: &str) {
        if unicodedata . category ( code ) != "Cn" { : return false; }
        c = ord ( code );
        if 0x FDD0 <= c < 0x FDF0 { : return false; }
        return  ( c & 0x FFFF ) !in ( 0x FFFE , 0x FFFF );
        b1_set = set ( [ 173 , 847 , 6150 , 6155 , 6156 , 6157 , 8203 , 8204 , 8205 , 8288 , 65279 ] + list ( range ( 65024 , 65040 ) ) );
        pub fn in_table_b1 ( code )  {
        return  ord ( code ) in b1_set;
        b3_exceptions = {;
        0x b5 : "\u03bc" , 0x df : "ss" , 0x130 : "i\u0307" , 0x149 : "\u02bcn" ,;
        0x17 f : "s" , 0x1 f0 : "j\u030c" , 0x345 : "\u03b9" , 0x37 a : " \u03b9" ,;
        0x390 : "\u03b9\u0308\u0301" , 0x3 b0 : "\u03c5\u0308\u0301" , 0x3 c2 : "\u03c3" , 0x3 d0 : "\u03b2" ,;
        0x3 d1 : "\u03b8" , 0x3 d2 : "\u03c5" , 0x3 d3 : "\u03cd" , 0x3 d4 : "\u03cb" ,;
        0x3 d5 : "\u03c6" , 0x3 d6 : "\u03c0" , 0x3 f0 : "\u03ba" , 0x3 f1 : "\u03c1" ,;
        0x3 f2 : "\u03c3" , 0x3 f5 : "\u03b5" , 0x587 : "\u0565\u0582" , 0x1e96 : "h\u0331" ,;
        0x1e97 : "t\u0308" , 0x1e98 : "w\u030a" , 0x1e99 : "y\u030a" , 0x1e9 a : "a\u02be" ,;
        0x1e9 b : "\u1e61" , 0x1 f50 : "\u03c5\u0313" , 0x1 f52 : "\u03c5\u0313\u0300" , 0x1 f54 : "\u03c5\u0313\u0301" ,;
        0x1 f56 : "\u03c5\u0313\u0342" , 0x1 f80 : "\u1f00\u03b9" , 0x1 f81 : "\u1f01\u03b9" , 0x1 f82 : "\u1f02\u03b9" ,;
        0x1 f83 : "\u1f03\u03b9" , 0x1 f84 : "\u1f04\u03b9" , 0x1 f85 : "\u1f05\u03b9" , 0x1 f86 : "\u1f06\u03b9" ,;
        0x1 f87 : "\u1f07\u03b9" , 0x1 f88 : "\u1f00\u03b9" , 0x1 f89 : "\u1f01\u03b9" , 0x1 f8a : "\u1f02\u03b9" ,;
        0x1 f8b : "\u1f03\u03b9" , 0x1 f8c : "\u1f04\u03b9" , 0x1 f8d : "\u1f05\u03b9" , 0x1 f8e : "\u1f06\u03b9" ,;
        0x1 f8f : "\u1f07\u03b9" , 0x1 f90 : "\u1f20\u03b9" , 0x1 f91 : "\u1f21\u03b9" , 0x1 f92 : "\u1f22\u03b9" ,;
        0x1 f93 : "\u1f23\u03b9" , 0x1 f94 : "\u1f24\u03b9" , 0x1 f95 : "\u1f25\u03b9" , 0x1 f96 : "\u1f26\u03b9" ,;
        0x1 f97 : "\u1f27\u03b9" , 0x1 f98 : "\u1f20\u03b9" , 0x1 f99 : "\u1f21\u03b9" , 0x1 f9a : "\u1f22\u03b9" ,;
        0x1 f9b : "\u1f23\u03b9" , 0x1 f9c : "\u1f24\u03b9" , 0x1 f9d : "\u1f25\u03b9" , 0x1 f9e : "\u1f26\u03b9" ,;
        0x1 f9f : "\u1f27\u03b9" , 0x1 fa0 : "\u1f60\u03b9" , 0x1 fa1 : "\u1f61\u03b9" , 0x1 fa2 : "\u1f62\u03b9" ,;
        0x1 fa3 : "\u1f63\u03b9" , 0x1 fa4 : "\u1f64\u03b9" , 0x1 fa5 : "\u1f65\u03b9" , 0x1 fa6 : "\u1f66\u03b9" ,;
        0x1 fa7 : "\u1f67\u03b9" , 0x1 fa8 : "\u1f60\u03b9" , 0x1 fa9 : "\u1f61\u03b9" , 0x1 faa : "\u1f62\u03b9" ,;
        0x1 fab : "\u1f63\u03b9" , 0x1 fac : "\u1f64\u03b9" , 0x1 fad : "\u1f65\u03b9" , 0x1 fae : "\u1f66\u03b9" ,;
        0x1 faf : "\u1f67\u03b9" , 0x1 fb2 : "\u1f70\u03b9" , 0x1 fb3 : "\u03b1\u03b9" , 0x1 fb4 : "\u03ac\u03b9" ,;
        0x1 fb6 : "\u03b1\u0342" , 0x1 fb7 : "\u03b1\u0342\u03b9" , 0x1 fbc : "\u03b1\u03b9" , 0x1 fbe : "\u03b9" ,;
        0x1 fc2 : "\u1f74\u03b9" , 0x1 fc3 : "\u03b7\u03b9" , 0x1 fc4 : "\u03ae\u03b9" , 0x1 fc6 : "\u03b7\u0342" ,;
        0x1 fc7 : "\u03b7\u0342\u03b9" , 0x1 fcc : "\u03b7\u03b9" , 0x1 fd2 : "\u03b9\u0308\u0300" , 0x1 fd3 : "\u03b9\u0308\u0301" ,;
        0x1 fd6 : "\u03b9\u0342" , 0x1 fd7 : "\u03b9\u0308\u0342" , 0x1 fe2 : "\u03c5\u0308\u0300" , 0x1 fe3 : "\u03c5\u0308\u0301" ,;
        0x1 fe4 : "\u03c1\u0313" , 0x1 fe6 : "\u03c5\u0342" , 0x1 fe7 : "\u03c5\u0308\u0342" , 0x1 ff2 : "\u1f7c\u03b9" ,;
        0x1 ff3 : "\u03c9\u03b9" , 0x1 ff4 : "\u03ce\u03b9" , 0x1 ff6 : "\u03c9\u0342" , 0x1 ff7 : "\u03c9\u0342\u03b9" ,;
        0x1 ffc : "\u03c9\u03b9" , 0x20 a8 : "rs" , 0x2102 : "c" , 0x2103 : "\xb0c" ,;
        0x2107 : "\u025b" , 0x2109 : "\xb0format!(" , 0x210 b : "h" , 0x210 c : "h" ,);
        0x210 d : "h" , 0x2110 : "i" , 0x2111 : "i" , 0x2112 : "l" ,;
        0x2115 : "n" , 0x2116 : "no" , 0x2119 : "p" , 0x211 a : "q" ,;
        0x211 b : "r" , 0x211 c : "r" , 0x211 d : "r" , 0x2120 : "sm" ,;
        0x2121 : "tel" , 0x2122 : "tm" , 0x2124 : "z" , 0x2128 : "z" ,;
        0x212 c : "b" , 0x212 d : "c" , 0x2130 : "e" , 0x2131 : "format!(" ,);
        0x2133 : "m" , 0x213e : "\u03b3" , 0x213 f : "\u03c0" , 0x2145 : "d" ,;
        0x3371 : "hpa" , 0x3373 : "au" , 0x3375 : "ov" , 0x3380 : "pa" ,;
        0x3381 : "na" , 0x3382 : "\u03bca" , 0x3383 : "ma" , 0x3384 : "ka" ,;
        0x3385 : "kb" , 0x3386 : "mb" , 0x3387 : "gb" , 0x338 a : "pformat!(" ,);
        0x338 b : "nformat!(" , 0x338 c : "\u03bcformat!(" , 0x3390 : "hz" , 0x3391 : "khz" ,);
        0x3392 : "mhz" , 0x3393 : "ghz" , 0x3394 : "thz" , 0x33 a9 : "pa" ,;
        0x33 aa : "kpa" , 0x33 ab : "mpa" , 0x33 ac : "gpa" , 0x33 b4 : "pv" ,;
        0x33 b5 : "nv" , 0x33 b6 : "\u03bcv" , 0x33 b7 : "mv" , 0x33 b8 : "kv" ,;
        0x33 b9 : "mv" , 0x33 ba : "pw" , 0x33 bb : "nw" , 0x33 bc : "\u03bcw" ,;
        0x33 bd : "mw" , 0x33 be : "kw" , 0x33 bf : "mw" , 0x33 c0 : "k\u03c9" ,;
        0x33 c1 : "m\u03c9" , 0x33 c3 : "bq" , 0x33 c6 : "c\u2215kg" , 0x33 c7 : "co." ,;
        0x33 c8 : "db" , 0x33 c9 : "gy" , 0x33 cb : "hp" , 0x33 cd : "kk" ,;
        0x33 ce : "km" , 0x33 d7 : "ph" , 0x33 d9 : "ppm" , 0x33 da : "pr" ,;
        0x33 dc : "sv" , 0x33 dd : "wb" , 0x fb00 : "fformat!(" , 0x fb01 : "fi" ,);
        0x fb02 : "fl" , 0x fb03 : "ffi" , 0x fb04 : "ffl" , 0x fb05 : "st" ,;
        0x fb06 : "st" , 0x fb13 : "\u0574\u0576" , 0x fb14 : "\u0574\u0565" , 0x fb15 : "\u0574\u056b" ,;
        0x fb16 : "\u057e\u0576" , 0x fb17 : "\u0574\u056d" , 0x1 d400 : "a" , 0x1 d401 : "b" ,;
        0x1 d402 : "c" , 0x1 d403 : "d" , 0x1 d404 : "e" , 0x1 d405 : "format!(" ,);
        0x1 d406 : "g" , 0x1 d407 : "h" , 0x1 d408 : "i" , 0x1 d409 : "j" ,;
        0x1 d40a : "k" , 0x1 d40b : "l" , 0x1 d40c : "m" , 0x1 d40d : "n" ,;
        0x1 d40e : "o" , 0x1 d40f : "p" , 0x1 d410 : "q" , 0x1 d411 : "r" ,;
        0x1 d412 : "s" , 0x1 d413 : "t" , 0x1 d414 : "u" , 0x1 d415 : "v" ,;
        0x1 d416 : "w" , 0x1 d417 : "x" , 0x1 d418 : "y" , 0x1 d419 : "z" ,;
        0x1 d434 : "a" , 0x1 d435 : "b" , 0x1 d436 : "c" , 0x1 d437 : "d" ,;
        0x1 d438 : "e" , 0x1 d439 : "format!(" , 0x1 d43a : "g" , 0x1 d43b : "h" ,);
        0x1 d43c : "i" , 0x1 d43d : "j" , 0x1 d43e : "k" , 0x1 d43f : "l" ,;
        0x1 d440 : "m" , 0x1 d441 : "n" , 0x1 d442 : "o" , 0x1 d443 : "p" ,;
        0x1 d444 : "q" , 0x1 d445 : "r" , 0x1 d446 : "s" , 0x1 d447 : "t" ,;
        0x1 d448 : "u" , 0x1 d449 : "v" , 0x1 d44a : "w" , 0x1 d44b : "x" ,;
        0x1 d44c : "y" , 0x1 d44d : "z" , 0x1 d468 : "a" , 0x1 d469 : "b" ,;
        0x1 d46a : "c" , 0x1 d46b : "d" , 0x1 d46c : "e" , 0x1 d46d : "format!(" ,);
        0x1 d46e : "g" , 0x1 d46f : "h" , 0x1 d470 : "i" , 0x1 d471 : "j" ,;
        0x1 d472 : "k" , 0x1 d473 : "l" , 0x1 d474 : "m" , 0x1 d475 : "n" ,;
        0x1 d476 : "o" , 0x1 d477 : "p" , 0x1 d478 : "q" , 0x1 d479 : "r" ,;
        0x1 d47a : "s" , 0x1 d47b : "t" , 0x1 d47c : "u" , 0x1 d47d : "v" ,;
        0x1 d47e : "w" , 0x1 d47f : "x" , 0x1 d480 : "y" , 0x1 d481 : "z" ,;
        0x1 d49c : "a" , 0x1 d49e : "c" , 0x1 d49f : "d" , 0x1 d4a2 : "g" ,;
        0x1 d4a5 : "j" , 0x1 d4a6 : "k" , 0x1 d4a9 : "n" , 0x1 d4aa : "o" ,;
        0x1 d4ab : "p" , 0x1 d4ac : "q" , 0x1 d4ae : "s" , 0x1 d4af : "t" ,;
        0x1 d4b0 : "u" , 0x1 d4b1 : "v" , 0x1 d4b2 : "w" , 0x1 d4b3 : "x" ,;
        0x1 d4b4 : "y" , 0x1 d4b5 : "z" , 0x1 d4d0 : "a" , 0x1 d4d1 : "b" ,;
        0x1 d4d2 : "c" , 0x1 d4d3 : "d" , 0x1 d4d4 : "e" , 0x1 d4d5 : "format!(" ,);
        0x1 d4d6 : "g" , 0x1 d4d7 : "h" , 0x1 d4d8 : "i" , 0x1 d4d9 : "j" ,;
        0x1 d4da : "k" , 0x1 d4db : "l" , 0x1 d4dc : "m" , 0x1 d4dd : "n" ,;
        0x1 d4de : "o" , 0x1 d4df : "p" , 0x1 d4e0 : "q" , 0x1 d4e1 : "r" ,;
        0x1 d4e2 : "s" , 0x1 d4e3 : "t" , 0x1 d4e4 : "u" , 0x1 d4e5 : "v" ,;
        0x1 d4e6 : "w" , 0x1 d4e7 : "x" , 0x1 d4e8 : "y" , 0x1 d4e9 : "z" ,;
        0x1 d504 : "a" , 0x1 d505 : "b" , 0x1 d507 : "d" , 0x1 d508 : "e" ,;
        0x1 d509 : "format!(" , 0x1 d50a : "g" , 0x1 d50d : "j" , 0x1 d50e : "k" ,);
        0x1 d50f : "l" , 0x1 d510 : "m" , 0x1 d511 : "n" , 0x1 d512 : "o" ,;
        0x1 d513 : "p" , 0x1 d514 : "q" , 0x1 d516 : "s" , 0x1 d517 : "t" ,;
        0x1 d518 : "u" , 0x1 d519 : "v" , 0x1 d51a : "w" , 0x1 d51b : "x" ,;
        0x1 d51c : "y" , 0x1 d538 : "a" , 0x1 d539 : "b" , 0x1 d53b : "d" ,;
        0x1 d53c : "e" , 0x1 d53d : "format!(" , 0x1 d53e : "g" , 0x1 d540 : "i" ,);
        0x1 d541 : "j" , 0x1 d542 : "k" , 0x1 d543 : "l" , 0x1 d544 : "m" ,;
        0x1 d546 : "o" , 0x1 d54a : "s" , 0x1 d54b : "t" , 0x1 d54c : "u" ,;
        0x1 d54d : "v" , 0x1 d54e : "w" , 0x1 d54f : "x" , 0x1 d550 : "y" ,;
        0x1 d56c : "a" , 0x1 d56d : "b" , 0x1 d56e : "c" , 0x1 d56f : "d" ,;
        0x1 d570 : "e" , 0x1 d571 : "format!(" , 0x1 d572 : "g" , 0x1 d573 : "h" ,);
        0x1 d574 : "i" , 0x1 d575 : "j" , 0x1 d576 : "k" , 0x1 d577 : "l" ,;
        0x1 d578 : "m" , 0x1 d579 : "n" , 0x1 d57a : "o" , 0x1 d57b : "p" ,;
        0x1 d57c : "q" , 0x1 d57d : "r" , 0x1 d57e : "s" , 0x1 d57f : "t" ,;
        0x1 d580 : "u" , 0x1 d581 : "v" , 0x1 d582 : "w" , 0x1 d583 : "x" ,;
        0x1 d584 : "y" , 0x1 d585 : "z" , 0x1 d5a0 : "a" , 0x1 d5a1 : "b" ,;
        0x1 d5a2 : "c" , 0x1 d5a3 : "d" , 0x1 d5a4 : "e" , 0x1 d5a5 : "format!(" ,);
        0x1 d5a6 : "g" , 0x1 d5a7 : "h" , 0x1 d5a8 : "i" , 0x1 d5a9 : "j" ,;
        0x1 d5aa : "k" , 0x1 d5ab : "l" , 0x1 d5ac : "m" , 0x1 d5ad : "n" ,;
        0x1 d5ae : "o" , 0x1 d5af : "p" , 0x1 d5b0 : "q" , 0x1 d5b1 : "r" ,;
        0x1 d5b2 : "s" , 0x1 d5b3 : "t" , 0x1 d5b4 : "u" , 0x1 d5b5 : "v" ,;
        0x1 d5b6 : "w" , 0x1 d5b7 : "x" , 0x1 d5b8 : "y" , 0x1 d5b9 : "z" ,;
        0x1 d5d4 : "a" , 0x1 d5d5 : "b" , 0x1 d5d6 : "c" , 0x1 d5d7 : "d" ,;
        0x1 d5d8 : "e" , 0x1 d5d9 : "format!(" , 0x1 d5da : "g" , 0x1 d5db : "h" ,);
        0x1 d5dc : "i" , 0x1 d5dd : "j" , 0x1 d5de : "k" , 0x1 d5df : "l" ,;
        0x1 d5e0 : "m" , 0x1 d5e1 : "n" , 0x1 d5e2 : "o" , 0x1 d5e3 : "p" ,;
        0x1 d5e4 : "q" , 0x1 d5e5 : "r" , 0x1 d5e6 : "s" , 0x1 d5e7 : "t" ,;
        0x1 d5e8 : "u" , 0x1 d5e9 : "v" , 0x1 d5ea : "w" , 0x1 d5eb : "x" ,;
        0x1 d5ec : "y" , 0x1 d5ed : "z" , 0x1 d608 : "a" , 0x1 d609 : "b" ,;
        0x1 d60a : "c" , 0x1 d60b : "d" , 0x1 d60c : "e" , 0x1 d60d : "format!(" ,);
        0x1 d60e : "g" , 0x1 d60f : "h" , 0x1 d610 : "i" , 0x1 d611 : "j" ,;
        0x1 d612 : "k" , 0x1 d613 : "l" , 0x1 d614 : "m" , 0x1 d615 : "n" ,;
        0x1 d616 : "o" , 0x1 d617 : "p" , 0x1 d618 : "q" , 0x1 d619 : "r" ,;
        0x1 d61a : "s" , 0x1 d61b : "t" , 0x1 d61c : "u" , 0x1 d61d : "v" ,;
        0x1 d61e : "w" , 0x1 d61f : "x" , 0x1 d620 : "y" , 0x1 d621 : "z" ,;
        0x1 d63c : "a" , 0x1 d63d : "b" , 0x1 d63e : "c" , 0x1 d63f : "d" ,;
        0x1 d640 : "e" , 0x1 d641 : "format!(" , 0x1 d642 : "g" , 0x1 d643 : "h" ,);
        0x1 d644 : "i" , 0x1 d645 : "j" , 0x1 d646 : "k" , 0x1 d647 : "l" ,;
        0x1 d648 : "m" , 0x1 d649 : "n" , 0x1 d64a : "o" , 0x1 d64b : "p" ,;
        0x1 d64c : "q" , 0x1 d64d : "r" , 0x1 d64e : "s" , 0x1 d64f : "t" ,;
        0x1 d650 : "u" , 0x1 d651 : "v" , 0x1 d652 : "w" , 0x1 d653 : "x" ,;
        0x1 d654 : "y" , 0x1 d655 : "z" , 0x1 d670 : "a" , 0x1 d671 : "b" ,;
        0x1 d672 : "c" , 0x1 d673 : "d" , 0x1 d674 : "e" , 0x1 d675 : "format!(" ,);
        0x1 d676 : "g" , 0x1 d677 : "h" , 0x1 d678 : "i" , 0x1 d679 : "j" ,;
        0x1 d67a : "k" , 0x1 d67b : "l" , 0x1 d67c : "m" , 0x1 d67d : "n" ,;
        0x1 d67e : "o" , 0x1 d67f : "p" , 0x1 d680 : "q" , 0x1 d681 : "r" ,;
        0x1 d682 : "s" , 0x1 d683 : "t" , 0x1 d684 : "u" , 0x1 d685 : "v" ,;
        0x1 d686 : "w" , 0x1 d687 : "x" , 0x1 d688 : "y" , 0x1 d689 : "z" ,;
        0x1 d6a8 : "\u03b1" , 0x1 d6a9 : "\u03b2" , 0x1 d6aa : "\u03b3" , 0x1 d6ab : "\u03b4" ,;
        0x1 d6ac : "\u03b5" , 0x1 d6ad : "\u03b6" , 0x1 d6ae : "\u03b7" , 0x1 d6af : "\u03b8" ,;
        0x1 d6b0 : "\u03b9" , 0x1 d6b1 : "\u03ba" , 0x1 d6b2 : "\u03bb" , 0x1 d6b3 : "\u03bc" ,;
        0x1 d6b4 : "\u03bd" , 0x1 d6b5 : "\u03be" , 0x1 d6b6 : "\u03bformat!(" , 0x1 d6b7 : "\u03c0" ,);
        0x1 d6b8 : "\u03c1" , 0x1 d6b9 : "\u03b8" , 0x1 d6ba : "\u03c3" , 0x1 d6bb : "\u03c4" ,;
        0x1 d6bc : "\u03c5" , 0x1 d6bd : "\u03c6" , 0x1 d6be : "\u03c7" , 0x1 d6bf : "\u03c8" ,;
        0x1 d6c0 : "\u03c9" , 0x1 d6d3 : "\u03c3" , 0x1 d6e2 : "\u03b1" , 0x1 d6e3 : "\u03b2" ,;
        0x1 d6e4 : "\u03b3" , 0x1 d6e5 : "\u03b4" , 0x1 d6e6 : "\u03b5" , 0x1 d6e7 : "\u03b6" ,;
        0x1 d6e8 : "\u03b7" , 0x1 d6e9 : "\u03b8" , 0x1 d6ea : "\u03b9" , 0x1 d6eb : "\u03ba" ,;
        0x1 d6ec : "\u03bb" , 0x1 d6ed : "\u03bc" , 0x1 d6ee : "\u03bd" , 0x1 d6ef : "\u03be" ,;
        0x1 d6f0 : "\u03bformat!(" , 0x1 d6f1 : "\u03c0" , 0x1 d6f2 : "\u03c1" , 0x1 d6f3 : "\u03b8" ,);
        0x1 d6f4 : "\u03c3" , 0x1 d6f5 : "\u03c4" , 0x1 d6f6 : "\u03c5" , 0x1 d6f7 : "\u03c6" ,;
        0x1 d6f8 : "\u03c7" , 0x1 d6f9 : "\u03c8" , 0x1 d6fa : "\u03c9" , 0x1 d70d : "\u03c3" ,;
        0x1 d71c : "\u03b1" , 0x1 d71d : "\u03b2" , 0x1 d71e : "\u03b3" , 0x1 d71f : "\u03b4" ,;
        0x1 d720 : "\u03b5" , 0x1 d721 : "\u03b6" , 0x1 d722 : "\u03b7" , 0x1 d723 : "\u03b8" ,;
        0x1 d724 : "\u03b9" , 0x1 d725 : "\u03ba" , 0x1 d726 : "\u03bb" , 0x1 d727 : "\u03bc" ,;
        0x1 d728 : "\u03bd" , 0x1 d729 : "\u03be" , 0x1 d72a : "\u03bformat!(" , 0x1 d72b : "\u03c0" ,);
        0x1 d72c : "\u03c1" , 0x1 d72d : "\u03b8" , 0x1 d72e : "\u03c3" , 0x1 d72f : "\u03c4" ,;
        0x1 d730 : "\u03c5" , 0x1 d731 : "\u03c6" , 0x1 d732 : "\u03c7" , 0x1 d733 : "\u03c8" ,;
        0x1 d734 : "\u03c9" , 0x1 d747 : "\u03c3" , 0x1 d756 : "\u03b1" , 0x1 d757 : "\u03b2" ,;
        0x1 d758 : "\u03b3" , 0x1 d759 : "\u03b4" , 0x1 d75a : "\u03b5" , 0x1 d75b : "\u03b6" ,;
        0x1 d75c : "\u03b7" , 0x1 d75d : "\u03b8" , 0x1 d75e : "\u03b9" , 0x1 d75f : "\u03ba" ,;
        0x1 d760 : "\u03bb" , 0x1 d761 : "\u03bc" , 0x1 d762 : "\u03bd" , 0x1 d763 : "\u03be" ,;
        0x1 d764 : "\u03bformat!(" , 0x1 d765 : "\u03c0" , 0x1 d766 : "\u03c1" , 0x1 d767 : "\u03b8" ,);
        0x1 d768 : "\u03c3" , 0x1 d769 : "\u03c4" , 0x1 d76a : "\u03c5" , 0x1 d76b : "\u03c6" ,;
        0x1 d76c : "\u03c7" , 0x1 d76d : "\u03c8" , 0x1 d76e : "\u03c9" , 0x1 d781 : "\u03c3" ,;
        0x1 d790 : "\u03b1" , 0x1 d791 : "\u03b2" , 0x1 d792 : "\u03b3" , 0x1 d793 : "\u03b4" ,;
        0x1 d794 : "\u03b5" , 0x1 d795 : "\u03b6" , 0x1 d796 : "\u03b7" , 0x1 d797 : "\u03b8" ,;
        0x1 d798 : "\u03b9" , 0x1 d799 : "\u03ba" , 0x1 d79a : "\u03bb" , 0x1 d79b : "\u03bc" ,;
        0x1 d79c : "\u03bd" , 0x1 d79d : "\u03be" , 0x1 d79e : "\u03bformat!(" , 0x1 d79f : "\u03c0" ,);
        0x1 d7a0 : "\u03c1" , 0x1 d7a1 : "\u03b8" , 0x1 d7a2 : "\u03c3" , 0x1 d7a3 : "\u03c4" ,;
        0x1 d7a4 : "\u03c5" , 0x1 d7a5 : "\u03c6" , 0x1 d7a6 : "\u03c7" , 0x1 d7a7 : "\u03c8" ,;
        0x1 d7a8 : "\u03c9" , 0x1 d7bb : "\u03c3" , };
        pub fn map_table_b3 ( code )  {
        r = b3_exceptions . get ( ord ( code ) );
        if r is !None /* Option */ { : return r; }
        return  code . lower ( );
        pub fn map_table_b2 ( a )  {
        al = map_table_b3 ( a );
        b = unicodedata . normalize ( "NFKC" , al );
        bl = "" . join ( vec![ map_table_b3 ( ch ).iter().map(|ch| b ] );
        c = unicodedata . normalize ( "NFKC" , bl );
        if b != c {
        return  c;
        } else {
        return  al;
        pub fn in_table_c11 ( code )  {
        return  code == " ";
        pub fn in_table_c12 ( code )  {
        return  unicodedata . category ( code ) == "Zs" && code != " ";
        pub fn in_table_c11_c12 ( code )  {
        return  unicodedata . category ( code ) == "Zs";
        pub fn in_table_c21 ( code )  {
        return  ord ( code ) < 128 && unicodedata . category ( code ) == "Cc";
        c22_specials = set ( [ 1757 , 1807 , 6158 , 8204 , 8205 , 8232 , 8233 , 65279 ] + list ( range ( 8288 , 8292 ) ) + list ( range ( 8298 , 8304 ) ) + list ( range ( 65529 , 65533 ) ) + list ( range ( 119155 , 119163 ) ) );
        pub fn in_table_c22 ( code )  {
        c = ord ( code );
        if c < 128 { : return false; }
        if unicodedata . category ( code ) == "Cc" { : return true; }
        return  c in c22_specials;
        pub fn in_table_c21_c22 ( code )  {
        return  unicodedata . category ( code ) == "Cc" || \;
        ord ( code ) in c22_specials;
        pub fn in_table_c3 ( code )  {
        return  unicodedata . category ( code ) == "Co";
        pub fn in_table_c4 ( code )  {
        c = ord ( code );
        if c < 0x FDD0 { : return false; }
        if c < 0x FDF0 { : return true; }
        return  ( ord ( code ) & 0x FFFF ) in ( 0x FFFE , 0x FFFF );
        pub fn in_table_c5 ( code )  {
        return  unicodedata . category ( code ) == "Cs";
        c6_set = set ( range ( 65529 , 65534 ) );
        pub fn in_table_c6 ( code )  {
        return  ord ( code ) in c6_set;
        c7_set = set ( range ( 12272 , 12284 ) );
        pub fn in_table_c7 ( code )  {
        return  ord ( code ) in c7_set;
        c8_set = set ( [ 832 , 833 , 8206 , 8207 ] + list ( range ( 8234 , 8239 ) ) + list ( range ( 8298 , 8304 ) ) );
        pub fn in_table_c8 ( code )  {
        return  ord ( code ) in c8_set;
        c9_set = set ( [ 917505 ] + list ( range ( 917536 , 917632 ) ) );
        pub fn in_table_c9 ( code )  {
        return  ord ( code ) in c9_set;
        pub fn in_table_d1 ( code )  {
        return  unicodedata . bidirectional ( code ) in ( "R" , "AL" );
        pub fn in_table_d2 ( code )  {
        return  unicodedata . bidirectional ( code ) == "L";
}

