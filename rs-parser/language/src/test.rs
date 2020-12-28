use std::collections::HashMap;

#[derive(Debug, PartialEq, Default)]
pub struct TestCase {
    pub ins: HashMap<u32, Vec<i8>>,
    pub outs: HashMap<u32, Vec<i8>>,
}

impl TestCase {
    pub fn input_into(mut self, input_slot: u32, values: Vec<i8>) -> Self {
        self.ins.insert(input_slot, values);
        self
    }

    pub fn output_from(mut self, output_slot: u32, values: Vec<i8>) -> Self {
        self.outs.insert(output_slot, values);
        self
    }
}
