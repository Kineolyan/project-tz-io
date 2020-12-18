use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::io;

use crate::java::class::{
  JavaClass,
  PoolElement
};
use crate::java::constants;
use crate::java::constructs::{
  Attribute,
  Operation
};

static MAGIC: [u8; 4] = [0xca_u8, 0xfe_u8, 0xba_u8, 0xbe_u8];
static VERSIONS: [u8; 4] = [/* minor */0, 0, /* major */0, 53]; // Java 9

// static DEBUG: bool = true;
// macro_rules! debug {
//   ($fmt:expr) => {
//     if DEBUG {
//       println!($fmt);
//     }
//    };
//   ($fmt:expr, $($arg:tt)*) => {
//     if DEBUG {
//       println!($fmt, $($arg)*);
//     }
//    };
// }

type StdResult = io::Result<()>;

trait Writer {
  fn write(&mut self, data: &[u8]) -> StdResult;
}

struct FileWriter {
  file: File
}

impl FileWriter {
  fn flush(&mut self) -> StdResult {
    self.file.flush()
  }
}

impl Writer for FileWriter {
  fn write(&mut self, data: &[u8]) -> StdResult {
    self.file.write_all(data)
  }
}

struct VecWriter {
  pub data: Vec<u8>
}

impl Writer for VecWriter {
  fn write(&mut self, data: &[u8]) -> StdResult {
    self.data.reserve_exact(data.len());
    for byte in data {
      self.data.push(*byte);
    }
    Ok(())
  }
}

fn write_u8(writer: &mut Writer, value: u8) -> StdResult {
  let buf: [u8; 1] = [value];
  writer.write(&buf)
}

fn write_u16(writer: &mut Writer, value: u16) -> StdResult {
  let buf: [u8; 2] = [
    (value >> 8) as u8,
    (value & 0xff) as u8
  ];
  writer.write(&buf)
}

fn write_u32(writer: &mut Writer, value: u32) -> StdResult {
  let buf: [u8; 4] = [
    (value >> 24) as u8,
    ((value >> 16) & 0xff) as u8,
    ((value >> 8) & 0xff) as u8,
    (value & 0xff) as u8
  ];
  writer.write(&buf)
}

fn write_string(writer: &mut Writer, value: &String) -> StdResult {
  let bytes = value.as_bytes();
  if !bytes.iter().all(|c| 0 < *c && *c < 128) {
    panic!("Unsupported chars in the string: `{}`", value);
  }
  writer.write(bytes)
}

fn write_header(writer: &mut Writer) -> StdResult {
  writer.write(&MAGIC)?;
  writer.write(&VERSIONS)
}

fn write_constant_pool(writer: &mut Writer, class: &JavaClass) -> StdResult {
  let pool_size = class.pool_size();
  write_u16(writer, pool_size)?;

  for (_idx, element) in class.pool_iter() {
    match element {
      &PoolElement::Utf8Value(ref value) => {
        write_u8(writer, constants::PoolCode::Utf8 as u8)?;
        write_u16(writer, value.len() as u16)?;
        write_string(writer, value)?;
      },
      &PoolElement::ClassInfo(c_idx) => {
        write_u8(writer, constants::PoolCode::Class as u8)?;
        write_u16(writer, c_idx)?;
      },
      &PoolElement::Integer(value) => {
        write_u8(writer, constants::PoolCode::Integer as u8)?;
        write_u32(writer, value)?;
      },
      &PoolElement::String(value_idx) => {
        write_u8(writer, constants::PoolCode::String as u8)?;
        write_u16(writer, value_idx)?;
      },
      &PoolElement::MethodRef(class_idx, name_idx) => {
        write_u8(writer, constants::PoolCode::MethodRef as u8)?;
        write_u16(writer, class_idx)?;
        write_u16(writer, name_idx)?;
      },
      &PoolElement::InterfaceMethodRef(class_idx, name_idx) => {
        write_u8(writer, constants::PoolCode::InterfaceMethodRef as u8)?;
        write_u16(writer, class_idx)?;
        write_u16(writer, name_idx)?;
      },
      &PoolElement::NameAndType(name_idx, descriptor_idx) => {
        write_u8(writer, constants::PoolCode::NameAndType as u8)?;
        write_u16(writer, name_idx)?;
        write_u16(writer, descriptor_idx)?;
      }
    }
  }
  Ok(())
}

fn write_class_info(writer: &mut Writer, class: &JavaClass) -> StdResult {
  let access: u16 = (constants::ClassAccess::FINAL as u16)
    | (constants::ClassAccess::SUPER as u16);
  write_u16(writer, access)?;
  write_u16(writer, class.class_id)?;
  write_u16(writer, class.super_class_id)?;

  // For now, tell that there are no interfaces
  write_u16(writer, class.interfaces.len() as u16)?;
  for interface_id in &class.interfaces {
    write_u16(writer, *interface_id)?;
  }
  Ok(())
}

fn write_class_definition(writer: &mut Writer, class: &JavaClass) -> StdResult {
  // TODO write the correct writer
  // No fields
  write_u16(writer, 0)?;

  // Write methods
  write_u16(writer, class.methods.len() as u16)?;
  for method in &class.methods {
    write_u16(writer, method.access)?;
    write_u16(writer, method.name_index)?;
    write_u16(writer, method.descriptor_index)?;
    write_u16(writer, method.attributes.len() as u16)?;
    for entry in &method.attributes {
      write_attribute(writer, entry, class)?;
    }
  }

  // No attributes
  write_u16(writer, 0)
}

fn write_attribute(
    writer: &mut Writer,
    &(ref idx, ref attribute): &(u16, Attribute),
    class: &JavaClass) -> StdResult {
  match attribute {
    &Attribute::Code {ref max_stack, ref operations, ref locals} => {
      write_u16(writer, *idx)?;

      let mut op_writer = VecWriter { data: Vec::new() };
      for operation in operations {
        write_operation(&mut op_writer, operation, class)?;
      }

      let mut attr_writer = VecWriter { data: Vec::new() };
      write_u16(&mut attr_writer, *max_stack)?;
      write_u16(&mut attr_writer, *locals)?;
      write_u32(&mut attr_writer, op_writer.data.len() as u32)?;
      attr_writer.write(&op_writer.data[..])?;

      // Not used so far
      write_u16(&mut attr_writer, 0)?; // No exception tables
      write_u16(&mut attr_writer, 0)?; // No attributes

      write_u32(writer, attr_writer.data.len() as u32)?;
      writer.write(&attr_writer.data[..])
    }
  }
}

fn write_operation(
    writer: &mut Writer,
    operation: &Operation,
    class: &JavaClass) -> StdResult {
  match operation {
    &Operation::aload(ref idx) => {
      // if idx > 3 { // TODO write the optimization
      write_u8(writer, 25)?;
      write_u8(writer, *idx)
    },
    &Operation::aconst_null => {
      write_u8(writer, 1)
    },
    &Operation::areturn => {
      write_u8(writer, 176)
    },
    &Operation::astore(ref idx) => {
      write_u8(writer, 58)?;
      write_u8(writer, *idx)
    },
    &Operation::bipush(ref value) => {
      write_u8(writer, 16)?;
      write_u8(writer, *value as u8)
    },
    &Operation::dup => {
      write_u8(writer, 89)
    },
    &Operation::iastore => {
      write_u8(writer, 79)
    },
    &Operation::iconst_1 => {
      write_u8(writer, 4)
    },
    &Operation::iconst_m1 => {
      write_u8(writer, 2)
    },
    &Operation::invokespecial(ref idx) => {
      write_u8(writer, 183)?;
      write_u16(writer, *idx)
    },
    &Operation::invokevirtual(ref idx) => {
      write_u8(writer, 182)?;
      write_u16(writer, *idx)
    },
    &Operation::invokestatic(ref idx) => {
      write_u8(writer, 184)?;
      write_u16(writer, *idx)
    },
    &Operation::invokedynamic(ref idx) => {
      write_u8(writer, 185)?;
      write_u16(writer, *idx)?;
      write_u8(writer, 0)?; // 3rd value must be 0
      write_u8(writer, 0) // 4th value must be 0
    },
    &Operation::invokeinterface(ref idx, ref count) => {
      write_u8(writer, 185)?;
      write_u16(writer, *idx)?;

      // Get the number of args for the count
      write_u8(writer, *count)?; // FIXME retrieve this from the class

      write_u8(writer, 0) // 4th value must be 0
    },
    &Operation::ldc(ref idx) => {
      // Optimize using ldc or ldc_w
      if *idx < 256 {
        write_u8(writer, 18)?;
        write_u8(writer, (*idx & 0xff) as u8)
      } else {
        write_u8(writer, 19)?;
        write_u16(writer, *idx)
      }
    },
    &Operation::new(ref idx) => {
      write_u8(writer, 187)?;
      write_u16(writer, *idx)
    },
    &Operation::newarray(ref array_type) => {
      write_u8(writer, 188)?;
      write_u8(writer, array_type.clone() as u8)
    },
    &Operation::nop => {
      write_u8(writer, 0)
    },
    &Operation::return_void => {
      write_u8(writer, 177)
    }
  }
}

pub fn write(class: &JavaClass, output_file: &Path) -> StdResult {
  // println!("class: {:?}", class);
  let mut writer = FileWriter { file: File::create(output_file)? };
  write_header(&mut writer)?;
  writer.flush()?;
  write_constant_pool(&mut writer, class)?;
  writer.flush()?;
  write_class_info(&mut writer, class)?;
  writer.flush()?;
  write_class_definition(&mut writer, class)?;
  writer.flush()
}