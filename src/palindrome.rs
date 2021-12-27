mod tests;
mod ascii;
mod fragment;
mod generator;
use generator::Generator;

pub fn generate_palindromes(word_list: &[&str], max_word_count: usize) -> Vec<String> {
    let mut generator = Generator::create(word_list, max_word_count);
    generator.generate();
    generator.into_palindrome_list()
}