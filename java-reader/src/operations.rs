use pool::{PoolList, resolve_method_name, resolve_class_name, resolve_field_name};
use printer::{print_bytes};
use reader::{Reader, ByteReader, ReadResult, to_u16};
use types::{ARRAY_TYPES};

fn print_op(name: &str) {
  println!("[{}]", name);
}

fn are_same_decimal(before: usize, now: usize) -> bool {
  let b = before / 10;
  let n = now / 10;
  b == n
}

pub fn read(
    reader: &mut ByteReader,
    pool: &PoolList,
    indent: u8) -> ReadResult {
  let mut last_size: usize = 0;
  while !reader.is_empty() {
    read_u8!(operation_code, reader, indent);
    if !are_same_decimal(last_size, reader.get_pos()) {
      print!("<{}> ", reader.get_pos());
    }
    last_size = reader.get_pos();

    match operation_code {
      0 => read_nop(),
      1 => read_aconst_null(),
      2 ... 8 => read_iconst(operation_code),
      9 | 10 => read_lconst_n(operation_code),
      11 ... 13 => read_fconst_n(operation_code),
      16 => read_bipush(reader, indent)?,
      17 => read_sipush(reader, indent)?,
      18 => read_ldc(reader, pool, indent)?,
      20 => read_ldc2_w(reader, pool, indent)?,
      25 => read_aload(reader, indent)?,
      26 ... 29 => read_iload_n(operation_code),
      42 ... 45 => read_aload_n(operation_code),
      58 => read_astore(reader, indent)?,
      75 ... 78 => read_astore_n(operation_code),
      79 => read_iastore(),
      80 => read_lastore(),
      83 => read_aastore(),
      87 => read_pop(),
      88 => read_pop2(),
      89 => read_dup(),
      176 => read_areturn(),
      177 => read_return(),
      178 => read_get_static(reader, pool, indent)?,
      181 => read_put_field(reader, pool, indent)?,
      182 => read_invoke_virtual(reader, pool, indent)?,
      183 => read_invoke_special(reader, pool, indent)?,
      184 => read_invoke_static(reader, pool, indent)?,
      185 => read_invoke_interface(reader, pool, indent)?,
      186 => read_invoke_dynamic(reader, pool, indent)?,
      187 => read_new(reader, pool, indent)?,
      188 => read_new_prim_array(reader, indent)?,
      189 => read_new_object_array(reader, pool, indent)?,
      _ => panic!("Unsupported operation: {}", operation_code)
    }
  }
  Ok(())
}

macro_rules! shortcut_op {
  ($f: ident, $name: tt, $base: tt, $to: tt) => {
    fn $f(operation: u8) {
      let num = operation as i8 - $base;
      match num {
        0 ... $to => print_op(&format!("{}_{}", $name, num)),
        _ => panic!("Invalid {}_n opcode {}", $name, num)
      }
    }
  };
}

macro_rules! single_op {
  ($f: ident, $name: tt) => {
    fn $f() {
      print_op($name);
    }
  }
}

single_op!(read_aconst_null, "aconst_null");

fn read_aload(reader: &mut Reader, indent: u8) -> ReadResult {
  print_op("aload");
  read_u8!(var_idx, reader, indent + 1);
  println!("load from var#{}", var_idx);

  Ok(())
}
shortcut_op!(read_aload_n, "aload", 42, 3);

fn read_astore(reader: &mut Reader, indent: u8) -> ReadResult {
  print_op("astore");
  read_u8!(var_idx, reader, indent + 1);
  println!("store into var#{}", var_idx);

  Ok(())
}
shortcut_op!(read_astore_n, "astore", 75, 3);

single_op!(read_iastore, "iastore");
single_op!(read_lastore, "lastore");
single_op!(read_aastore, "aastore");

fn read_bipush(reader: &mut Reader, indent: u8) -> ReadResult {
  print_op("bipush");
  read_u8!(value, reader, indent + 1);
  let value: i32 = value.into();
  println!("Push int {} to stack", value);
  Ok(())
}

fn read_sipush(reader: &mut Reader, indent: u8) -> ReadResult {
  print_op("sipush");
  read_u16!(value, reader, indent + 1);
  let value: i32 = value.into();
  println!("Push int {} to stack", value);
  Ok(())
}

shortcut_op!(read_fconst_n, "fconst", 11, 2);

fn read_iconst(operation: u8) {
  let num = operation as i8 - 3;
  match num {
    -1 => print_op("iconst_m1"),
    0 ... 5 => print_op(&format!("iconst_{}", num)),
    _ => panic!("Invalid integer constant opcode {}", num)
  }
}

shortcut_op!(read_iload_n, "iload", 26, 3);

shortcut_op!(read_lconst_n, "lconst", 9, 1);

single_op!(read_areturn, "areturn");
single_op!(read_return, "return");

single_op!(read_nop, "nop");

fn read_get_static(reader: &mut Reader, pool: &PoolList, indent: u8) -> ReadResult {
  print_op("getstatic");
  read_u16!(field_idx, reader, indent + 1);
  let (ref class_name, ref field_name, ref descriptor) = resolve_field_name(pool, field_idx as usize)
    .expect(&format!("No field reference in pool at {}", field_idx));
  println!("Get static field #{} -> {}.{}:{}", field_idx, class_name, field_name, descriptor);
  Ok(())
}

fn read_put_field(reader: &mut Reader, pool: &PoolList, indent: u8) -> ReadResult {
  print_op("putfield");
  read_u16!(field_idx, reader, indent + 1);
  let (ref class_name, ref field_name, ref descriptor) = resolve_field_name(pool, field_idx as usize)
    .expect(&format!("No field reference in pool at {}", field_idx));
  println!("Put field #{} -> {}.{}:{}", field_idx, class_name, field_name, descriptor);
  Ok(())
}

/// Macro interpreting invoke of standard methods
macro_rules! read_std_invoke {
  ($method_name: ident, $name: expr) => {
    fn $method_name(reader: &mut Reader, pool: &PoolList, indent: u8) -> ReadResult {
      print_op($name);
      read_u16!(method_idx, reader, indent + 1);
      let (ref class_name, ref method_name, ref descriptor) = resolve_method_name(pool, method_idx as usize)
        .expect(&format!("No method reference in pool at {}", method_idx));
      println!("Invoke #{} -> {}.{}:{}", method_idx, class_name, method_name, descriptor);
      Ok(())
    }
  };
}
read_std_invoke!(read_invoke_virtual, "invokevirtual");
read_std_invoke!(read_invoke_special, "invokespecial");
read_std_invoke!(read_invoke_static, "invokestatic");

fn read_invoke_interface(reader: &mut Reader, pool: &PoolList, indent: u8) -> ReadResult {
  print_op("invokeinterface");
  read_u16!(method_idx, reader, indent + 1);
  let (ref class_name, ref method_name, ref descriptor) = resolve_method_name(pool, method_idx as usize)
    .expect(&format!("No method reference in pool at {}", method_idx));
  println!("Invoke #{} -> {}.{}:{}", method_idx, class_name, method_name, descriptor);

  read_u8!(count, reader, indent + 1);
  println!("Interface arg count: {}", count);

  read_u8!(null_value, reader, indent + 1);
  println!("Expected 0 value: <{}>", null_value);

  Ok(())
}
fn read_invoke_dynamic(reader: &mut Reader, pool: &PoolList, indent: u8) -> ReadResult {
  print_op("invokeinterface");
  read_u16!(method_idx, reader, indent + 1);
  let (ref class_name, ref method_name, ref descriptor) = resolve_method_name(pool, method_idx as usize)
    .expect(&format!("No method reference in pool at {}", method_idx));
  println!("Invoke #{} -> {}.{}:{}", method_idx, class_name, method_name, descriptor);

  read_u16!(first_null, reader, indent + 1);
  read_u8!(second_null, reader, indent + 1);
  println!("Expected 0 values: <{}> <{}>", first_null, second_null);

  Ok(())
}

fn read_new(reader: &mut Reader, pool: &PoolList, indent: u8) -> ReadResult {
  print_op("new");
  read_u16!(class_idx, reader, indent + 1);
  let class_name = resolve_class_name(pool, class_idx as usize)
    .expect(&format!("No class name in pool at {}", class_idx));
  println!("New {}", class_name);

  Ok(())
}

fn read_new_prim_array(reader: &mut Reader, indent: u8) -> ReadResult {
  print_op("newarray");
  read_u8!(array_type, reader, indent + 1);
  let type_name = ARRAY_TYPES.get(&array_type)
    .expect(&format!("No array type with code {}", array_type));
  println!("new array of {}", type_name);

  Ok(())
}

fn read_new_object_array(reader: &mut Reader, pool: &PoolList, indent: u8) -> ReadResult {
  print_op("anewarray");
  read_u16!(class_idx, reader, indent + 1);
  let class_name = resolve_class_name(pool, class_idx as usize)
    .expect(&format!("No class name in pool at {}", class_idx));
  println!("new array of {}", class_name);

  Ok(())
}

fn read_ldc(reader: &mut Reader, pool: &PoolList, indent: u8) -> ReadResult {
  print_op("ldc");
  read_u8!(cst_idx, reader, indent + 1);
  if let &Some(ref entry) = &pool[cst_idx as usize] {
    // TODO Test that the read value is an int, short, ref, ...
    println!("load constant #{} -> {:?}", cst_idx, entry);
  } else {
    panic!("No constant in the pool at {}", cst_idx);
  }
  Ok(())
}

fn read_ldc2_w(reader: &mut Reader, pool: &PoolList, indent: u8) -> ReadResult {
  print_op("ldc2_w");
  read_u16!(cst_idx, reader, indent + 1);
  if let &Some(ref entry) = &pool[cst_idx as usize] {
    // TODO test that the value is either an double or a long
    println!("load constant #{} -> {:?}", cst_idx, entry);
  } else {
    panic!("No constant in the pool at {}", cst_idx);
  }
  Ok(())
}

single_op!(read_dup, "dup");
single_op!(read_pop, "pop");
single_op!(read_pop2, "pop2");
