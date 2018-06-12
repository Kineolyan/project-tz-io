use flags::to_method_access;
use pool::{PoolList, resolve_utf8_value};
use printer::print_bytes;
use reader::{Reader, ReadResult, to_u16};
use attributes::read as read_attributes;

fn read_access(reader: &mut Reader, indent: u8) -> ReadResult {
	read_u16!(flag_value, reader, indent);
	let flags = to_method_access(flag_value);
	print!("Flags:");
	for flag in &flags {
		print!(" {}", flag);
	}
	println!("");

	Ok(())
}

fn read_method_name(reader: &mut Reader, pool: &PoolList, indent: u8) -> ReadResult {
	read_u16!(name_idx, reader, indent);
	let method_name = resolve_utf8_value(pool, name_idx as usize)
		.expect(&format!(
			"Method name not in the constant pool at {}",
			name_idx));

	println!("Method '{}'", method_name);

	Ok(())
}

fn read_descriptor(reader: &mut Reader, pool: &PoolList, indent: u8) -> ReadResult {
	read_u16!(descriptor_idx, reader, indent);
	let method_name = resolve_utf8_value(pool, descriptor_idx as usize)
		.expect(&format!(
			"Method descriptor not in the constant pool at {}",
			descriptor_idx));

	println!("Descriptor '{}'", method_name);

	Ok(())
}

fn read_method(reader: &mut Reader, pool: &PoolList, indent: u8) -> ReadResult {
	read_access(reader, indent)?;
	read_method_name(reader, pool, indent)?;
	read_descriptor(reader, pool, indent + 1)?;
	read_attributes(reader, pool, indent + 1)
}

pub fn read(reader: &mut Reader, pool: &PoolList) -> ReadResult {
	read_u16!(count, reader, 0);
	println!("Method count = {}", count);

	for _i in 0..count {
		read_method(reader, pool, 1)?;
	}

	Ok(())
}