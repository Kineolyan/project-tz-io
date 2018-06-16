use inheritance::read_class_access;
use pool::{PoolList, resolve_utf8_value, resolve_class_name};
use printer::print_bytes;
use reader::{Reader, ByteReader, ReadResult, to_u16, to_u32};
use operations;

pub fn read(reader: &mut Reader, pool: &PoolList, indent: u8) -> ReadResult {
	read_u16!(count, reader, indent);
	println!("Attribute count = {}", count);

	for _i in 0..count { 
		read_attribute(reader, pool, indent)?;
	}
	Ok(())
}

pub fn read_attribute(reader: &mut Reader, pool: &PoolList, indent: u8) -> ReadResult {
  read_u16!(attribute_idx, reader, indent);
  let attribute_name = resolve_utf8_value(pool, attribute_idx as usize)
    .expect(&format!(
      "No attribute name in constant pool at {}",
      attribute_idx));
  println!("Attribute '{}'", attribute_name);

  read_u32!(length, reader, indent + 1);
  println!("Attribute length = {}", length);

  let mut bytes = vec![0; length as usize];
  reader.read(&mut bytes[..])?;

  let mut attribute_reader = ByteReader::new(&bytes);
  match attribute_name {
    "Code" => read_code(&mut attribute_reader, pool, indent + 1),
    "InnerClasses" => read_inner_classes(&mut attribute_reader, pool, indent + 1),
    "LineNumberTable" => read_line_number_table(&mut attribute_reader, indent + 1),
    "SourceFile" => read_source_file(&mut attribute_reader, pool, indent + 1),
    _ => panic!("Unsupported attribute '{}'", attribute_name)
  }
}

fn read_code(reader: &mut ByteReader, pool: &PoolList, indent: u8) -> ReadResult {
  read_u16!(max_stack, reader, indent);
  println!("Max stack = {}", max_stack);

  read_u16!(max_locals, reader, indent);
  println!("Max local vars = {}", max_locals);

  read_u32!(code_length, reader, indent);
  println!("Code length = {}", code_length);

  {
    let code_bytes = reader.get_slice(code_length as usize)?;
    let mut code_reader = ByteReader::new(&code_bytes);
    operations::read(&mut code_reader, pool, indent + 1)?;
  }

  read_u16!(exception_length, reader, indent);
  println!("Exception table length = {}", exception_length);
  if exception_length > 0 {
    panic!("No support for exception table");
  }

  read(reader, pool, indent)
}

fn read_inner_classes(reader: &mut ByteReader, pool: &PoolList, indent: u8) -> ReadResult {
  read_u16!(count, reader, indent);
  println!("Number of inners = {}", count);

  for _i in 0..count {
    read_u16!(inner_class_idx, reader, indent + 1);
    let inner_class_name = resolve_class_name(pool, inner_class_idx as usize)
      .expect(&format!(
        "No class in the pool at {}", inner_class_idx));
    println!("Inner class {} -> {}", inner_class_idx, inner_class_name);

    read_u16!(outer_class_idx, reader, indent + 1);
    let outer_class_name = if outer_class_idx == 0 {
      "<anonymous>"
    } else {
      resolve_class_name(pool, outer_class_idx as usize)
        .expect(&format!(
          "No class in the pool at {}", outer_class_idx))
    };
    println!("Outer class {} -> {}", outer_class_idx, outer_class_name);

    read_u16!(user_idx, reader, indent + 1);
    let user_name = if user_idx == 0 {
      "<anonymous>"
    } else {
      resolve_utf8_value(pool, user_idx as usize)
        .expect(&format!("No string value in the pool at {}", user_idx))
    };
    println!("Defined class name {} -> {}", user_idx, user_name);

    read_class_access(reader, indent + 1)?;
  }

  Ok(())
}

fn read_line_number_table(reader: &mut ByteReader, indent: u8) -> ReadResult {
  read_u16!(table_length, reader, indent);
  println!("Table length = {}", table_length);

  for _i in 0..table_length {
    read_u16!(start_pc, reader, indent + 1);
    println!("Start in the code array = {}", start_pc);

    read_u16!(line_number, reader, indent + 1);
    println!("Code line = {}", line_number);
  }

  Ok(())
}

fn read_source_file(reader: &mut ByteReader, pool: &PoolList, indent: u8) -> ReadResult {
  read_u16!(file_idx, reader, indent);
  let file_name = resolve_utf8_value(pool, file_idx as usize)
    .expect(&format!(
      "No file name string at index {}", file_idx));
  println!("Source file: {}", file_name);

  Ok(())
}
