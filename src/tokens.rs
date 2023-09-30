// use core::fmt;

// also operators
#[derive(Debug)]
pub struct Keyword<'a> {
    pub syntax: &'a str,
    pub t_type: TokenType,
}

#[derive(Debug)]
pub struct Token {
    pub t_type: TokenType,
    pub value: Option<String>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum TokenType {
    T_RETURN,
    T_SEMI,
    // Values without const keyword
    T_INT,
}

pub const RETURN: Keyword = Keyword {
    syntax: "ret",
    t_type: TokenType::T_RETURN,
};
pub const LINEEND: Keyword = Keyword {
    syntax: ";",
    t_type: TokenType::T_SEMI,
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
