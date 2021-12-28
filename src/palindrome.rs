mod tests;
mod ascii;
mod fragment;
mod generator;
use rayon::prelude::*;
use fragment::Fragment;
use generator::Generator;

pub fn generate_palindromes(word_list: &[&str], max_word_count: usize) -> Vec<String> {
        if max_word_count == 0 {
        return Vec::new()
    }
    
    let all_cores = get_cores(word_list);
    
    let generate_from_cores = move |cores: &[Fragment]| {
        let mut generator = Generator::create(word_list, max_word_count);
        for core in cores.iter() {
            generator.generate_from_core(&core);
        }
        generator.into_palindrome_list()
    };
    
    let thread_count = num_cpus::get();
    let chunk_size = 1 + (all_cores.len() - 1) / thread_count;
    let chunks: Vec<_> = all_cores.chunks(chunk_size).collect();
    
    chunks.par_iter()
        .flat_map(|chunk| generate_from_cores(chunk))
        .collect()
}

fn get_cores<'a>(word_list: &[&'a str]) -> Vec<Fragment<'a>> {
    word_list.iter()
        .flat_map(|&word| {
            let length = word.len() as isize;
            (-length..length).filter_map(|offset| Fragment::from_single_word(word, offset))
        })
        .collect()  
}
