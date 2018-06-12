use std::io;
use std::io::{BufReader, Read, Error, ErrorKind};
use std::fs::File;

pub type ReadResult = io::Result<()>;

pub trait Reader {

	fn read_1u(&mut self) -> io::Result<&[u8]>;

  fn read_2u(&mut self) -> io::Result<&[u8]>;

	fn read_4u(&mut self) -> io::Result<&[u8]>;

	fn read(&mut self, buffer: &mut [u8]) -> ReadResult;

  fn read_up_to_u16(&mut self, length: u16) -> io::Result<&[u8]>;
}

pub struct FileReader {
	buffer: BufReader<File>,
	data_buffer: [u8; 100]
}

impl FileReader {
	pub fn new(file: File) -> FileReader {
		FileReader {
			buffer: BufReader::new(file),
			data_buffer: [0; 100]
		}
	}

  fn read_buffer(reader: &mut BufReader<File>, buffer: &mut [u8]) -> ReadResult {
    // reader.read_exact(buffer)
    let mut remaining = buffer.len();
    let mut start = 0;
    while remaining > 0 {
      let read = reader.read(&mut buffer[start..])?;
      if read > 0 {
        if read <= remaining {
          remaining -= read;
          start += read;
        } else {
          panic!("Too much bytes read.");
        }
      } else {
        panic!("Cannot read more than {} bytes, but asking for {}", start, buffer.len());
      }
    }
    Ok(())
  }
}

impl Reader for FileReader {

	fn read_1u(&mut self) -> io::Result<&[u8]> {
		self.buffer.read_exact(&mut self.data_buffer[0..1])?;
		Ok(&self.data_buffer[0..1])
	}

	fn read_2u(&mut self) -> io::Result<&[u8]> {
		self.buffer.read_exact(&mut self.data_buffer[0..2])?;
		Ok(&self.data_buffer[0..2])
	}

	fn read_4u(&mut self) -> io::Result<&[u8]> {
		self.buffer.read_exact(&mut self.data_buffer[0..4])?;
		Ok(&self.data_buffer[0..4])
	}

	fn read(&mut self, buffer: &mut [u8]) -> ReadResult {
		Self::read_buffer(&mut self.buffer, buffer)
	}

  fn read_up_to_u16(&mut self, length: u16) -> io::Result<&[u8]> {
    if length <= 100 {
      let end = length as usize;
      Self::read_buffer(&mut self.buffer, &mut self.data_buffer[0..end])?;
      Ok(&self.data_buffer[0..end])
    } else {
      panic!("Not supporting read > 100 chars yet. Asked: {}", length);
    }
  }

}

pub struct ByteReader<'a> {
  bytes: &'a[u8],
  position: usize
}

impl <'a> ByteReader<'a> {
  pub fn new(bytes: &'a[u8]) -> ByteReader {
    ByteReader { bytes: bytes, position: 0 }
  }

  pub fn is_empty(&self) -> bool {
    self.bytes.len() <= self.position
  }

  pub fn get_slice(&mut self, length: usize) -> io::Result<&[u8]> {
    let start = self.position;
    let end = start + length;
    if end <= self.bytes.len() {
      self.position = end;
      Ok(&self.bytes[start..end])
    } else {
      Err(Error::new(ErrorKind::UnexpectedEof, "Not enough bytes"))
    }
  }
}

impl <'a> Reader for ByteReader<'a> {

	fn read_1u(&mut self) -> io::Result<&[u8]> {
		self.get_slice(1)
	}

	fn read_2u(&mut self) -> io::Result<&[u8]> {
		self.get_slice(2)
	}

	fn read_4u(&mut self) -> io::Result<&[u8]> {
		self.get_slice(4)
	}

	fn read(&mut self, buffer: &mut [u8]) -> ReadResult {
    let content = self.get_slice(buffer.len())?;
		buffer.clone_from_slice(content);
    Ok(())
	}

  fn read_up_to_u16(&mut self, length: u16) -> io::Result<&[u8]> {
    self.get_slice(length as usize)
  }

}

pub fn to_u16(bytes: &[u8]) -> u16 {
	((bytes[0] as u16) << 8) | (bytes[1] as u16)
}

pub fn to_u32(bytes: &[u8]) -> u32 {
	((bytes[0] as u32) << 24) 
	  | ((bytes[1] as u32) << 16) 
	  | ((bytes[2] as u32) << 8) 
    | (bytes[3] as u32)
}

macro_rules! read_u8 {
	($result: ident, $reader: tt, $indent:expr) => {
		let $result: u8;
    {
      let bytes = $reader.read_1u()?;
      print_bytes($indent, bytes);

      $result = bytes[0];
    }
	};
}

macro_rules! read_u16 {
	($result: ident, $reader: tt, $indent:expr) => {
		let $result: u16;
    {
      let bytes = $reader.read_2u()?;
      print_bytes($indent, bytes);

      $result = to_u16(bytes);
    }
	};
}

macro_rules! read_u32 {
	($result: ident, $reader: tt, $indent:expr) => {
		let $result: u32;
    {
      let bytes = $reader.read_4u()?;
      print_bytes($indent, bytes);

      $result = to_u32(bytes);
    }
	};
}