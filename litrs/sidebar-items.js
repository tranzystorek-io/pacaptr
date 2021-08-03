initSidebarItems({"enum":[["BoolLit","A bool literal: `true` or `false`. Also see the reference."],["FloatType","All possible float type suffixes."],["IntegerBase","The bases in which an integer can be specified."],["IntegerType","All possible integer type suffixes."],["Literal","A literal. This is the main type of this library."]],"struct":[["ByteLit","A (single) byte literal, e.g. `b'k'` or `b'!'`."],["ByteStringLit","A byte string or raw byte string literal, e.g. `b\"hello\"` or `br#\"abc\"def\"#`."],["CharLit","A character literal, e.g. `'g'` or `'🦊'`."],["FloatLit","A floating point literal, e.g. `3.14`, `8.`, `135e12`, `27f32` or `1.956e2f64`."],["IntegerLit","An integer literal, e.g. `27`, `0x7F`, `0b101010u8` or `5_000_000i64`."],["InvalidToken","An error signaling that a different kind of token was expected. Returned by the various `TryFrom` impls."],["ParseError","Errors during parsing."],["StringLit","A string or raw string literal, e.g. `\"foo\"`, `\"Grüße\"` or `r#\"a🦊c\"d🦀f\"#`."]],"trait":[["Buffer","A shared or owned string buffer. Implemented for `String` and `&str`. Implementation detail."],["FromIntegerLiteral","Integer literal types. Implementation detail."]],"type":[["OwnedLiteral","A literal which owns the underlying buffer."],["SharedLiteral","A literal whose underlying buffer is borrowed."]]});