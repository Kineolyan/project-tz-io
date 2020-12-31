use std::collections::HashMap;

use crate::address::{InputSlot, OutputSlot};

#[derive(Debug, PartialEq, Default)]
pub struct TestCase {
    // Notice that inputs are indexed by output slots, because they are considered
    // as the ouputs of a fictious node
    pub ins: HashMap<crate::address::OutputSlot, Vec<i8>>,
    pub outs: HashMap<crate::address::InputSlot, Vec<i8>>,
}

impl TestCase {
    pub fn input_into(mut self, input_slot: OutputSlot, values: Vec<i8>) -> Self {
        self.ins.insert(input_slot, values);
        self
    }

    pub fn output_from(mut self, output_slot: InputSlot, values: Vec<i8>) -> Self {
        self.outs.insert(output_slot, values);
        self
    }
}
