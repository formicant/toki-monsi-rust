mod command_line;
use std::process;
use std::path::PathBuf;
use std::fs::File;
use std::io::Write;
use std::time::Instant;
use structopt::StructOpt;
use toki_monsi_rust::*;

fn main() {
    let args = command_line::Arguments::from_args();
    
    match args.output {
        Some(file_path) => write_to_file(args.max_word_count, args.sort, file_path),
        None => write_to_stdout(args.max_word_count, args.sort),
    }
}

fn write_to_stdout(max_word_count: usize, sort: Option<SortCriterion>) {
    let palindromes = generate_palindromes(max_word_count, sort);
    
    for palindrome in palindromes {
        println!("{}", palindrome);
    }
}

fn write_to_file(max_word_count: usize, sort: Option<SortCriterion>, file_path: PathBuf) {
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
    let palindromes = generate_palindromes(max_word_count, sort);
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
