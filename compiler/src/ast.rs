#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Type {
    I32,
    F32,
    Stream,
    PixelStream,
    FrameBuffer,
    VectorCanvas,
    Unknown,
}

#[derive(Debug, PartialEq)]
pub enum Expr<'a> {
    NumberLiteral(&'a str),
    StringLiteral(&'a str),
    Identifier(&'a str),
    Call {
        name: &'a str,
        args: Vec<Expr<'a>>,
    },
    Assignment {
        name: &'a str,
        value: Box<Expr<'a>>,
    },
    BinaryOp {
        left: Box<Expr<'a>>,
        op: &'a str,
        right: Box<Expr<'a>>,
    },
    Input,
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
    Sync {
        protocol: &'a str,
        body: Vec<Stmt<'a>>,
    },
    Gossip {
        target: &'a str,
    },
    Contract {
        spec: &'a str,
    },
    Knowledge {
        name: &'a str,
        dim: usize,
    },
    Publish {
        target: &'a str,
    },
    Window {
        title: &'a str,
        width: usize,
        height: usize,
    },
    Event {
        kind: &'a str,
        body: Vec<Stmt<'a>>,
    },
    Assert {
        condition: Expr<'a>,
    },
    Layout {
        kind: &'a str,
        content: Vec<Stmt<'a>>,
    },
    Component {
        kind: &'a str,
        args: Vec<Expr<'a>>,
    },
    Poll,
    CaptureFrame,
    CaptureStream,
    Print {
        value: Expr<'a>,
        x: Expr<'a>,
        y: Expr<'a>,
        color: Expr<'a>,
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
        params: Vec<&'a str>,
        return_ty: Option<Type>,
        body: Vec<Stmt<'a>>,
        verification: Option<Box<Node<'a>>>,
    },
    VerifyBlock {
        tests: Vec<Node<'a>>,
    },
    Test {
        name: &'a str,
        body: Vec<Stmt<'a>>,
    },
    Render {
        name: &'a str,
        params: Vec<&'a str>,
        body: Vec<Node<'a>>,
        verification: Option<Box<Node<'a>>>,
    },
    Layout {
        kind: &'a str,
        content: Vec<Node<'a>>,
    },
    Component {
        kind: &'a str,
        args: Vec<Expr<'a>>,
    },
    Poll,
}
