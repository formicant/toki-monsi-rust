mod words;
mod palindrome;
mod sorting;
mod timing;

use rayon::slice::ParallelSliceMut;

pub use sorting::SortCriterion;
use palindrome::PalindromeGenerator;
use timing::Timing;


pub fn generate_palindromes(max_word_count: usize, sort: Option<SortCriterion>) -> (Vec<String>, Timing) {
    let mut timing = Timing::new();
    
    let generator = PalindromeGenerator::new(&words::PU_WORDS);
    timing.mark("graph");
    
    let mut results = generator.generate(max_word_count);
    timing.mark("generation");
    
    if let Some(criterion) = sort {
        results.par_sort_unstable_by(criterion.comparator());
        timing.mark("sorting");
    }
    
    (results, timing)
}
