#![allow(non_camel_case_types)]
use std::rc::Rc;

#[derive(Debug)]
pub struct NodeProg {
    pub stmts: Vec<Rc<NodeStmt>>,
}

impl NodeProg {
    pub fn new() -> NodeProg {
        NodeProg { stmts: Vec::new() }
    }
}

/* #region Statments */
#[derive(Default, Debug)]
pub enum VarStmt {
    #[default]
    NONE,
    RET(Rc<NodeStmtRet>),
    ASSIGN(Rc<NodeStmtAssign>),
    // ASSIGN2(NodeStmtAssign),
}

#[derive(Debug)]
pub struct NodeStmt {
    pub var: VarStmt,
}

impl NodeStmt {
    pub fn new() -> NodeStmt {
        NodeStmt {
            var: Default::default(),
        }
    }
}

#[derive(Debug)]
pub struct NodeStmtRet {
    pub expr: Rc<NodeExpr>,
}

impl NodeStmtRet {
    pub fn new() -> NodeStmtRet {
        NodeStmtRet {
            expr: NodeExpr::new().into(),
        }
    }
}

#[derive(Debug)]
pub struct NodeStmtAssign {
    pub var_type: Rc<NodeType>,
    pub ident: String,
    pub expr: Rc<NodeExpr>,
}

impl NodeStmtAssign {
    pub fn new() -> NodeStmtAssign {
        NodeStmtAssign {
            var_type: Default::default(),
            ident: String::new(),
            expr: NodeExpr::new().into(),
        }
    }
}
/* #endregion */

/* #region Expressions */
#[derive(Default, Debug)]
pub enum VarExpr {
    #[default]
    NONE,
    TERM(Rc<NodeTerm>),
    BIN(Rc<NodeBinExpr>),
}

#[derive(Debug)]
pub struct NodeExpr {
    pub var: VarExpr,
}

impl NodeExpr {
    pub fn new() -> NodeExpr {
        NodeExpr {
            var: Default::default(),
        }
    }
}
/* #endregion */

/* #region Terms */
#[derive(Default, Debug)]
pub enum VarTerm {
    #[default]
    NONE,
    INT_LIT(Rc<NodeTermIntLit>),
    IDENT(Rc<NodeTermIdent>),
    NEG(Rc<NodeTermNeg>),
    PAR(Rc<NodeTermPar>),
}

#[derive(Debug)]
pub struct NodeTerm {
    pub var: VarTerm,
}

impl NodeTerm {
    pub fn new() -> NodeTerm {
        NodeTerm {
            var: Default::default(),
        }
    }
}

#[derive(Debug)]
pub struct NodeTermIntLit {
    pub value: String,
}

impl NodeTermIntLit {
    pub fn new() -> NodeTermIntLit {
        NodeTermIntLit {
            value: Default::default(),
        }
    }
}

#[derive(Debug)]
pub struct NodeTermIdent {
    pub ident: String,
}

impl NodeTermIdent {
    pub fn new() -> NodeTermIdent {
        NodeTermIdent {
            ident: String::new(),
        }
    }
}

#[derive(Debug)]
pub struct NodeTermNeg {
    pub term: Rc<NodeTerm>,
}

impl NodeTermNeg {
    pub fn new() -> NodeTermNeg {
        NodeTermNeg {
            term: NodeTerm::new().into(),
        }
    }
}

#[derive(Debug)]
pub struct NodeTermPar {
    pub expr: Rc<NodeExpr>,
}

impl NodeTermPar {
    pub fn new() -> NodeTermPar {
        NodeTermPar {
            expr: NodeExpr::new().into(),
        }
    }
}
/* #endregion */

/* #region Binary Expresions */
#[derive(Debug)]
pub struct NodeBinExpr {
    pub var: VarBinExpr,
    pub lhs: Rc<NodeExpr>,
    pub rhs: Rc<NodeExpr>,
}

impl NodeBinExpr {
    pub fn new() -> NodeBinExpr {
        NodeBinExpr {
            var: Default::default(),
            lhs: NodeExpr::new().into(),
            rhs: NodeExpr::new().into(),
        }
    }
}

#[derive(Debug, Default)]
pub enum VarBinExpr {
    #[default]
    NONE,
    ADD,
    SUB,
    MUL,
    DIV,
}
/* #endregion */

/* #region Types */
#[derive(Default, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum NodeType {
    #[default]
    NONE,
    N_INT,
}
/* #endregion */

// #[derive(Debug)]
// pub struct NodeType {
//     pub var: VarType,
//     pub value: String,
// }

// impl NodeType {
//     pub fn new() -> NodeType {
//         NodeType {
//             var: Default::default(),
//             value: String::new(),
//         }
//     }
// }

// #[derive(Debug)]
// pub struct TypeInt {
//     pub
// }

// pub struct NodeStmt {
//     NodeStmtRet,
//     NodeStmtAssign {
//         pub n_type: NodeType,
//         pub expr: NodeExpr,
//     },
// }

// pub struct NodeStmtRet {
//     pub expr: NodeExpr,
// }

// pub enum NodeExpr {
//     NodeExprBinExpr {},
// }
