#[macro_use]
extern crate nom;

use std::env;
use std::fs::File;
use std::io::prelude::*;

mod parser;
mod postprocessor;
mod checker;
mod generator;

use parser::{ParsingResult, ParsingTree, parse};

fn parse_file(filename: &str) -> ParsingResult {
	println!("Compiling {}", filename);
	let mut f = File::open(filename).expect("file not found");

	let mut contents = String::new();
	// Could use buffering to read larger files
	// let mut buf_reader = BufReader::new(file);
	// let mut contents = String::new();
	// buf_reader.read_to_string(&mut contents)?;
	f.read_to_string(&mut contents)
		.expect("something went wrong reading the file");

	parse(contents.as_bytes())
}

fn check_file(result: ParsingTree) -> Result<ParsingTree, String> {
	let check_result = checker::check(&result);
	check_result.print_report();
	if check_result.has_errors() {
		Err(String::from("Exit after errors"))
	} else {
		Ok(result)
	}
}

fn process_input(filename: &str) -> Result<ParsingTree, String> {
	parse_file(filename)
		.map_err(|_| String::from("Failed parsing"))
		.map(postprocessor::process)
		.and_then(check_file)
}

fn create_output(result: ParsingTree, filename: &str, target_dir: &str) -> Result<(), String> {
	generator::generate(result, filename, target_dir)
}

fn main() {
	let args: Vec<String> = env::args().collect();

	let filename = &args[1];
	let target_dir = &args[2];
	let result = process_input(filename)
		.and_then(|result| create_output(result, filename, target_dir));
	if result.is_ok() {
		println!("File {} compiled with success to {}", filename, target_dir);
	} else {
		panic!("Error in file {} compilation: {}", filename, result.unwrap_err());
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_sample_sum() {
		let res = process_input("../language-samples/sum.io");
		assert_eq!(res.is_ok(), true);
	}

	#[test]
	fn test_sample_increment() {
		let res = process_input("../language-samples/increment.io");
		assert_eq!(res.is_ok(), true);
	}

}
