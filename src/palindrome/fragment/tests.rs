#![cfg(test)]
use test_case::test_case;
use super::Fragment;

#[test_case("alasa", -6 => panics ; "[] alasa (non-intersecting, bad)")]
#[test_case("alasa", -5 => true   ; "[]alasa (non-intersecting, ok)")]
#[test_case("alasa", -4 => true   ; "[a]lasa")]
#[test_case("alasa", -3 => false  ; "[al]asa")]
#[test_case("alasa", -2 => true   ; "[ala]sa")]
#[test_case("alasa", -1 => false  ; "[alas]a")]
#[test_case("alasa",  0 => false  ; "[alasa]")]
#[test_case("alasa",  1 => false  ; "a[lasa]")]
#[test_case("alasa",  2 => true   ; "al[asa]")]
#[test_case("alasa",  3 => false  ; "ala[sa]")]
#[test_case("alasa",  4 => true   ; "alas[a]")]
#[test_case("alasa",  5 => panics ; "alasa[] (non-intersecting, bad)")]
fn test_from_single_word(word: &str, offset: isize) -> bool {
    Fragment::from_single_word(word, offset).is_some()
}

#[test_case("alasa", 2, "la"     => panics)]
#[test_case("ala",   0, "a"      => panics)]
#[test_case("kala", -3, "li"     => None)]
#[test_case("kala", -3, "a"      => Some(-2))]
#[test_case("kala", -3, "ala"    => Some(0))]
#[test_case("kala", -3, "pakala" => Some(3))]
fn test_prepend(initial_word: &str, initial_offset: isize, prepended_word: &str) -> Option<isize> {
    let initial = Fragment::from_single_word(initial_word, initial_offset);
    let prepended = initial?.prepend(prepended_word);
    Some(prepended?.offset)
}

#[test_case("kala", -3, "ala"  => panics)]
#[test_case("alasa", 2, "li"   => None)]
#[test_case("ala",   0, "a"    => Some(-1))]
#[test_case("alasa", 2, "la"   => Some(0))]
#[test_case("alasa", 2, "lape" => Some(-2))]
#[test_case("wile",  3, "li"   => Some(1))]
fn test_append(initial_word: &str, initial_offset: isize, appended_word: &str) -> Option<isize> {
    let initial = Fragment::from_single_word(initial_word, initial_offset);
    let appended = initial?.append(appended_word);
    Some(appended?.offset)
}

#[test]
fn test_multiple_operations() {
    let fragment = Fragment::from_single_word("pipi", 1).unwrap()
        .append("pilin").unwrap()
        .prepend("li").unwrap()
        .prepend("ni").unwrap();
    
    assert!(fragment.is_complete());
    assert_eq!(fragment.len(), 4);
    assert_eq!(fragment.get_phrase(), "ni li pipi pilin");
}
