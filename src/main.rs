mod words;
mod palindrome;
use std::env;
use std::process;
use std::time::Instant;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("exactly one argument is required: max_word_count");
        process::exit(1)
    }
    let arg = &args[1];
    let max_word_count = match arg.parse::<usize>() {
        Ok(number)  => number,
        Err(_) => {
            eprintln!("invalid usize value: {arg}", arg=arg);
            process::exit(1)
        }
    };

    let start_time = Instant::now();
    
    let palindromes = palindrome::generate_palindromes(&words::PU_WORDS, max_word_count);
    
    let end_time = Instant::now();
    let elapsed = (end_time - start_time).as_secs_f64();
    
    for palindrome in palindromes.iter() {
        println!("{}", palindrome);
    }
    
    println!();
    println!("count: {}", palindromes.len());
    println!("elapsed: {:.3}", elapsed);
}
