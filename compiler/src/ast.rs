#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Type {
    I32,
    F32,
    Stream,
    Unknown,
}

#[derive(Debug, PartialEq)]
pub enum Expr<'a> {
    NumberLiteral(&'a str),
    StringLiteral(&'a str),
    Identifier(&'a str),
    BinaryOp {
        left: Box<Expr<'a>>,
        op: &'a str,
        right: Box<Expr<'a>>,
    },
}

#[derive(Debug, PartialEq)]
pub enum Stmt<'a> {
    Let {
        name: &'a str,
        ty: Option<Type>,
        value: Expr<'a>,
    },
    Return {
        value: Option<Expr<'a>>,
    },
    Expression {
        expr: Expr<'a>,
    },
    If {
        condition: Expr<'a>,
        then_branch: Vec<Stmt<'a>>,
        else_branch: Option<Vec<Stmt<'a>>>,
    },
    While {
        condition: Expr<'a>,
        body: Vec<Stmt<'a>>,
    },
    Memory {
        name: &'a str,
        ty: Type,
        size: usize,
    },
    Evolve {
        body: Vec<Stmt<'a>>,
    },
    Budget {
        limit: f32,
        body: Vec<Stmt<'a>>,
    },
    Prob {
        branches: Vec<(f32, Vec<Stmt<'a>>)>,
    },
}

#[derive(Debug, PartialEq)]
pub enum Node<'a> {
    Module {
        name: &'a str,
        body: Vec<Node<'a>>,
    },
    ComplexityBlock {
        complexity: &'a str,
        content: Vec<Node<'a>>,
    },
    Function {
        name: &'a str,
        params: Vec<&'a str>, // simplified
        return_ty: Option<Type>,
        body: Vec<Stmt<'a>>,
    },
    VerifyBlock {
        tests: Vec<Node<'a>>,
    },
    Test {
        name: &'a str,
        body: Vec<Stmt<'a>>,
    },
}
