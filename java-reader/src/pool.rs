use std::io;
use reader::{Reader, to_u16, to_i32, to_i64};
use printer::print_bytes;
use std::str::from_utf8;

#[derive(Debug, Clone)]
pub enum PoolElement {
  Utf8Value(String),
  ClassInfo(usize),
  Integer(i32),
  Long(i64),
  NameAndType(usize, usize),
  FieldRef {class_idx: usize, name_and_type_idx: usize },
  MethodRef {class_idx: usize, name_and_type_idx: usize }
}

pub type PoolList = Vec<Option<PoolElement>>;

fn read_utf8_value(reader: &mut Reader, indent: u8) -> io::Result<PoolElement> {
  read_u16!(length, reader, indent);
  println!("length {}", length);

  let value: String;
  {
    let mut bytes = vec![0; length as usize];
    reader.read(&mut bytes)?;
    print_bytes(indent, &bytes);
    value = String::from(from_utf8(&bytes).expect("Invalid utf8 content"));
  }
  // TODO support the full string encoding
  Ok(PoolElement::Utf8Value(value))
}

fn read_class_info(reader: &mut Reader, indent: u8) -> io::Result<PoolElement> {
  read_u16!(idx, reader, indent);
  Ok(PoolElement::ClassInfo(idx as usize))
}

fn read_integer(reader: &mut Reader, indent: u8) -> io::Result<PoolElement> {
  read_i32!(value, reader, indent);
  Ok(PoolElement::Integer(value))
}

fn read_long(reader: &mut Reader, indent: u8) -> io::Result<PoolElement> {
  read_i64!(value, reader, indent);
  Ok(PoolElement::Long(value))
}

fn read_name_and_type(reader: &mut Reader, indent: u8) -> io::Result<PoolElement> {
  let bytes = reader.read_4u()?;
  print_bytes(indent, bytes);
  let name_idx = to_u16(&bytes[0..2]) as usize;
  let descriptor_idx = to_u16(&bytes[2..4]) as usize;

  Ok(PoolElement::NameAndType(name_idx, descriptor_idx))
}

fn read_field_ref(reader: &mut Reader, indent: u8) -> io::Result<PoolElement> {
  let bytes = reader.read_4u()?;
  print_bytes(indent, bytes);
  let class_idx = to_u16(&bytes[0..2]) as usize;
  let name_and_type_idx = to_u16(&bytes[2..4]) as usize;

  Ok(PoolElement::FieldRef {class_idx: class_idx, name_and_type_idx: name_and_type_idx})
}

fn read_method_ref(reader: &mut Reader, indent: u8) -> io::Result<PoolElement> {
  let bytes = reader.read_4u()?;
  print_bytes(indent, bytes);
  let class_idx = to_u16(&bytes[0..2]) as usize;
  let name_and_type_idx = to_u16(&bytes[2..4]) as usize;

  Ok(PoolElement::MethodRef { class_idx: class_idx, name_and_type_idx: name_and_type_idx })
}

fn read_entry(reader: &mut Reader, index: &mut u16) -> io::Result<PoolElement> {
  let pool_code: u8;
  {
    let entry_code = reader.read_1u()?;
    print_bytes(1, entry_code);
    pool_code = entry_code[0];
  }

  print!("#{} ", index);
  let indent = 2;
  let element = match pool_code {
    1 => {
      println!("Utf8 constant");
      read_utf8_value(reader, indent)?
    },
    3 => {
      println!("Integer constant");
      read_integer(reader, indent)?
    },
    5 => {
      println!("Long constant");
      *index += 1;
      read_long(reader, indent)?
    },
    7 => {
      println!("Class info");
      read_class_info(reader, indent)?
    },
    9 => {
      println!("Field ref");
      read_field_ref(reader, indent)?
    },
    10 => {
      println!("Method ref");
      read_method_ref(reader, indent)?
    },
    12 => {
      println!("Name and type");
      read_name_and_type(reader, indent)?
    }
    _ => panic!("Unsupported pool element. Code = {}", pool_code)
  };
  *index += 1;
  println!("{:?}", element);
  Ok(element)
}

pub fn read_class_pool(reader: &mut Reader) -> io::Result<PoolList> {
  read_u16!(count, reader, 0);
  println!("constant pool size = {}", count);

	let mut entries = vec![None; count as usize];
  let mut i = 1;
	while i < count {
    let idx = i as usize;
    let entry = read_entry(reader, &mut i)?;
    entries[idx] = Some(entry);
	}

	Ok(entries)
}

pub fn resolve_utf8_value<'a>(pool: &'a PoolList, index: usize) -> Option<&'a str> {
	if let &Some(ref entry) = &pool[index] {
		match entry {
      &PoolElement::Utf8Value(ref value) => Some(value),
      &PoolElement::ClassInfo(idx) => {
        let class_entry = &pool[idx];
        if let &Some(PoolElement::Utf8Value(ref value)) = class_entry {
          Some(value)
        } else {
          panic!("Invalid index into pool: {:?}", class_entry);
        }
      },
      _ => None
    }
	} else {
		None
	}
}

pub fn resolve_class_name<'a>(pool: &'a PoolList, index: usize) -> Option<&'a str> {
	if let &Some(PoolElement::ClassInfo(ref name_idx)) = &pool[index] {
    let name = resolve_utf8_value(pool, *name_idx)
      .expect(&format!("No class name at {}", name_idx));
    Some(name)
	} else {
		None
	}
}

fn resolve_name_and_type<'a>(pool: &'a PoolList, index: usize) -> Option<(&'a str, &'a str)> {
	if let &Some(PoolElement::NameAndType(ref name_idx, ref descriptor_idx)) = &pool[index] {
    let name = resolve_utf8_value(pool, *name_idx)
      .expect(&format!("No method name at {}", name_idx));
    let descriptor = resolve_utf8_value(pool, *descriptor_idx)
      .expect(&format!("No descriptor string at {}", descriptor_idx));
    Some((name, descriptor))
	} else {
		None
	}
}

pub fn resolve_field_name<'a>(pool: &'a PoolList, index: usize) -> Option<(&'a str, &'a str, &'a str)> {
	if let &Some(PoolElement::FieldRef { ref class_idx, ref name_and_type_idx }) = &pool[index] {
    let class_name = resolve_utf8_value(pool, *class_idx)
      .expect(&format!("No field name at {}", class_idx));
    let name_type = resolve_name_and_type(pool, *name_and_type_idx)
      .expect(&format!("No name & type at {}", name_and_type_idx));
    Some((class_name, name_type.0, name_type.1))
	} else {
		None
	}
}

pub fn resolve_method_name<'a>(pool: &'a PoolList, index: usize) -> Option<(&'a str, &'a str, &'a str)> {
	if let &Some(PoolElement::MethodRef { ref class_idx, ref name_and_type_idx }) = &pool[index] {
    let class_name = resolve_utf8_value(pool, *class_idx)
      .expect(&format!("No method name at {}", class_idx));
    let name_type = resolve_name_and_type(pool, *name_and_type_idx)
      .expect(&format!("No name & type at {}", name_and_type_idx));
    Some((class_name, name_type.0, name_type.1))
	} else {
		None
	}
}