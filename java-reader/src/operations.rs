use pool::{PoolList, resolve_method_name, resolve_class_name};
use printer::{print_bytes};
use reader::{Reader, ByteReader, ReadResult, to_u16};
use types::{ARRAY_TYPES};

fn print_op(name: &str) {
  println!("[{}]", name);
}

pub fn read(
    reader: &mut ByteReader, 
    pool: &PoolList,
    indent: u8) -> ReadResult {
  while !reader.is_empty() {
    read_u8!(operation_code, reader, indent);
    match operation_code {
      2 ... 8 => read_iconst(operation_code),
      11 ... 13 => read_fconst(operation_code),
      18 => read_ldc(reader, indent)?,
      25 => read_aload(reader, indent)?,
      58 => read_astore(reader, indent)?,
      79 => read_iastore(),
      89 => read_dup()?,
      182 => read_invoke_virtual(reader, pool, indent)?,
      183 => read_invoke_special(reader, pool, indent)?,
      184 => read_invoke_static(reader, pool, indent)?,
      187 => read_new(reader, pool, indent)?,
      188 => read_new_array(reader, indent)?,
      _ => panic!("Unsupported operation: {}", operation_code)
    }
  }
  Ok(())
}

fn read_aload(reader: &mut Reader, indent: u8) -> ReadResult {
  print_op("aload");
  read_u8!(var_idx, reader, indent + 1);
  println!("load from var#{}", var_idx);

  Ok(())
}

fn read_astore(reader: &mut Reader, indent: u8) -> ReadResult {
  print_op("astore");
  read_u8!(var_idx, reader, indent + 1);
  println!("store into var#{}", var_idx);

  Ok(())
}

fn read_iastore() {
  print_op("iastore");
}

fn read_fconst(operation: u8) {
  let num = operation as i8 - 11;
  match num {
    0 ... 2 => print_op(&format!("fconst_{}", num)),
    _ => panic!("Invalid float constant opcode {}", num)
  }
}

fn read_iconst(operation: u8) {
  let num = operation as i8 - 3;
  match num {
    -1 => print_op("iconst_m1"),
    0 ... 5 => print_op(&format!("iconst_{}", num)),
    _ => panic!("Invalid integer constant opcode {}", num)
  }
}

macro_rules! read_invoke {
  ($method_name: ident, $name: expr) => {
    fn $method_name(reader: &mut Reader, pool: &PoolList, indent: u8) -> ReadResult {
      print_op($name);
      read_u16!(method_idx, reader, indent + 1);
      let (ref class_name, ref method_name, ref descriptor) = resolve_method_name(pool, method_idx as usize)
        .expect(&format!("No method reference in pool at {}", method_idx));
      println!("Invoke #{} = {}.{}:{}", method_idx, class_name, method_name, descriptor);
      Ok(())
    }
  };
}
read_invoke!(read_invoke_virtual, "invokevirtual");
read_invoke!(read_invoke_special, "invokespecial");
read_invoke!(read_invoke_static, "invokestatic");

fn read_new(reader: &mut Reader, pool: &PoolList, indent: u8) -> ReadResult {
  print_op("new");
  read_u16!(class_idx, reader, indent + 1);
  let class_name = resolve_class_name(pool, class_idx as usize)
    .expect(&format!("No class name in pool at {}", class_idx));
  println!("New {}", class_name);

  Ok(())
}

fn read_new_array(reader: &mut Reader, indent: u8) -> ReadResult {
  print_op("newarray");
  read_u8!(array_type, reader, indent + 1);
  let type_name = ARRAY_TYPES.get(&array_type)
    .expect(&format!("No array type with code {}", array_type));
  println!("new array of {}", type_name);

  Ok(())
}

fn read_ldc(reader: &mut Reader, indent: u8) -> ReadResult {
  print_op("ldc");
  read_u8!(idx, reader, indent + 1);
  println!("load constant #{}", idx);
  Ok(())
}

fn read_dup() -> ReadResult {
  print_op("dup");
  Ok(())
}
