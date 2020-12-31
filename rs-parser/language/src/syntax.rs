#[derive(Debug, PartialEq)]
pub struct InputMapping {
	pub from: crate::address::Port<crate::address::OutputSlot>,
	pub to: crate::address::InputSlot,
}
#[derive(Debug, PartialEq)]
pub struct OutputMapping {
	pub from: crate::address::OutputSlot,
	pub to: crate::address::Port<crate::address::InputSlot>,
}

// TODO: would be better with an explicit structure
pub type NodeBlock = (
	crate::address::Node,
	Vec<InputMapping>,
	Vec<OutputMapping>,
	Vec<crate::instruction::Operation>);

pub struct Program {
  pub nodes: Vec<NodeBlock>,
  pub tests: Option<crate::test::TestCase>,
}
