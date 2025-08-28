#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int,
    Bool,
    String,
    Array(Box<Type>),
    Void,
}

#[derive(Debug, Clone)]
pub enum Expr {
    Number(i64),
    Boolean(bool),
    String(String),
    Ident(String),
    ArrayLiteral(Vec<Expr>),
    ArrayIndex {
        array: Box<Expr>,
        index: Box<Expr>,
    },
    Infix {
        left: Box<Expr>,
        op: String,
        right: Box<Expr>,
    },
    Call {
        function: String,
        args: Vec<Expr>,
    },
    Grouped(Box<Expr>),
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Let { name: String, type_annotation: Option<Type>, value: Expr },
    Assign { target: String, value: Expr },
    If {
        condition: Expr,
        then_block: Vec<Stmt>,
        else_block: Option<Vec<Stmt>>,
    },
    While {
        condition: Expr,
        body: Vec<Stmt>,
    },
    For {
        init: Box<Stmt>,
        condition: Expr,
        increment: Box<Stmt>,
        body: Vec<Stmt>,
    },
    Function {
        name: String,
        params: Vec<(String, Type)>,
        return_type: Type,
        body: Vec<Stmt>,
    },
    Return(Option<Expr>),
    Expression(Expr),
    Print(Expr),
}

#[derive(Debug, Clone)]
pub struct Program {
    pub statements: Vec<Stmt>,
}
