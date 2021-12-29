mod words;
mod palindrome;

pub fn generate_palindromes(max_word_count: usize) -> Vec<String> {
  palindrome::generate_palindromes(&words::PU_WORDS, max_word_count)
}
