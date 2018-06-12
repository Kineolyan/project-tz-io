use pool::{PoolList, resolve_utf8_value};
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