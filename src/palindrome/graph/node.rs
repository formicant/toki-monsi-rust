use std::rc::Rc;
use std::fmt;
use unicode_segmentation::UnicodeSegmentation;


#[cfg_attr(test, derive(Debug))]
#[derive(PartialEq, Eq, Hash)]
pub enum Node {
    Start,
    Head(String),
    Tail(String),
    Final,
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Start      => write!(f, "[start]"),
            Self::Head(head) => write!(f, "{head}-"),
            Self::Tail(tail) => write!(f, "-{tail}"),
            Self::Final      => write!(f, "-"),
        }
    }
}

impl Node {
    pub fn try_create<'a>(forward: &'a str, backward: &'a str) -> Option<Rc<Self>> {
        let mut forward_iter = forward.grapheme_indices(true);
        let mut backward_iter = backward.grapheme_indices(true).rev();
        
        // Iterate while both iterators get the same graphemes
        loop {
            match (forward_iter.next(), backward_iter.next()) {
                // both iterators ended
                // one string is the complete reverse of the other
                (None, None) => {
                    return Some(Rc::new(Self::Final));
                }
                // forward iterator ended
                // place the rest of the backward string into a head node
                (None, Some((index, grapheme))) => {
                    let head = String::from(&backward[..index + grapheme.len()]);
                    return Some(Rc::new(Self::Head(head)));
                }
                // backward iterator ended
                // place the rest of the forward string into a tail node
                (Some((index, _)), None) => {
                    let tail = String::from(&forward[index..]);
                    return Some(Rc::new(Self::Tail(tail)));
                }
                // iterators get different graphemes
                // the strings are not palindromic
                (Some((_, forward_grapheme)), Some((_, backward_grapheme))) => {
                    if forward_grapheme != backward_grapheme {
                        return None;
                    }
                }
            }
        }
    }    
}
