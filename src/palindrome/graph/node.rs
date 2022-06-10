use std::sync::Arc;
use std::fmt;
use unicode_segmentation::UnicodeSegmentation;


#[cfg_attr(test, derive(Debug))]
#[derive(PartialEq, Eq, Hash)]
pub enum Node {
    Final,
    Head(String),
    Tail(String),
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Final      => write!(f, "-"),
            Self::Head(head) => write!(f, "{head}-"),
            Self::Tail(tail) => write!(f, "-{tail}"),
        }
    }
}

impl Node {
    pub fn try_create<'a>(forward: &'a str, backward: &'a str) -> Option<Arc<Self>> {
        let mut forward_iter = forward.grapheme_indices(true);
        let mut backward_iter = backward.grapheme_indices(true).rev();
        
        loop {
            match (forward_iter.next(), backward_iter.next()) {
                (None, None) => {
                    return Some(Arc::new(Self::Final));
                }
                (None, Some((index, grapheme))) => {
                    let head = String::from(&backward[..index + grapheme.len()]);
                    return Some(Arc::new(Self::Head(head)));
                }
                (Some((index, _)), None) => {
                    let tail = String::from(&forward[index..]);
                    return Some(Arc::new(Self::Tail(tail)));
                }
                (Some((_, forward_grapheme)), Some((_, backward_grapheme))) => {
                    if forward_grapheme != backward_grapheme {
                        return None;
                    }
                }
            }
        }
    }    
}
