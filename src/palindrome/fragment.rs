mod tests;
mod vec_ext;
use vec_ext::VecExt;
use super::ascii::equal_reversed;


#[derive(Debug)]
pub struct Fragment<'a> {
    words: Vec<&'a str>,
    offset: isize,
}

impl<'a> Fragment<'a> {
    pub fn len(&self) -> usize { self.words.len() }
    
    pub fn is_complete(&self) -> bool { self.offset == 0 }
    
    pub fn can_prepend(&self) -> bool { self.offset < 0 }
    
    pub fn can_append(&self) -> bool { self.offset >= 0 }
    
    pub fn get_phrase(&self) -> String {
        self.words.join(" ")
    }
    
    pub fn loose_end(&self) -> &'a str {
        debug_assert!(self.can_prepend());
        let last_word = self.words.last().unwrap();
        let abs_offset = -self.offset as usize;
        &last_word[last_word.len() - abs_offset..]
    }
    
    pub fn loose_beginning(&self) -> &'a str {
        debug_assert!(self.can_append());
        let first_word = self.words.first().unwrap();
        let abs_offset = self.offset as usize;
        &first_word[..abs_offset]
    }
    
    pub fn from_single_word(word: &str, offset: isize) -> Option<Fragment> {
        let length = word.len() as isize;
        debug_assert!((-length..length).contains(&offset));
        
        let matching_part =
            if offset < 0 { &word[..(length + offset) as usize] }
            else { &word[offset as usize..] };
        
        if equal_reversed(matching_part, matching_part) {
            let words = vec![word];
            Some(Fragment { words, offset })
        } else {
            None
        }
    }
    
    pub fn extend(&self, word: &'a str) -> Fragment<'a> {
        if self.can_prepend() {
            self.prepend(word)
        } else {
            self.append(word)
        }
    }
    
    fn prepend(&self, word: &'a str) -> Fragment<'a> {
        debug_assert!(self.can_prepend());
        debug_assert!(equal_reversed(self.loose_end(), word));
        
        let words = self.words.clone_and_prepend(word);
        let offset = self.offset + word.len() as isize;
        Fragment { words, offset }
    }
    
    fn append(&self, word: &'a str) -> Fragment<'a> {
        debug_assert!(self.can_append());
        debug_assert!(equal_reversed(word, self.loose_beginning()));
        
        let words = self.words.clone_and_append(word);
        let offset = self.offset - word.len() as isize;
        Fragment { words, offset }
    }
}
