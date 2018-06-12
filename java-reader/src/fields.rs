use printer::print_bytes;
use reader::{Reader, ReadResult, to_u16};

pub fn read(reader: &mut Reader) -> ReadResult {
	read_u16!(count, reader, 0);
	println!("Field count = {}", count);

	if count > 0 {
		panic!("Class defining {} field(s). Not supporting this", count);
	}

	Ok(())
}