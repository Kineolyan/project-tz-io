#[derive(Debug, PartialEq)]
pub struct TestCase {
    pub ins: Vec<i8>,
    pub outs: Vec<i8>,
}

impl TestCase {
    pub fn new(ins: Vec<i8>, outs: Vec<i8>) -> TestCase {
        TestCase { ins, outs }
    }
}
