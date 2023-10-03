// use core::fmt;

#![allow(non_camel_case_types)]

// also operators
#[derive(Debug)]
pub struct Keyword<'a> {
    pub syntax: &'a str,
    pub t_type: TokenType,
}

#[derive(Debug, Default)]
pub struct Token {
    pub t_type: TokenType,
    pub value: Option<String>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum TokenType {
    #[default]
    NONE,

    // Keywords (words)
    T_RETURN,
    T_INT,

    // Keywords (symbols)
    T_EQ,
    T_PLUS,
    T_MINUS,
    T_FSLASH,
    T_STAR,
    T_SEMI,
    T_UNDERSCORE,
    T_OPEN_PAR,
    T_CLOSE_PAR,

    // Values without const keyword
    T_INT_LIT,
    T_IDENT,
}

pub fn bin_prec(token_type: &TokenType) -> Option<u8> {
    return match token_type {
        TokenType::T_PLUS | TokenType::T_MINUS => Some(0),
        TokenType::T_FSLASH | TokenType::T_STAR => Some(1),
        _ => None,
    };
}

// Keywords (words)
pub const RETURN: Keyword = Keyword {
    syntax: "ret",
    t_type: TokenType::T_RETURN,
};

pub const INT: Keyword = Keyword {
    syntax: "int",
    t_type: TokenType::T_INT,
};

// Keywords (symbols)
pub const EQ: Keyword = Keyword {
    syntax: "=",
    t_type: TokenType::T_EQ,
};

pub const LINEEND: Keyword = Keyword {
    syntax: ";",
    t_type: TokenType::T_SEMI,
};

pub const PLUS: Keyword = Keyword {
    syntax: "+",
    t_type: TokenType::T_PLUS,
};

pub const MINUS: Keyword = Keyword {
    syntax: "-",
    t_type: TokenType::T_MINUS,
};

pub const STAR: Keyword = Keyword {
    syntax: "*",
    t_type: TokenType::T_STAR,
};

pub const FSLASH: Keyword = Keyword {
    syntax: "/",
    t_type: TokenType::T_FSLASH,
};

pub const UNDERSCORE: Keyword = Keyword {
    syntax: "_",
    t_type: TokenType::T_UNDERSCORE,
};

pub const OPEN_PAR: Keyword = Keyword {
    syntax: "(",
    t_type: TokenType::T_OPEN_PAR,
};

pub const CLOSE_PAR: Keyword = Keyword {
    syntax: ")",
    t_type: TokenType::T_CLOSE_PAR,
};

// impl fmt::Display for TokenType {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match self {
//             TokenType::RETURN => write!(f, "RETURN"),
//             TokenType::INT => write!(f, "INT"),
//             TokenType::SEMI => write!(f, "SEMI"),
//         }
//     }
// }
