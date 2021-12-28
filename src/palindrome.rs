mod tests;
mod ascii;
mod fragment;
mod generator;
use fragment::Fragment;
use generator::Generator;

pub fn generate_palindromes(word_list: &[&str], max_word_count: usize) -> Vec<String> {
    if max_word_count == 0 {
        return Vec::new()
    }
    
    let cores = get_cores(word_list);

    let mut generator = Generator::create(word_list, max_word_count);
    
    for core in cores {
        generator.generate_from_core(&core);
    }
    
    generator.into_palindrome_list()
}

fn get_cores<'a>(word_list: &[&'a str]) -> Vec<Fragment<'a>> {
    word_list.iter()
        .flat_map(|&word| {
            let length = word.len() as isize;
            (-length..length).filter_map(|offset| Fragment::from_single_word(word, offset))
        })
        .collect()  
}
