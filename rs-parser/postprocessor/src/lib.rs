mod mapping;

use language::syntax::Program;

pub fn process(tree: Program) -> Program {
  mapping::complete_mappings(tree)
}
