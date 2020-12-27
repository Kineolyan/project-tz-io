use std::collections::HashMap;

#[derive(Debug, PartialEq, Default)]
pub struct TestCase {
    pub ins: HashMap<u32, Vec<i8>>,
    pub outs: HashMap<u32, Vec<i8>>,
}

impl TestCase {
    pub fn inputInto(mut self, input_slot: u32, values: Vec<i8>) -> Self {
        self.ins.insert(input_slot, values);
        self
    }

    pub fn outputFrom(mut self, output_slot: u32, values: Vec<i8>) -> Self {
        self.outs.insert(output_slot, values);
        self
    }
}
