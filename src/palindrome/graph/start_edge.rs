use std::rc::Rc;
use std::fmt;
use caseless::default_case_fold_str;
use unicode_segmentation::UnicodeSegmentation;

use super::Node;


#[cfg_attr(test, derive(Debug, PartialEq, Eq))]
pub struct StartEdge {
    pub word: String,
    pub to_node: Rc<Node>,
}

impl fmt::Display for StartEdge {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({})→ {}", self.word, self.to_node)
    }
}

impl StartEdge {
    pub fn get_all(word_list: &[&str]) -> Vec<Self> {
        let mut start_edges = Vec::new();
        
        for &word in word_list {
            let caseless_word = default_case_fold_str(word);
            
            for (index, _) in caseless_word.grapheme_indices(true) {
                if let Some(to_node) = Node::try_create(&caseless_word, &caseless_word[..index]) {
                    start_edges.push(Self {
                        word: String::from(word),
                        to_node,
                    });
                }
                if let Some(to_node) = Node::try_create(&caseless_word[index..], &caseless_word) {
                    start_edges.push(Self {
                        word: String::from(word),
                        to_node,
                    });
                }
            }
        }
    
        start_edges
    }
}
