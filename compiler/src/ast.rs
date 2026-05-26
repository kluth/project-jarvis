#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Type<'a> {
    I32,
    F32,
    Stream,
    PixelStream,
    FrameBuffer,
    VectorCanvas,
    Struct(&'a str),
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
    UnaryOp {
        op: &'a str,
        expr: Box<Expr<'a>>,
    },
    FieldAccess {
        object: Box<Expr<'a>>,
        field: &'a str,
    },
    Input,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Attribute<'a> {
    Interrupt(&'a str),
    NoMangle,
    Section(&'a str),
}

#[derive(Debug, PartialEq)]
pub enum Stmt<'a> {
    Let {
        name: &'a str,
        ty: Option<Type<'a>>,
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
    For {
        var: &'a str,
        iterable: Expr<'a>,
        body: Vec<Stmt<'a>>,
    },
    Memory {
        name: &'a str,
        ty: Type<'a>,
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
        width: i32,
        height: i32,
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
    Print {
        value: Expr<'a>,
        x: Expr<'a>,
        y: Expr<'a>,
        color: Expr<'a>,
    },
    CaptureFrame,
    CaptureStream,
    Asm {
        block: &'a str,
    },
    VolatileWrite {
        address: Expr<'a>,
        value: Expr<'a>,
    },
    VolatileRead {
        address: Expr<'a>,
        dest: &'a str,
    },
    PortWrite {
        port: Expr<'a>,
        value: Expr<'a>,
    },
    PortRead {
        port: Expr<'a>,
        dest: &'a str,
    },
    AtomicOp {
        op: &'a str,
        args: Vec<Expr<'a>>,
    },
    Hologram {
        kind: &'a str,
        depth: Expr<'a>,
        body: Vec<Stmt<'a>>,
    },
    PostProcess {
        effect: &'a str,
        intensity: Expr<'a>,
        body: Vec<Stmt<'a>>,
    },
    NeuroAdapt {
        load: Expr<'a>,
        body: Vec<Stmt<'a>>,
    },
}

#[derive(Debug, PartialEq)]
pub enum Node<'a> {
    Module {
        name: &'a str,
        body: Vec<Node<'a>>,
    },
    Import {
        path: &'a str,
    },
    ComplexityBlock {
        complexity: &'a str,
        content: Vec<Node<'a>>,
    },
    Struct {
        name: &'a str,
        fields: Vec<(&'a str, Type<'a>)>,
    },
    Function {
        name: &'a str,
        params: Vec<&'a str>,
        return_ty: Option<Type<'a>>,
        body: Vec<Stmt<'a>>,
        verification: Option<Box<Node<'a>>>,
        attributes: Vec<Attribute<'a>>,
    },
    Static {
        name: &'a str,
        address: usize,
        size: usize,
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
    Allocator {
        name: &'a str,
        body: Vec<Stmt<'a>>,
    },
    Hologram {
        kind: &'a str,
        depth: Expr<'a>,
        content: Vec<Node<'a>>,
    },
    PostProcess {
        effect: &'a str,
        intensity: Expr<'a>,
        content: Vec<Node<'a>>,
    },
    NeuroAdapt {
        load: Expr<'a>,
        content: Vec<Node<'a>>,
    },
}