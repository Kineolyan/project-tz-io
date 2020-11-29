#[derive(Debug, PartialEq)]
pub struct InputMapping {
	pub from: crate::address::Port,
	pub to: u32,
}
#[derive(Debug, PartialEq)]
pub struct OutputMapping {
	pub from: u32,
	pub to: crate::address::Port,
}

// TODO: would be better with an explicit structure
pub type NodeBlock = (
	crate::address::Node,
	Vec<InputMapping>,
	Vec<OutputMapping>,
	Vec<crate::instruction::Operation>);

pub struct Program {
  pub nodes: Vec<NodeBlock>,
  pub tests: Vec<crate::test::TestCase>,
}
