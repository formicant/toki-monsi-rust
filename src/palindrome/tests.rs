#![cfg(test)]
use test_case::test_case;
use std::iter::repeat;
use itertools::Itertools;
use crate::words::PU_WORDS;
use super::ascii::equal_reversed;
use super::generate_palindromes;

const SMALL_WORD_LIST: [&str; 6] = ["a", "ala", "alasa", "kala", "la", "pu"];

fn generate_palindromes_naïvely(word_list: &[&str], max_word_count: usize) -> Vec<String> {
    let combinations = (1..1 + max_word_count).flat_map(|word_count|
        repeat(word_list.iter()).take(word_count).multi_cartesian_product());
    combinations
        .filter(|combination| {
            let joined = combination.iter().join("");
            equal_reversed(&joined, &joined)
        })
        .map(|combination| combination.iter().join(" "))
        .collect()
}

#[test_case(&SMALL_WORD_LIST, 6)]
#[test_case(&PU_WORDS, 2)]
fn test_generate_palindromes(word_list: &[&str], max_word_count: usize) {
    let mut actual = generate_palindromes(word_list, max_word_count);
    actual.sort();
    
    let mut expected = generate_palindromes_naïvely(word_list, max_word_count);
    expected.sort();
    
    assert_eq!(actual, expected);
}

#[test]
fn test_case_insensitiveness() {
    const CASED_WORD_LIST: [&str; 3] = ["ala", "Ala", "kALa"];
    const EXPECTED: [&str; 8] = ["Ala", "Ala Ala", "Ala ala", "Ala kALa", "ala", "ala Ala", "ala ala", "ala kALa"];
    
    let mut actual = generate_palindromes(&CASED_WORD_LIST, 2);
    actual.sort();
    
    assert_eq!(actual, EXPECTED);
}

#[test]
fn test_max_count_zero() {
    assert_eq!(generate_palindromes(&PU_WORDS, 0).len(), 0)   
}
