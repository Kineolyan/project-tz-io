use std::io;

use flags::to_class_access;
use pool::{PoolList, resolve_utf8_value};
use printer::print_bytes;
use reader::{Reader, ReadResult, to_u16};

pub fn read_class_access(reader: &mut Reader, indent: u8) -> ReadResult {
	let bytes = reader.read_2u()?;
	let flag_value = to_u16(bytes);
	let flags = to_class_access(flag_value);

	print_bytes(indent, bytes);
	print!("Flags:");
	for flag in &flags {
		print!(" {}", flag);
	}
	println!("");

	Ok(())
}

fn read_class_name<'a>(reader: &'a mut Reader, pool: &'a PoolList) -> io::Result<(&'a [u8], Option<&'a str>)> {
	let bytes = reader.read_2u()?;
	let index = to_u16(bytes);
	let class_name = resolve_utf8_value(pool, index as usize);

	Ok((bytes, class_name))
}

fn read_class(reader: &mut Reader, pool: &PoolList, indent: u8) -> ReadResult {
	let (bytes, class_name) = read_class_name(reader, pool)?;
	print_bytes(indent, bytes);
	println!(
		"Class '{}'",
		class_name.expect("Class name is not present in the constant pool"));

	Ok(())
}

fn read_super_class(reader: &mut Reader, pool: &PoolList, indent: u8) -> ReadResult {
	let (bytes, class_name) = read_class_name(reader, pool)?;
	print_bytes(indent, bytes);
	println!(
		"Super class '{}'",
		class_name.expect("Super name is not present in the constant pool"));

	Ok(())
}

fn read_interfaces(reader: &mut Reader, pool: &PoolList, indent: u8) -> ReadResult {
	let interface_count: u16;
	{
		let bytes = reader.read_2u()?;
		interface_count = to_u16(bytes);
		print_bytes(indent, bytes);
		println!("Interface count: {}", interface_count);
	}

	for _i in 0..interface_count {
		let (bytes, class_name) = read_class_name(reader, pool)?;
		print_bytes(indent + 1, bytes);
		println!(
			"Interface '{}'",
			class_name.expect("Interface name is not present in the constant pool"));
	}

	Ok(())
}

pub fn read(reader: &mut Reader, pool: &PoolList) -> ReadResult {
	read_class_access(reader, 0)?;
	read_class(reader, pool, 0)?;
	read_super_class(reader, pool, 1)?;
	read_interfaces(reader, pool, 1)
}