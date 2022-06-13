mod command_line;

use std::process;
use std::path::PathBuf;
use std::fs::File;
use std::io::{BufWriter, Write};
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
    let (palindromes, _) = generate_palindromes(max_word_count, sort);
    
    for palindrome in palindromes {
        println!("{palindrome}");
    }
}

fn write_to_file(max_word_count: usize, sort: Option<SortCriterion>, file_path: PathBuf) {
    let path_string = file_path.to_string_lossy();
    let file = match File::create(&file_path) {
        Ok(file) => file,
        Err(message) => {
            eprintln!("Cannot create file '{path_string}': {message}");
            process::exit(1);
        }
    };
    
    println!("Generating palindromes with max word count {max_word_count}...");
    
    let (palindromes, timing) = generate_palindromes(max_word_count, sort);
    let count = palindromes.len();
    
    println!("  palindromes generated: {count}", );
    println!("Timing: {timing}");
    
    println!("Saving to file '{path_string}'...");
    
    let mut writer = BufWriter::new(file);
    for palindrome in palindromes {
        match writeln!(&mut writer, "{palindrome}") {
            Ok(()) => (),
            Err(message) => {
                eprintln!("Cannot write to file: {message}");
                process::exit(1);
            }
        }
    }
    
    println!("Done.");
}
