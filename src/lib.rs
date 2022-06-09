mod words;
mod palindrome;
mod sorting;

use rayon::slice::ParallelSliceMut;

pub use sorting::SortCriterion;
use palindrome::PalindromeGenerator;


pub fn generate_palindromes(max_word_count: usize, sort: Option<SortCriterion>) -> Vec<String> {
    let generator = PalindromeGenerator::new(&words::PU_WORDS);
    let mut results = generator.generate(max_word_count);
    
    if let Some(criterion) = sort {
        results.par_sort_unstable_by(criterion.comparator());
    }
    
    results
}
