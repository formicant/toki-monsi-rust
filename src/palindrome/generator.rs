use std::collections::HashMap;
use super::ascii::equal_reversed;
use super::fragment::Fragment;

pub struct Generator<'a> {
    word_list: &'a [&'a str],
    max_word_count: usize,
    palindrome_list: Vec<String>,
    words_for_prepending: HashMap<&'a str, Vec<&'a str>>,
    words_for_appending: HashMap<&'a str, Vec<&'a str>>,
}

impl<'a> Generator<'a> {
    pub fn create(word_list: &'a [&'a str], max_word_count: usize) -> Generator<'a> {
         Generator {
             word_list,
             max_word_count,
             palindrome_list: Vec::new(),
             words_for_prepending: HashMap::new(),
             words_for_appending: HashMap::new(),
        }
    }
    
    pub fn generate_from_core(&mut self, core: &Fragment<'a>) {
        self.add_palindromes_recursively(core);
    }
    
    pub fn into_palindrome_list(self) -> Vec<String> { self.palindrome_list }
    
    fn get_words_for_prepending(&mut self, matching_part: &'a str) -> &Vec<&'a str> {
        self.words_for_prepending.entry(matching_part)
            .or_insert_with(||
                self.word_list.iter()
                    .map(|&word| word)  // ???
                    .filter(|&word| equal_reversed(matching_part, word))
                    .collect())
    }
    
    fn get_words_for_appending(&mut self, matching_part: &'a str) -> &Vec<&'a str> {
        self.words_for_appending.entry(matching_part)
            .or_insert_with(||
                self.word_list.iter()
                    .map(|&word| word)  // ???
                    .filter(|&word| equal_reversed(word, matching_part))
                    .collect())
    }
    
    fn add_palindromes_recursively(&mut self, fragment: &Fragment<'a>) {
        let word_count = fragment.len();
        debug_assert!(word_count <= self.max_word_count);
        
        if fragment.is_complete() {
            self.palindrome_list.push(fragment.get_phrase());
        }
        
        if word_count < self.max_word_count {
            let words = 
                if fragment.can_prepend() {
                    self.get_words_for_prepending(fragment.loose_end())
                } else {
                    self.get_words_for_appending(fragment.loose_beginning())
                };
            
            let extensions: Vec<_> = words.iter().map(|&word| fragment.extend(word)).collect();
            for extension in extensions.iter() {
                self.add_palindromes_recursively(&extension)
            }
        }
    }
}
