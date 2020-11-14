mod mapping;

use parser::ParsingTree;

pub fn process(tree: ParsingTree) -> ParsingTree {
  mapping::complete_mappings(tree)
}
