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
    pub fn is_complete(&self) -> bool {
        self.offset == 0
    }
    
    pub fn len(&self) -> usize {
        self.words.len()
    }
    
    pub fn get_phrase(&self) -> String {
        self.words.join(" ")
    }
    
    pub fn from_single_word(word: &str, offset: isize) -> Option<Fragment> {
        let length = word.len() as isize;
        assert!((-length..length).contains(&offset));
        
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
    
    pub fn extend(&self, word: &'a str) -> Option<Fragment<'a>> {
        if self.offset < 0 {
            self.prepend(word)
        } else {
            self.append(word)
        }
    }
    
    fn prepend(&self, word: &'a str) -> Option<Fragment<'a>> {
        assert!(self.offset < 0);
        let abs_offset = -self.offset as usize;
        let last_word = self.words.last()?;
        let matching_part = &last_word[last_word.len() - abs_offset..];
        
        if equal_reversed(matching_part, word) {
            let words = self.words.clone_and_prepend(word);
            let offset = self.offset + word.len() as isize;
            Some(Fragment { words, offset })
        } else {
            None
        }
    }
    
    fn append(&self, word: &'a str) -> Option<Fragment<'a>> {
        assert!(self.offset >= 0);
        let abs_offset = self.offset as usize;
        let first_word = self.words.first()?;
        let matching_part = &first_word[..abs_offset];
        
        if equal_reversed(word, matching_part) {
            let words = self.words.clone_and_append(word);
            let offset = self.offset - word.len() as isize;
            Some(Fragment { words, offset })
        } else {
            None
        }
    }
}
