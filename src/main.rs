mod words;
mod palindrome;
use std::time::Instant;

fn main() {
    let start_time = Instant::now();
    
    let palindromes = palindrome::generate_palindromes(&words::PU_WORDS, 6);
    
    let end_time = Instant::now();
    let elapsed = (end_time - start_time).as_secs_f64();
    
    for palindrome in palindromes.iter() {
        println!("{}", palindrome);
    }
    
    println!();
    println!("count: {}", palindromes.len());
    println!("elapsed: {:.3}", elapsed);
}
