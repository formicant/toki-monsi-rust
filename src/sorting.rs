use std::cmp::Ordering;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum SortCriterion {
    Alphabetical,
    Length,
    WordCount,
}

#[derive(Debug, Clone)]
pub struct ParseError;

impl ToString for ParseError {
    fn to_string(&self) -> String {
        String::from("Not a valid sort criterion")
    }
}

impl FromStr for SortCriterion {
    type Err = ParseError;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "a" | "alphabetical" => Ok(SortCriterion::Alphabetical),
            "l" | "length"       => Ok(SortCriterion::Length),
            "w" | "word-count"   => Ok(SortCriterion::WordCount),
            _ => Err(ParseError),
        }
    }
    
}

impl SortCriterion {
    pub fn comparator(&self) -> impl Fn(&String, &String) -> Ordering {
        match self {
            SortCriterion::Alphabetical => alphabetical_comparer,
            SortCriterion::Length => length_comparer,
            SortCriterion::WordCount => word_count_comparer,
        }
    }
}

fn alphabetical_comparer(a: &String, b: &String) -> Ordering {
    a.cmp(b)
}

fn length_comparer(a: &String, b: &String) -> Ordering {
    let length_ordering = a.len().cmp(&b.len());
    match length_ordering {
        Ordering::Equal => alphabetical_comparer(a, b),
        other => other,
    }
}

fn word_count_comparer(a: &String, b: &String) -> Ordering {
    let word_count_ordering = count_spaces(a).cmp(&count_spaces(b));
    match word_count_ordering {
        Ordering::Equal => alphabetical_comparer(a, b),
        other => other,
    }
}

fn count_spaces(s: &str) -> usize {
    s.as_bytes().iter().filter(|&&b| b == ' ' as u8).count()
}
