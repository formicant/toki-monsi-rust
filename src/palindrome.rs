mod tests;
mod ascii;
mod fragment;
use fragment::Fragment;

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

fn add_palindromes_recursively<'a>(word_list: &[&'a str], max_word_count: usize, palindrome_list: &mut Vec<String>, fragment: Fragment<'a>) {
    if fragment.len() < max_word_count {
        let extensions = word_list.iter().filter_map(|&word| fragment.extend(word));
        for extension in extensions {
            add_palindromes_recursively(word_list, max_word_count, palindrome_list, extension)
        }
    }
    if fragment.is_complete() {
        palindrome_list.push(fragment.get_phrase());
    }
}
