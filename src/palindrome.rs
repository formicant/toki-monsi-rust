mod tests;
mod ascii;
mod fragment;
use fragment::Fragment;
use ascii::equal_reversed;

pub fn generate_palindromes(word_list: &[&str], max_word_count: usize) -> Vec<String> {
    let cores = word_list.iter()
        .flat_map(|&word| {
            let length = word.len() as isize;
            (-length..length).filter_map(|offset| Fragment::from_single_word(word, offset))
        });
    
    let mut palindrome_list = Vec::new();
    for core in cores {
        add_palindromes_recursively(word_list, max_word_count, &mut palindrome_list, core);
    }
    
    palindrome_list
}

fn get_words_for_prepending<'a>(word_list: &[&'a str], matching_part: &str) -> Vec<&'a str> {
    word_list.iter()
        .map(|&word| word)
        .filter(|&word| equal_reversed(matching_part, word))
        .collect()
}

fn get_words_for_appending<'a>(word_list: &[&'a str], matching_part: &str) -> Vec<&'a str> {
    word_list.iter()
        .map(|&word| word)
        .filter(|&word| equal_reversed(word, matching_part))
        .collect()
}

fn add_palindromes_recursively<'a>(word_list: &[&'a str], max_word_count: usize, palindrome_list: &mut Vec<String>, fragment: Fragment<'a>) {
    if fragment.len() < max_word_count {
        let words = 
            if fragment.can_prepend() {
                get_words_for_prepending(word_list, fragment.loose_end())
            } else {
                get_words_for_appending(word_list, fragment.loose_beginning())
            };
        
        let extensions = words.iter().map(|&word| fragment.extend(word));
        for extension in extensions {
            add_palindromes_recursively(word_list, max_word_count, palindrome_list, extension)
        }
    }
    if fragment.is_complete() {
        palindrome_list.push(fragment.get_phrase());
    }
}
