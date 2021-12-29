mod command_line;
use std::time::Instant;
use structopt::StructOpt;
use toki_monsi_rust::generate_palindromes;

fn main() {
    let arguments = command_line::Arguments::from_args();

    let start_time = Instant::now();
    
    let palindromes = generate_palindromes(arguments.max_word_count);
    
    let end_time = Instant::now();
    let elapsed = (end_time - start_time).as_secs_f64();
    
    for palindrome in palindromes.iter() {
        println!("{}", palindrome);
    }
    
    println!();
    println!("count: {}", palindromes.len());
    println!("elapsed: {:.3}", elapsed);
}
