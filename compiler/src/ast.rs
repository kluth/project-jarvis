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
        body: Vec<Node<'a>>,
    },
    VerifyBlock {
        tests: Vec<Node<'a>>,
    },
    Test {
        name: &'a str,
        body: Vec<Node<'a>>,
    },
}
