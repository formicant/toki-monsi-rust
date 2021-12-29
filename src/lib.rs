mod words;
mod palindrome;
mod sorting;
use rayon::slice::ParallelSliceMut;

pub use sorting::SortCriterion;

pub fn generate_palindromes(max_word_count: usize, sort: Option<SortCriterion>) -> Vec<String> {
    let mut results = palindrome::generate_palindromes(&words::PU_WORDS, max_word_count);
    
    match sort {
        None => (),
        Some(criterion) => results.par_sort_unstable_by(criterion.comparator()),
    }
    
    results
}
