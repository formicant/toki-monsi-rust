mod command_line;
use std::process;
use std::path::PathBuf;
use std::fs::File;
use std::io::Write;
use std::time::Instant;
use structopt::StructOpt;
use toki_monsi_rust::generate_palindromes;

fn main() {
    let arguments = command_line::Arguments::from_args();
    
    match arguments.output {
        Some(file_path) => write_to_file(arguments.max_word_count, file_path),
        None => write_to_stdout(arguments.max_word_count),
    }
}

fn write_to_stdout(max_word_count: usize) {
    let palindromes = generate_palindromes(max_word_count);
    
    for palindrome in palindromes {
        println!("{}", palindrome);
    }
}

fn write_to_file(max_word_count: usize, file_path: PathBuf) {
    let path_string = file_path.to_string_lossy();
    let mut file = match File::create(&file_path) {
        Ok(file) => file,
        Err(message) => {
            eprintln!("Cannot create file '{}': {}", path_string, message);
            process::exit(1);
        }
    };
    
    println!("Generating palindromes with max word count {}...", max_word_count);
    
    let start_time = Instant::now();
    let palindromes = generate_palindromes(max_word_count);
    let end_time = Instant::now();
    let elapsed = (end_time - start_time).as_secs_f64();
    
    println!("  palindromes generated: {}", palindromes.len());
    println!("  elapsed: {:.3} s", elapsed);
    
    println!("Saving to file '{}'...", path_string);
    for palindrome in palindromes {
        match writeln!(&mut file, "{}", palindrome) {
            Ok(()) => (),
            Err(message) => {
                eprintln!("Cannot write to file: {}", message);
                process::exit(1);
            }
        }
    }
    
    println!("Done.");
}
