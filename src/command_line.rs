use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "toki-monsi-rust",
    about = "Generates palindromes in Toki Pona.",
)]
pub struct Arguments {
    /// Maximum palindrome word count
    pub max_word_count: usize,
}
