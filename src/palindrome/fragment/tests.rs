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
#[test_case("kala", -3, "li"     => panics)]
#[test_case("kala", -3, "a"      => -2)]
#[test_case("kala", -3, "ala"    => 0)]
#[test_case("kala", -3, "pakala" => 3)]
fn test_prepend_offset(initial_word: &str, initial_offset: isize, prepended_word: &str) -> isize {
    let initial = Fragment::from_single_word(initial_word, initial_offset).unwrap();
    let prepended = initial.prepend(prepended_word);
    prepended.offset
}

#[test_case("kala", -3, "ala"  => panics)]
#[test_case("alasa", 2, "li"   => panics)]
#[test_case("ala",   0, "a"    => -1)]
#[test_case("alasa", 2, "la"   => 0)]
#[test_case("alasa", 2, "lape" => -2)]
#[test_case("wile",  3, "li"   => 1)]
fn test_append_offset(initial_word: &str, initial_offset: isize, appended_word: &str) -> isize {
    let initial = Fragment::from_single_word(initial_word, initial_offset).unwrap();
    let appended = initial.append(appended_word);
    appended.offset
}

#[test]
fn test_multiple_operations() {
    let fragment = Fragment::from_single_word("pipi", 1).unwrap()
        .append("pilin")
        .prepend("li")
        .prepend("ni");
    
    assert!(fragment.is_complete());
    assert_eq!(fragment.len(), 4);
    assert_eq!(fragment.get_phrase(), "ni li pipi pilin");
}
