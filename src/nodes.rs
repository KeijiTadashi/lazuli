use std::default;

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
    // pub fn parse_stmt(){
    //     match var {
    //         RET(n) =>
    //     }
    // }
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
    pub expr: NodeExpr,
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
    INT_LIT(NodeTermInt),
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
pub struct NodeTermInt {
    pub value: String,
}

impl NodeTermInt {
    pub fn new() -> NodeTermInt {
        NodeTermInt {
            value: Default::default(),
        }
    }
}

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
