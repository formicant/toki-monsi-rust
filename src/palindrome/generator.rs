use std::collections::HashMap;
use super::ascii::equal_reversed;
use super::fragment::Fragment;

pub struct Generator<'a> {
    word_list: &'a [&'a str],
    max_word_count: usize,
    palindrome_list: Vec<String>,
    words_for_prepending: HashMap<String, Vec<&'a str>>,
    words_for_appending: HashMap<String, Vec<&'a str>>,
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
    
    pub fn generate(&mut self) {
        let cores = self.word_list.iter()
            .flat_map(|&word| {
                let length = word.len() as isize;
                (-length..length).filter_map(|offset| Fragment::from_single_word(word, offset))
            });
        
        for core in cores {
            self.add_palindromes_recursively(core);
        }
    }
    
    pub fn into_palindrome_list(self) -> Vec<String> { self.palindrome_list }
    
    fn get_words_for_prepending(&mut self, matching_part: &str) -> &Vec<&'a str> {
        self.words_for_prepending.entry(String::from(matching_part))
            .or_insert_with(||
                self.word_list.iter()
                    .map(|&word| word)  // ???
                    .filter(|&word| equal_reversed(matching_part, word))
                    .collect())
    }
    
    fn get_words_for_appending(&mut self, matching_part: &str) -> &Vec<&'a str> {
        self.words_for_appending.entry(String::from(matching_part))
            .or_insert_with(||
                self.word_list.iter()
                    .map(|&word| word)  // ???
                    .filter(|&word| equal_reversed(word, matching_part))
                    .collect())
    }
    
    fn add_palindromes_recursively(&mut self, fragment: Fragment) {
        if fragment.is_complete() {
            self.palindrome_list.push(fragment.get_phrase());
        }
        if fragment.len() < self.max_word_count {
            let words = 
                if fragment.can_prepend() {
                    self.get_words_for_prepending(fragment.loose_end())
                } else {
                    self.get_words_for_appending(fragment.loose_beginning())
                };
            
            let extensions: Vec<_> = words.iter().map(|&word| fragment.extend(word)).collect();
            for extension in extensions {
                self.add_palindromes_recursively(extension)
            }
        }
    }
}
