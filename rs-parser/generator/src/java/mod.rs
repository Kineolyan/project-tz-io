mod class;
mod constants;
mod constructs;
mod dictionary;
mod writer;

use std::cmp::Eq;
use std::collections::HashMap;
use std::path::PathBuf;

use crate::java::dictionary::Dictionary;
use language::address::Node;
use language::instruction::{MemoryPointer, Operation, ValuePointer};
use language::syntax::NodeBlock;
use language::syntax::Program;

const OBJECT_CLASS_NAME: &str = "java/lang/Object";
const STRING_CLASS_NAME: &str = "java/lang/String";
const ARRAY_LIST_CLASS_NAME: &str = "java/util/ArrayList";
const TZ_ENV_CLASS_NAME: &str = "com/kineolyan/tzio/v1/api/TzEnv";
const TZ_SYSTEM_CLASS_NAME: &str = "com/kineolyan/tzio/v1/api/arch/TzSystem";
const OPERATION_CLASS_NAME: &str = "com/kineolyan/tzio/v1/api/ops/OperationType";
const INPUT_CLASS_NAME: &str = "com/kineolyan/tzio/v1/api/ref/InputReferenceType";
const OUTPUT_CLASS_NAME: &str = "com/kineolyan/tzio/v1/api/ref/OutputReferenceType";
const REFERENCES_CLASS_NAME: &str = "com/kineolyan/tzio/v1/api/ref/References";
const OPERATION_FACADE_CLASS_NAME: &str = "com/kineolyan/tzio/v1/api/ops/Operations";

type SlotIndex = HashMap<usize, Vec<u32>>;

#[derive(Debug, PartialEq, Hash)]
struct NodeSlot<'a>(&'a str, u32, &'a str, u32);
impl<'a> Eq for NodeSlot<'a> {}

#[derive(Debug)]
struct SlotStructure {
    count: u32,
    node_inputs: SlotIndex,
    node_outputs: SlotIndex,
    input_indexes: Vec<u32>,
    output_indexes: Vec<u32>,
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
    instructions: &mut Vec<constructs::Operation>,
) {
    match value_pointer {
        ValuePointer::ACC => {
            let acc_method_idx = class.map_method(
                REFERENCES_CLASS_NAME,
                if input { "inAcc" } else { "outAcc" },
                &constructs::Signature {
                    return_type: constants::Type::Object(String::from(if input {
                        INPUT_CLASS_NAME
                    } else {
                        OUTPUT_CLASS_NAME
                    })),
                    parameter_types: vec![],
                },
            );
            instructions.push(constructs::Operation::invokestatic(acc_method_idx));
        }
        ValuePointer::NIL => {
            let nil_method_idx = class.map_method(
                REFERENCES_CLASS_NAME,
                if input { "inNil" } else { "outNil" },
                &constructs::Signature {
                    return_type: constants::Type::Object(String::from(if input {
                        INPUT_CLASS_NAME
                    } else {
                        OUTPUT_CLASS_NAME
                    })),
                    parameter_types: vec![],
                },
            );
            instructions.push(constructs::Operation::invokestatic(nil_method_idx));
        }
        ValuePointer::VALUE(ref value) => {
            let cst_idx = class.map_integer(*value);
            let value_method_idx = class.map_method(
                REFERENCES_CLASS_NAME,
                "value",
                &constructs::Signature {
                    return_type: constants::Type::Object(String::from(INPUT_CLASS_NAME)),
                    parameter_types: vec![constants::Type::Integer],
                },
            );
            instructions.push(constructs::Operation::ldc(cst_idx));
            instructions.push(constructs::Operation::invokestatic(value_method_idx));
        }
        ValuePointer::PORT(ref id) => {
            let cst_idx = class.map_integer(*id);
            let value_method_idx = class.map_method(
                REFERENCES_CLASS_NAME,
                if input { "inSlot" } else { "outSlot" },
                &constructs::Signature {
                    return_type: constants::Type::Object(String::from(if input {
                        INPUT_CLASS_NAME
                    } else {
                        OUTPUT_CLASS_NAME
                    })),
                    parameter_types: vec![constants::Type::Integer],
                },
            );
            instructions.push(constructs::Operation::ldc(cst_idx));
            instructions.push(constructs::Operation::invokestatic(value_method_idx));
        }
    }
}

fn create_slot_indexes(tree: &Program) -> SlotStructure {
    let mut s = SlotStructure {
        count: 0,
        node_inputs: HashMap::new(),
        node_outputs: HashMap::new(),
        input_indexes: vec![],
        output_indexes: vec![],
    };
    let mut slots: Dictionary<NodeSlot> = Dictionary::new();

    for (i, node) in tree.nodes.iter().enumerate() {
        let node_name = node.0.get_id();
        let mut ins = Vec::new();
        for input in &node.1 {
            let input_name = match &input.from.node {
                Node::Node(ref id) => id,
                Node::In => "<IN>",
                _ => panic!("Unexpect input node {:?} for {:?}", input.from.node, node),
            };
            let node_slot = NodeSlot(input_name, input.from.port, node_name, input.to);
            let dic_idx = slots.map(node_slot) as u32;
            ins.push(dic_idx);
            if let Node::In = &input.from.node {
                s.input_indexes.push(dic_idx);
            }
        }
        s.node_inputs.insert(i, ins);

        let mut outs = Vec::new();
        for output in &node.2 {
            let output_name = match &output.to.node {
                Node::Node(ref id) => id,
                Node::Out => "<OUT>",
                _ => panic!("Unexpect input node {:?} for {:?}", output.to.node, node),
            };
            let node_slot = NodeSlot(node_name, output.from, output_name, output.to.port);
            let dic_idx = slots.map(node_slot) as u32;
            outs.push(dic_idx);
            if let Node::Out = &output.to.node {
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
    values: &[u32],
    var_idx: u8,
) -> constructs::Attribute {
    let array_size = class.map_integer(values.len() as u32);
    let mut operations = vec![
        constructs::Operation::ldc(array_size),
        constructs::Operation::newarray(constants::ArrayType::INT),
        constructs::Operation::astore(var_idx),
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

    constructs::Attribute::Code {
        max_stack: 3,
        operations,
        locals: (var_idx as u16) + 1,
    }
}

fn create_operation_array(
    class: &mut class::JavaClass,
    operations: &[Operation],
    var_idx: u8,
) -> constructs::Attribute {
    // Create the array for the operations and store it as the var
    let arraylist_class_idx = class.map_class(ARRAY_LIST_CLASS_NAME);
    let list_cstr_idx = class.map_method(
        ARRAY_LIST_CLASS_NAME,
        "<init>",
        &constructs::Signature {
            return_type: constants::Type::Void,
            parameter_types: vec![],
        },
    );
    let mut instructions = vec![
        constructs::Operation::new(arraylist_class_idx),
        constructs::Operation::dup,
        // this could be improved by creating an arraylist of the correct size
        constructs::Operation::invokespecial(list_cstr_idx),
        constructs::Operation::astore(var_idx),
    ];
    let add_to_list_idx = class.map_method(
        ARRAY_LIST_CLASS_NAME,
        "add",
        &constructs::Signature {
            return_type: constants::Type::Boolean,
            parameter_types: vec![constants::Type::Object(String::from(OBJECT_CLASS_NAME))],
        },
    );

    // Create each operation
    for operation in operations {
        // Load the array
        instructions.push(constructs::Operation::aload(var_idx));
        // Construct the operation object
        match operation {
            Operation::MOV(ref from_pointer, ref to_pointer) => {
                create_mov_operation(class, from_pointer, to_pointer, &mut instructions);
            }
            Operation::SAV(ref mem_pointer) => {
                create_memory_operation(class, "SAV", mem_pointer, &mut instructions);
            }
            Operation::SWP(ref mem_pointer) => {
                create_memory_operation(class, "SWP", mem_pointer, &mut instructions);
            }
            Operation::ADD(ref value_pointer) => {
                create_math_operation(class, "ADD", value_pointer, &mut instructions);
            }
            Operation::SUB(ref value_pointer) => {
                create_math_operation(class, "SUB", value_pointer, &mut instructions);
            }
            Operation::NEG => {
                let method_idx = class.map_method(
                    OPERATION_FACADE_CLASS_NAME,
                    "NEG",
                    &constructs::Signature {
                        return_type: constants::Type::Object(String::from(OPERATION_CLASS_NAME)),
                        parameter_types: vec![],
                    },
                );
                instructions.push(constructs::Operation::invokestatic(method_idx));
            }
            Operation::LABEL(ref name) => {
                create_labeled_operation(class, "LABEL", name, &mut instructions);
            }
            Operation::JMP(ref name) => {
                create_labeled_operation(class, "JMP", name, &mut instructions);
            }
            Operation::JEZ(ref name) => {
                create_labeled_operation(class, "JEZ", name, &mut instructions);
            }
            Operation::JNZ(ref name) => {
                create_labeled_operation(class, "JNZ", name, &mut instructions);
            }
            Operation::JLZ(ref name) => {
                create_labeled_operation(class, "JLZ", name, &mut instructions);
            }
            Operation::JGZ(ref name) => {
                create_labeled_operation(class, "JGZ", name, &mut instructions);
            }
            Operation::JRO(ref value_pointer) => {
                create_jro_operation(class, value_pointer, &mut instructions);
            }
        }
        // Add the operation to the list
        instructions.push(constructs::Operation::invokevirtual(add_to_list_idx));
    }

    let locals = constructs::count_local_vars(None, &instructions);
    constructs::Attribute::Code {
        max_stack: 10,
        operations: instructions,
        locals,
    }
}

fn create_mov_operation(
    class: &mut class::JavaClass,
    from_pointer: &ValuePointer,
    to_pointer: &ValuePointer,
    instructions: &mut Vec<constructs::Operation>,
) {
    let method_idx = class.map_method(
        OPERATION_FACADE_CLASS_NAME,
        "MOV",
        &constructs::Signature {
            return_type: constants::Type::Object(String::from(OPERATION_CLASS_NAME)),
            parameter_types: vec![
                constants::Type::Object(String::from(INPUT_CLASS_NAME)),
                constants::Type::Object(String::from(OUTPUT_CLASS_NAME)),
            ],
        },
    );
    create_reference_instructions(class, from_pointer, true, instructions);
    create_reference_instructions(class, to_pointer, false, instructions);
    instructions.push(constructs::Operation::invokestatic(method_idx));
}

fn create_memory_operation(
    class: &mut class::JavaClass,
    constructor_method: &str,
    mem_pointer: &MemoryPointer,
    instructions: &mut Vec<constructs::Operation>,
) {
    let method_idx = class.map_method(
        OPERATION_FACADE_CLASS_NAME,
        constructor_method,
        &constructs::Signature {
            return_type: constants::Type::Object(String::from(OPERATION_CLASS_NAME)),
            parameter_types: vec![constants::Type::Integer],
        },
    );
    let &MemoryPointer::BAK(i) = mem_pointer;
    let memory_slot: i8 = u8_to_i8(i);
    instructions.push(constructs::Operation::bipush(memory_slot));
    instructions.push(constructs::Operation::invokestatic(method_idx));
}

fn create_math_operation(
    class: &mut class::JavaClass,
    constructor_method: &str,
    value_pointer: &ValuePointer,
    instructions: &mut Vec<constructs::Operation>,
) {
    let method_idx = class.map_method(
        OPERATION_FACADE_CLASS_NAME,
        constructor_method,
        &constructs::Signature {
            return_type: constants::Type::Object(String::from(OPERATION_CLASS_NAME)),
            parameter_types: vec![constants::Type::Object(String::from(INPUT_CLASS_NAME))],
        },
    );

    // Generate the input reference
    create_reference_instructions(class, value_pointer, true, instructions);

    // Generate the operation
    instructions.push(constructs::Operation::invokestatic(method_idx));
}

fn create_labeled_operation(
    class: &mut class::JavaClass,
    constructor_method: &str,
    label: &str,
    instructions: &mut Vec<constructs::Operation>,
) {
    let method_idx = class.map_method(
        OPERATION_FACADE_CLASS_NAME,
        constructor_method,
        &constructs::Signature {
            return_type: constants::Type::Object(String::from(OPERATION_CLASS_NAME)),
            parameter_types: vec![constants::Type::Object(String::from(STRING_CLASS_NAME))],
        },
    );
    let label_idx = class.map_string(label);
    instructions.push(constructs::Operation::ldc(label_idx));
    instructions.push(constructs::Operation::invokestatic(method_idx));
}

fn create_jro_operation(
    class: &mut class::JavaClass,
    value_pointer: &ValuePointer,
    instructions: &mut Vec<constructs::Operation>,
) {
    let method_idx = class.map_method(
        OPERATION_FACADE_CLASS_NAME,
        "JRO",
        &constructs::Signature {
            return_type: constants::Type::Object(String::from(OPERATION_CLASS_NAME)),
            parameter_types: vec![constants::Type::Object(String::from(INPUT_CLASS_NAME))],
        },
    );
    create_reference_instructions(class, value_pointer, true, instructions);
    instructions.push(constructs::Operation::invokestatic(method_idx));
}

pub fn create_main_file(tree: &Program, package: &str, output_dir: &PathBuf) -> Result<(), String> {
    let slots = create_slot_indexes(tree);
    let mut class = class::JavaClass::new();

    let mut classname = String::from("com/kineolyan/tzio/");
    classname.push_str(package);
    classname.push_str("/Main");
    class.set_class(&classname);

    class.set_super_class(&OBJECT_CLASS_NAME);

    let mut definition_methods: Vec<class::PoolIdx> = vec![];
    for (i, node) in tree.nodes.iter().enumerate() {
        let pool_idx = create_node_definition_method(i, node, &mut class, &slots);
        definition_methods.push(pool_idx);
    }

    let create_idx = create_construction(&mut class, &definition_methods, &slots);
    create_main(&mut class, create_idx);

    let mut output_file = output_dir.clone();
    output_file.push("Main");
    output_file.set_extension("class");
    writer::write(&class, output_file.as_path())
        .map_err(|e| format!("Failed to write into file. Caused by {}", e))
}

/// Create a static method
fn create_node_definition_method(
    i: usize,
    node: &NodeBlock,
    class: &mut class::JavaClass,
    slots: &SlotStructure,
) -> class::PoolIdx {
    let add_node_idx = class.map_interface_method(
        &TZ_ENV_CLASS_NAME,
        "addNode",
        &constructs::Signature {
            return_type: constants::Type::Object(String::from(TZ_ENV_CLASS_NAME)),
            parameter_types: vec![
                constants::Type::Object(String::from(STRING_CLASS_NAME)),
                constants::Type::Integer,
                constants::Type::PrimitiveArray(1, constants::ArrayType::INT),
                constants::Type::PrimitiveArray(1, constants::ArrayType::INT),
                constants::Type::Object(String::from("java/util/List")),
            ],
        },
    );

    let signature = constructs::Signature {
        return_type: constants::Type::Object(String::from(TZ_ENV_CLASS_NAME)),
        parameter_types: vec![constants::Type::Object(String::from(TZ_ENV_CLASS_NAME))],
    };

    let node_name = class.map_string(&node.0.get_id());
    let input_array_var_idx = 1;
    let create_input_array = create_int_array(
        class,
        &slots
            .node_inputs
            .get(&i)
            .unwrap_or_else(|| panic!("No inputs for node {}", i)),
        input_array_var_idx,
    );
    let output_array_var_idx = 2;
    let create_output_array = create_int_array(
        class,
        &slots
            .node_outputs
            .get(&i)
            .unwrap_or_else(|| panic!("No outputs for node {}", i)),
        output_array_var_idx,
    );
    let operation_array_var_idx = 3;
    let create_op_array = create_operation_array(class, &node.3, operation_array_var_idx);
    let call_to_add_node = vec![
        constructs::Operation::aload(0),                       // first arg
        constructs::Operation::ldc(node_name),                 // node name
        constructs::Operation::iconst_1,                       // node memory size
        constructs::Operation::aload(input_array_var_idx),     // input array
        constructs::Operation::aload(output_array_var_idx),    // output array
        constructs::Operation::aload(operation_array_var_idx), // operation array
        constructs::Operation::invokeinterface(add_node_idx, 6),
        constructs::Operation::areturn,
    ];

    let access: u16 =
        (constants::MethodAccess::STATIC as u16) | (constants::MethodAccess::PRIVATE as u16);

    let mut method_name = String::from("createNode");
    method_name.push_str(&(i as u32).to_string());

    let method_code = constructs::merge_codes(
        Some(&signature),
        vec![
            create_input_array,
            create_output_array,
            create_op_array,
            constructs::Attribute::Code {
                max_stack: 6,
                locals: constructs::count_local_vars(None, &call_to_add_node),
                operations: call_to_add_node,
            },
        ],
    );

    class.create_method(access, &method_name, signature, vec![method_code])
}

fn create_construction(
    class: &mut class::JavaClass,
    definition_methods: &[class::PoolIdx],
    slots: &SlotStructure,
) -> class::PoolIdx {
    let get_instance_idx = class.map_interface_method(
        TZ_SYSTEM_CLASS_NAME,
        &"getInstance",
        &constructs::Signature {
            return_type: constants::Type::Object(String::from(TZ_SYSTEM_CLASS_NAME)),
            parameter_types: vec![],
        },
    );
    let create_env_idx = class.map_interface_method(
        TZ_SYSTEM_CLASS_NAME,
        &"createEnv",
        &constructs::Signature {
            return_type: constants::Type::Object(String::from(TZ_ENV_CLASS_NAME)),
            parameter_types: vec![],
        },
    );
    let create_env_ops = vec![
        constructs::Operation::invokestatic(get_instance_idx),
        constructs::Operation::invokeinterface(create_env_idx, 1),
    ];
    let create_env_code = constructs::Attribute::Code {
        max_stack: 1,
        locals: constructs::count_local_vars(None, &create_env_ops),
        operations: create_env_ops,
    };

    let create_input_array_op = create_int_array(class, &slots.input_indexes, 1);
    let create_output_array_op = create_int_array(class, &slots.output_indexes, 2);

    let slot_count_cst = class.map_integer(slots.count);
    let with_slots_idx = get_with_slots_idx(class);
    let with_slots_op = vec![
        // TzEnv already on the top of the stack
        constructs::Operation::ldc(slot_count_cst), // slot count
        constructs::Operation::aload(1),            // inputs array
        constructs::Operation::aload(2),            // output array
        constructs::Operation::invokeinterface(with_slots_idx, 4), // Method returning the TzEnv instance
    ];

    let mut create_nodes_op = Vec::new();
    for idx in definition_methods {
        // Call each definition static method
        // TzEnv already on the top of the stack
        create_nodes_op.push(constructs::Operation::invokestatic(*idx));
        // Method returning the TzEnv instance
    }

    // Complete the function by returning the TzEnv
    create_nodes_op.push(constructs::Operation::areturn);

    let this_signature = constructs::Signature {
        return_type: constants::Type::Object(String::from(TZ_ENV_CLASS_NAME)),
        parameter_types: vec![],
    };

    let access: u16 =
        constants::MethodAccess::PRIVATE as u16 | constants::MethodAccess::STATIC as u16;

    let method_code = constructs::merge_codes(
        Some(&this_signature),
        vec![
            create_env_code,
            create_input_array_op,
            create_output_array_op,
            constructs::Attribute::Code {
                max_stack: 5,
                locals: constructs::count_local_vars(None, &with_slots_op),
                operations: with_slots_op,
            },
            constructs::Attribute::Code {
                max_stack: 2,
                locals: constructs::count_local_vars(None, &create_nodes_op),
                operations: create_nodes_op,
            },
        ],
    );

    class.create_method(access, &"create", this_signature, vec![method_code])
}

fn create_main(class: &mut class::JavaClass, creator_idx: class::PoolIdx) -> class::PoolIdx {
    let signature = constructs::Signature {
        return_type: constants::Type::Void,
        parameter_types: vec![constants::Type::ObjectArray(
            1,
            String::from("java/lang/String"),
        )],
    };

    let run_from_idx = get_run_from_system_idx(class);
    let operations = vec![
        constructs::Operation::invokestatic(creator_idx),
        // Call 'runFromSystem' with main parameter array
        constructs::Operation::aload(0),
        constructs::Operation::invokeinterface(run_from_idx, 2),
        constructs::Operation::iconst_m1,
        constructs::Operation::nop,
        constructs::Operation::return_void,
    ];

    let access: u16 =
        (constants::MethodAccess::STATIC as u16) | (constants::MethodAccess::PUBLIC as u16);

    let local_count = constructs::count_local_vars(Some(&signature), &operations);
    class.create_method(
        access,
        &"main",
        signature,
        vec![constructs::Attribute::Code {
            max_stack: 3,
            operations,
            locals: local_count,
        }],
    )
}

fn get_with_slots_idx(class: &mut class::JavaClass) -> class::PoolIdx {
    class.map_interface_method(
        TZ_ENV_CLASS_NAME,
        &"withSlots",
        &constructs::Signature {
            return_type: constants::Type::Object(String::from(TZ_ENV_CLASS_NAME)),
            parameter_types: vec![
                constants::Type::Integer,
                constants::Type::PrimitiveArray(1, constants::ArrayType::INT),
                constants::Type::PrimitiveArray(1, constants::ArrayType::INT),
            ],
        },
    )
}

fn get_run_from_system_idx(class: &mut class::JavaClass) -> class::PoolIdx {
    class.map_interface_method(
        &TZ_ENV_CLASS_NAME,
        &"runFromSystem",
        &constructs::Signature {
            return_type: constants::Type::Void,
            parameter_types: vec![constants::Type::ObjectArray(
                1,
                String::from("java/lang/String"),
            )],
        },
    )
}
