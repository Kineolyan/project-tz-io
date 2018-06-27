mod dictionary;
mod class;
mod writer;
mod constants;
mod constructs;

use std::cmp::Eq;
use std::path::PathBuf;
use std::collections::HashMap;

use parser::ParsingTree;
use parser::syntax::NodeBlock;
use parser::address::Node;
use parser::instruction::{Operation, MemoryPointer, ValuePointer};
use generator::java::dictionary::Dictionary;

const STRING_CLASS_NAME: &str = "java/lang/String";
const TZ_ENV_CLASS_NAME: &str = "com/kineolyan/tzio/v1/TzEnv";
const ARRAY_LIST_CLASS_NAME: &str = "java/util/ArrayList";
const OPERATION_FACADE_CLASS_NAME: &str = "com/kineolyan/tzio/v1/ops/Operations";
const OPERATION_CLASS_NAME: &str = "com/kineolyan/tzio/v1/ops/Operation";
const REFERENCES_CLASS_NAME: &str = "com/kineolyan/tzio/v1/ref/References";
const INPUT_CLASS_NAME: &str = "com/kineolyan/tzio/v1/ref/InputReference";
const OUTPUT_CLASS_NAME: &str = "com/kineolyan/tzio/v1/ref/OutputReference";

type SlotIndex = HashMap<usize, Vec<u32>>;
#[derive(Debug, PartialEq, Hash)]
struct NodeSlot<'a>(&'a str, u32, &'a str, u32);
impl <'a> Eq for NodeSlot<'a> {}
struct SlotStructure {
  count: u32,
  node_inputs: SlotIndex,
  node_outputs: SlotIndex,
  input_indexes: Vec<u32>,
  output_indexes: Vec<u32>
}

fn u8_to_i8(value: u8) -> i8 {
  if value < 128 {
    value as i8
  } else {
    panic!("Memory pointer index out of range: {}", value);
  }
}

fn create_reference_instructions(
    class: &mut class::JavaClass,
    value_pointer: &ValuePointer,
    input: bool,
    instructions: &mut Vec<constructs::Operation>) {
  match value_pointer {
    &ValuePointer::ACC => {
      let acc_method_idx = class.map_method(
        REFERENCES_CLASS_NAME, 
        "acc", 
        &constructs::Signature {
          return_type: constants::Type::Object(String::from(INPUT_CLASS_NAME)),
          parameter_types: vec![]
        });
      instructions.push(constructs::Operation::invokestatic(acc_method_idx));
    },
    &ValuePointer::NIL => {
      let nil_method_idx = class.map_method(
        REFERENCES_CLASS_NAME, 
        "NIL", 
        &constructs::Signature {
          return_type: constants::Type::Object(String::from(INPUT_CLASS_NAME)),
          parameter_types: vec![]
        });
      instructions.push(constructs::Operation::invokestatic(nil_method_idx));
    },
    &ValuePointer::VALUE(ref value) => {
      let cst_idx = class.map_integer(*value);
      let value_method_idx = class.map_method(
        REFERENCES_CLASS_NAME, 
        "value", 
        &constructs::Signature {
          return_type: constants::Type::Object(String::from(INPUT_CLASS_NAME)),
          parameter_types: vec![
            constants::Type::Integer
          ]
        });
      instructions.push(constructs::Operation::ldc(cst_idx));
      instructions.push(constructs::Operation::invokestatic(value_method_idx));
    },
    &ValuePointer::PORT(ref id) => {
      let cst_idx = class.map_integer(*id);
      let value_method_idx = class.map_method(
        REFERENCES_CLASS_NAME, 
        if input { "inSlot" } else { "outSlot" }, 
        &constructs::Signature {
          return_type: constants::Type::Object(
            String::from(
              if input { INPUT_CLASS_NAME } else { OUTPUT_CLASS_NAME })),
          parameter_types: vec![
            constants::Type::Integer
          ]
        });
      instructions.push(constructs::Operation::ldc(cst_idx));
      instructions.push(constructs::Operation::invokestatic(value_method_idx));
    }
  }
}

fn create_slot_indexes(tree: &ParsingTree) -> SlotStructure {
  let mut s = SlotStructure {
    count: 0,
    node_inputs: HashMap::new(), 
    node_outputs: HashMap::new(),
    input_indexes: vec![],
    output_indexes: vec![]
  };
  let mut slots: Dictionary<NodeSlot> = Dictionary::new();

  for (i, node) in tree.iter().enumerate() {
    let node_name = node.0.get_id();
    let mut ins = Vec::new();
    for input in &node.1 {
      let input_name = match &input.from.node {
        &Node::Node(ref id) => id,
        &Node::In => "<IN>",
        _ => panic!("Unexpect input node {:?} for {:?}", input.from.node, node)
      };
      let node_slot = NodeSlot(
        input_name,
        input.from.port,
        node_name,
        input.to);
      let dic_idx = slots.map(node_slot) as u32;
      ins.push(dic_idx);
      if let &Node::In = &input.from.node {
        s.input_indexes.push(dic_idx);
      }
    }
    s.node_inputs.insert(i, ins);

    let mut outs = Vec::new();
    for output in &node.2 {
      let output_name = match &output.to.node {
        &Node::Node(ref id) => id,
        &Node::Out => "<OUT>",
        _ => panic!("Unexpect input node {:?} for {:?}", output.to.node, node)
      };
      let node_slot = NodeSlot(
        node_name,
        output.from,
        output_name,
        output.to.port);
      let dic_idx = slots.map(node_slot) as u32;
      outs.push(dic_idx);
      if let &Node::Out = &output.to.node {
        s.output_indexes.push(dic_idx);
      }
    }
    s.node_outputs.insert(i, outs);
  }

  s.count = slots.size() as u32;
  s
}

fn create_int_array(
    class: &mut class::JavaClass,
    values: &Vec<u32>, 
    var_idx: u8) -> constructs::Attribute {
  let array_size = class.map_integer(values.len() as u32);
  let mut operations = vec![
    constructs::Operation::ldc(array_size),
    constructs::Operation::newarray(constants::ArrayType::INT),
    constructs::Operation::astore(var_idx)
  ];
  for (i, value) in values.iter().enumerate() {
    let value_idx = class.map_integer(*value);
    let index_idx = class.map_integer(i as u32);

    // Add value to array
    operations.push(constructs::Operation::aload(var_idx));
    operations.push(constructs::Operation::ldc(index_idx));
    operations.push(constructs::Operation::ldc(value_idx));
    operations.push(constructs::Operation::iastore);
  }

  constructs::Attribute::Code(3, operations)
}

fn create_operation_array(
    class: &mut class::JavaClass,
    operations: &Vec<Operation>, 
    var_idx: u8) -> constructs::Attribute {
  // Create the array for the operations and store it as the var
  let arraylist_class_idx = class.map_class(ARRAY_LIST_CLASS_NAME);
  let list_cstr_idx = class.map_method(
    ARRAY_LIST_CLASS_NAME, 
    "<init>", 
    &constructs::Signature {
      return_type: constants::Type::Void,
      parameter_types: vec![]
    });
  let mut instructions = vec![
    constructs::Operation::new(arraylist_class_idx),
    constructs::Operation::dup,
    // this could be improved by creating an arraylist of the correct size
    constructs::Operation::invokespecial(list_cstr_idx),
    constructs::Operation::astore(var_idx)
  ];
  let add_to_list_idx = class.map_method(
    ARRAY_LIST_CLASS_NAME, 
    "add", 
    &constructs::Signature {
      return_type: constants::Type::Void,
      parameter_types: vec![
        constants::Type::Object(String::from("java/lang/Object"))
      ]
    });

  // Create each operation
  for operation in operations {
    // Load the array
    instructions.push(constructs::Operation::aload(var_idx));
    // Construct the operation object
    match operation {
      &Operation::MOV(ref from_pointer, ref to_pointer) => {
        create_mov_operation(class, from_pointer, to_pointer, &mut instructions);
      },
      &Operation::SAV(ref mem_pointer) => {
        create_memory_operation(class, "SAV", mem_pointer, &mut instructions);
      },
      &Operation::SWP(ref mem_pointer) => {
        create_memory_operation(class, "SWP", mem_pointer, &mut instructions);
      },
      &Operation::ADD(ref value_pointer) => {
        create_math_operation(class, "ADD", value_pointer, &mut instructions);
      },
      &Operation::SUB(ref value_pointer) => {
        create_math_operation(class, "ADD", value_pointer, &mut instructions);
      },
      &Operation::NEG => {
        let method_idx = class.map_method(
          OPERATION_FACADE_CLASS_NAME, 
          "NEG", 
          &constructs::Signature {
            return_type: constants::Type::Object(String::from(OPERATION_CLASS_NAME)),
            parameter_types: vec![]
          });
        instructions.push(constructs::Operation::invokestatic(method_idx));
      },
      &Operation::LABEL(ref name)  => {
        create_labeled_operation(class, "LABEL", name, &mut instructions);
      },
      &Operation::JMP(ref name) => {
        create_labeled_operation(class, "JMP", name, &mut instructions);
      },
      &Operation::JEZ(ref name) => {
        create_labeled_operation(class, "JEZ", name, &mut instructions);
      },
      &Operation::JNZ(ref name) => {
        create_labeled_operation(class, "JNZ", name, &mut instructions);
      },
      &Operation::JLZ(ref name) => {
        create_labeled_operation(class, "JLZ", name, &mut instructions);
      },
      &Operation::JGZ(ref name) => {
        create_labeled_operation(class, "JGZ", name, &mut instructions);
      },
      &Operation::JRO(ref value_pointer) => {
        create_jro_operation(class, value_pointer, &mut instructions);
      }
    }
    // Add the operation to the list
    instructions.push(constructs::Operation::invokevirtual(add_to_list_idx));
  }
  constructs::Attribute::Code(10, instructions)
}

fn create_mov_operation(
    class: &mut class::JavaClass,
    from_pointer: &ValuePointer,
    to_pointer: &ValuePointer,
    instructions: &mut Vec<constructs::Operation>) {
  let method_idx = class.map_method(
    OPERATION_FACADE_CLASS_NAME, 
    "MOV", 
    &constructs::Signature {
      return_type: constants::Type::Object(String::from(OPERATION_CLASS_NAME)),
      parameter_types: vec![
        constants::Type::Object(String::from(INPUT_CLASS_NAME)),
        constants::Type::Object(String::from(OUTPUT_CLASS_NAME))
      ]
    });
  create_reference_instructions(class, from_pointer, true, instructions);
  create_reference_instructions(class, to_pointer, false, instructions);
  instructions.push(constructs::Operation::invokestatic(method_idx));
}

fn create_memory_operation(
    class: &mut class::JavaClass,
    constructor_method: &str,
    mem_pointer: &MemoryPointer,
    instructions: &mut Vec<constructs::Operation>) {
  let method_idx = class.map_method(
    OPERATION_FACADE_CLASS_NAME, 
    constructor_method, 
    &constructs::Signature {
      return_type: constants::Type::Object(String::from(OPERATION_CLASS_NAME)),
      parameter_types: vec![
        constants::Type::Integer
      ]
    });
  let &MemoryPointer::BAK(i) = mem_pointer;
  let memory_slot: i8 = u8_to_i8(i);
  instructions.push(constructs::Operation::bipush(memory_slot.into()));
  instructions.push(constructs::Operation::invokestatic(method_idx));
}

fn create_math_operation(
    class: &mut class::JavaClass,
    constructor_method: &str,
    value_pointer: &ValuePointer,
    instructions: &mut Vec<constructs::Operation>) {
  let method_idx = class.map_method(
    OPERATION_FACADE_CLASS_NAME, 
    constructor_method, 
    &constructs::Signature {
      return_type: constants::Type::Object(String::from(OPERATION_CLASS_NAME)),
      parameter_types: vec![
        constants::Type::Object(String::from(INPUT_CLASS_NAME))
      ]
    });

  // Generate the input reference
  create_reference_instructions(class, value_pointer, true, instructions);

  // Generate the operation
  instructions.push(constructs::Operation::invokestatic(method_idx));
}

fn create_labeled_operation(
    class: &mut class::JavaClass,
    constructor_method: &str,
    label: &str,
    instructions: &mut Vec<constructs::Operation>) {
  let method_idx = class.map_method(
    OPERATION_FACADE_CLASS_NAME, 
    constructor_method, 
    &constructs::Signature {
      return_type: constants::Type::Object(String::from(OPERATION_CLASS_NAME)),
      parameter_types: vec![
        constants::Type::Object(String::from(STRING_CLASS_NAME))
      ]
    });
  let label_idx = class.map_string(label);
  instructions.push(constructs::Operation::ldc(label_idx));
  instructions.push(constructs::Operation::invokestatic(method_idx));
}

fn create_jro_operation(
    class: &mut class::JavaClass,
    value_pointer: &ValuePointer,
    instructions: &mut Vec<constructs::Operation>) {
  let method_idx = class.map_method(
    OPERATION_FACADE_CLASS_NAME, 
    "JRO", 
    &constructs::Signature {
      return_type: constants::Type::Object(String::from(OPERATION_CLASS_NAME)),
      parameter_types: vec![
        constants::Type::Object(String::from(INPUT_CLASS_NAME))
      ]
    });
  create_reference_instructions(class, value_pointer, true, instructions);
  instructions.push(constructs::Operation::invokestatic(method_idx));
}

pub fn create_main_file(
    tree: &ParsingTree, 
    package: &str,
    output_dir: &PathBuf) -> Result<(), String> {
  let slots = create_slot_indexes(tree);
  let mut class = class::JavaClass::new();

  let mut classname = String::from("com/kineolyan/tzio/");
  classname.push_str(package);
  classname.push_str("/Main");
  class.set_class(&classname);

  class.set_super_class(&TZ_ENV_CLASS_NAME);

  let mut definition_methods: Vec<class::PoolIdx> = vec![];
  for (i, node) in tree.iter().enumerate() {
    let pool_idx = create_node_definition_method(i, node, &mut class);
    definition_methods.push(pool_idx);
  }

  let init_idx = create_constructor(&mut class, &definition_methods, &slots);
  create_main(&mut class, init_idx);

  let mut output_file = output_dir.clone();
  output_file.push("Main");
  output_file.set_extension("class");
  writer::write(&class, output_file.as_path())
    .map_err(|e| format!("Failed to write into file. Caused by {}", e))
}

fn create_node_definition_method(
    i: usize,
    node: &NodeBlock,
    class: &mut class::JavaClass) -> class::PoolIdx {
  let add_node_idx = class.map_method(
    &TZ_ENV_CLASS_NAME, 
    "addNode", 
    &constructs::Signature {
      return_type: constants::Type::Object(String::from(TZ_ENV_CLASS_NAME)),
      parameter_types: vec![
        constants::Type::Object(String::from(STRING_CLASS_NAME)),
        constants::Type::Integer,
        constants::Type::PrimitiveArray(1, constants::ArrayType::INT),
        constants::Type::PrimitiveArray(1, constants::ArrayType::INT),
        constants::Type::Object(String::from("java/util/List"))
      ]
    });
  
  let signature = constructs::Signature {
    return_type: constants::Type::Void,
    parameter_types: vec![]
  };
  
  let node_name = class.map_string(&node.0.get_id());
  let create_input_array = create_int_array(class, &vec![0, 1], 1);
  let create_output_array = create_int_array(class, &vec![1, 2], 2);
  let create_op_array = create_operation_array(class, &node.3, 3);
  let call_to_add_node = vec![
    constructs::Operation::aload(0),
    constructs::Operation::ldc(node_name), // node name
    constructs::Operation::iconst_1, // node memory size
    constructs::Operation::aload(1), // input array
    constructs::Operation::aload(2), // output array
    constructs::Operation::aload(3), // operation array
    constructs::Operation::invokevirtual(add_node_idx),
    constructs::Operation::return_void
  ];

  let access: u16 = 
    (constants::MethodAccess::FINAL as u16) |
    (constants::MethodAccess::PRIVATE as u16);

  let mut method_name = String::from("createNode");
  method_name.push_str(&(i as u32).to_string());

  let method_code = constructs::merge_codes(vec![
      create_input_array,
      create_output_array,
      create_op_array,
      constructs::Attribute::Code(6, call_to_add_node)
    ]);

  class.create_method(
    access,
    &method_name,
    signature,
    vec![method_code])
}

fn create_constructor(
    class: &mut class::JavaClass, 
    definition_methods: &Vec<class::PoolIdx>,
    slots: &SlotStructure) -> class::PoolIdx {
  let super_init_idx = class.map_method(
    TZ_ENV_CLASS_NAME,
    &"<init>",
    &constructs::Signature {
      return_type: constants::Type::Void,
      parameter_types: vec![]
    });
  let obj_init_op = constructs::Attribute::Code(
    1,
    vec![
      constructs::Operation::aload(0),
      constructs::Operation::invokespecial(super_init_idx)
    ]);
  let create_input_array_op = create_int_array(class, &slots.input_indexes, 1);
  let create_output_array_op = create_int_array(class, &slots.output_indexes, 2);

  let slot_count_cst = class.map_integer(slots.count);
  let with_slots_idx = get_with_slots_idx(class);
  let with_slots_op = vec![
    constructs::Operation::aload(0),
    constructs::Operation::ldc(slot_count_cst), // slot count
    constructs::Operation::aload(1), // inputs array
    constructs::Operation::aload(2), // output array
    constructs::Operation::invokevirtual(with_slots_idx)
  ];

  let mut create_nodes_op = Vec::new(); 
  for idx in definition_methods {
    // Call each definition private method
    create_nodes_op.push(constructs::Operation::aload(0));
    create_nodes_op.push(constructs::Operation::invokespecial(*idx));
  }

  // Complete the function with a return
  create_nodes_op.push(constructs::Operation::return_void);

  let this_signature = constructs::Signature {
    return_type: constants::Type::Void,
    parameter_types: vec![]
  };

  let access: u16 = constants::MethodAccess::PUBLIC as u16;

  let method_code = constructs::merge_codes(vec![
    obj_init_op,
    create_input_array_op,
    create_output_array_op,
    constructs::Attribute::Code(5, with_slots_op),
    constructs::Attribute::Code(1, create_nodes_op)
  ]);

  class.create_method(
    access,
    &"<init>",
    this_signature,
    vec![method_code])
}

fn create_main(class: &mut class::JavaClass, init_idx: class::PoolIdx) -> class::PoolIdx {
  let signature = constructs::Signature {
    return_type: constants::Type::Void,
    parameter_types: vec![
      constants::Type::ObjectArray(1, String::from("java/lang/String"))
    ]
  };

  let this_class_idx = class.class_id;
  let run_from_idx = get_run_from_idx(class);
  let operations = vec![
    // Create a new instance of this class
    constructs::Operation::new(this_class_idx),
    // Init the new instance
    constructs::Operation::dup,
    constructs::Operation::invokespecial(init_idx),
    // Call 'runFromSystem' with main parameter array
    constructs::Operation::aload(0),
    constructs::Operation::invokevirtual(run_from_idx),
    constructs::Operation::return_void
  ];

  let access: u16 = 
    (constants::MethodAccess::STATIC as u16) |
    (constants::MethodAccess::PUBLIC as u16);

  class.create_method(
    access,
    &"main",
    signature,
    vec![
      constructs::Attribute::Code(3, operations)
    ])
} 

fn get_with_slots_idx(class: &mut class::JavaClass) -> class::PoolIdx {
  let class_name: String;
  {
    class_name = class.get_class_name().expect("No class name set yet");
  }

  class.map_method(
    TZ_ENV_CLASS_NAME,
    &"withSlots", 
    &constructs::Signature {
      return_type: constants::Type::Object(class_name),
      parameter_types: vec![
        constants::Type::Integer,
        constants::Type::PrimitiveArray(
          1,
          constants::ArrayType::INT),
        constants::Type::PrimitiveArray(
          1,
          constants::ArrayType::INT)
      ]
    })
}

fn get_run_from_idx(class: &mut class::JavaClass) -> class::PoolIdx {
  class.map_method(
    &TZ_ENV_CLASS_NAME, 
    &"runFromSystem",
    &constructs::Signature {
      return_type: constants::Type::Void,
      parameter_types: vec![
        constants::Type::ObjectArray(1, String::from("java/lang/String"))
      ]
    })
}