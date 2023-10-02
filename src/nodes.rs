#![allow(non_camel_case_types)]
use std::rc::Rc;

#[derive(Debug)]
pub struct NodeProg {
    pub stmts: Vec<NodeStmt>,
}

impl NodeProg {
    pub fn new() -> NodeProg {
        NodeProg { stmts: Vec::new() }
    }
}

#[derive(Default, Debug)]
pub enum VarStmt {
    #[default]
    NONE,
    RET(NodeStmtRet),
    ASSIGN(NodeStmtAssign),
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
    pub expr: NodeExpr,
}

impl NodeStmtRet {
    pub fn new() -> NodeStmtRet {
        NodeStmtRet {
            expr: NodeExpr::new(),
        }
    }
}

#[derive(Debug)]
pub struct NodeStmtAssign {
    pub var_type: Rc<NodeType>,
    pub ident: String,
    pub expr: NodeExpr,
}

impl NodeStmtAssign {
    pub fn new() -> NodeStmtAssign {
        NodeStmtAssign {
            var_type: Default::default(),
            ident: String::new(),
            expr: NodeExpr::new(),
        }
    }
}

#[derive(Default, Debug)]
pub enum VarExpr {
    #[default]
    NONE,
    TERM(NodeTerm),
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

#[derive(Default, Debug)]
pub enum VarTerm {
    #[default]
    NONE,
    INT_LIT(NodeTermIntLit),
    IDENT(NodeTermIdent),
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

#[derive(Default, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum NodeType {
    #[default]
    NONE,
    N_INT,
}

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
