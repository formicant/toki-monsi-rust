use test_case::test_case;
use std::iter::repeat;
use itertools::Itertools;
use unicode_segmentation::UnicodeSegmentation;

use crate::words::PU_WORDS;
use super::PalindromeGenerator;


const NO_PALINDROME_POSSIBLE: [&str; 2] = ["pu", "en"];
const SMALL_WORD_LIST: [&str; 6] = ["a", "ala", "alasa", "kala", "la", "pu"];

fn generate_palindromes_naïvely(word_list: &[&str], max_word_count: usize) -> Vec<String> {
    let combinations = (1..1 + max_word_count).flat_map(|word_count|
        repeat(word_list.iter()).take(word_count).multi_cartesian_product());
    combinations
        .filter(|combination| {
            let joined = combination.iter().join("");
            is_palindromic(&joined)
        })
        .map(|combination| combination.iter().join(" "))
        .collect()
}

fn is_palindromic(string: &str) -> bool {
    let forward_iter = string.graphemes(true);
    let backward_iter = string.graphemes(true).rev();
    forward_iter.eq(backward_iter)
}


#[test_case(&NO_PALINDROME_POSSIBLE, 6)]
#[test_case(&SMALL_WORD_LIST, 6)]
#[test_case(&PU_WORDS, 2)]
fn test_generate_palindromes(word_list: &[&str], max_word_count: usize) {
    let generator = PalindromeGenerator::new(word_list);
    let mut actual = generator.generate(max_word_count);
    actual.sort();
    
    let mut expected = generate_palindromes_naïvely(word_list, max_word_count);
    expected.sort();
    
    assert_eq!(actual, expected);
}

#[test]
fn test_case_insensitiveness() {
    const CASED_WORD_LIST: [&str; 3] = ["ala", "Ala", "kALa"];
    const EXPECTED: [&str; 8] = ["Ala", "Ala Ala", "Ala ala", "Ala kALa", "ala", "ala Ala", "ala ala", "ala kALa"];
    
    let generator = PalindromeGenerator::new(&CASED_WORD_LIST);
    let mut actual = generator.generate(2);
    actual.sort();
    
    assert_eq!(actual, EXPECTED);
}

#[test]
fn test_max_count_zero() {
    let generator = PalindromeGenerator::new(&PU_WORDS);
    let actual = generator.generate(0);
    
    assert_eq!(actual.len(), 0)   
}
