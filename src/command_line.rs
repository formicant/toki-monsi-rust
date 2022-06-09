use std::path::PathBuf;
use structopt::StructOpt;
use toki_monsi_rust::SortCriterion;


#[derive(Debug, StructOpt)]
#[structopt(
    name = "toki-monsi-rust",
    about = "Generates palindromes in Toki Pona.",
)]
pub struct Arguments {
    /// Maximum palindrome word count
    pub max_word_count: usize,
    
    /// Output path, `stdout` if not present
    #[structopt(short, long)]
    #[structopt(parse(from_os_str))]
    pub output: Option<PathBuf>,
    
    /// Result sorting: A[lphabetical], L[ength], or W[ord-count]
    #[structopt(short, long)]
    pub sort: Option<SortCriterion>,
}
