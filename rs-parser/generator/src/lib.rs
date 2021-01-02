mod java;

use language::syntax::Program;
use std::fs;
use std::path::{Path, PathBuf};

/// Generates the output directory for a given program.
/// To comply with Java module naming, this creates a series of parent directories for the final
/// program. From the passed filename, it extracts the name of the module directory, containing the main class
/// as well as any additional classes.
fn prepare_output<'a>(filename: &'a str, target_dir: &'a str) -> Result<PathBuf, String> {
    let base_name = Path::new(filename).file_stem().unwrap();
    let mut output_dir_buffer = PathBuf::from(target_dir);
    output_dir_buffer.push("com");
    output_dir_buffer.push("kineolyan");
    output_dir_buffer.push("tzio");
    output_dir_buffer.push(base_name);
    let mut result: Result<(), String> = Ok(());
    {
        let output_dir = output_dir_buffer.as_path();
        // Clean the existing directory
        let _deleted = fs::remove_dir_all(output_dir);
        let created = fs::create_dir_all(output_dir);
        if created.is_err() {
            result = Err(format!(
                "Could not create output directory {} due to error {}",
                output_dir.to_str().unwrap(),
                created.unwrap_err()
            ))
        }
    }

    result.map(|_| output_dir_buffer)
}

/// Do generate the program into an existing directory.
fn generate_program(program: &Program, output_dir: PathBuf) -> Result<(), String> {
    let package = output_dir.file_stem().unwrap().to_str().unwrap();
    java::create_main_file(&program, package, &output_dir)
}

/// Generates a TZIO program as a Java project.
///
/// # Arguments
///  - `program` - definition of the program to write
///  - `filename` - name of the file into which the main program must be written
///  - `target_dir` - directory where the project files and skaffold will be written
pub fn generate(program: Program, filename: &str, target_dir: &str) -> Result<(), String> {
    prepare_output(filename, target_dir)
        .and_then(|output_dir| generate_program(&program, output_dir))
}
