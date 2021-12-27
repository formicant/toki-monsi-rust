pub fn equal_reversed(forward: &str, backward: &str) -> bool {
  forward.as_bytes().iter()
      .zip(backward.as_bytes().iter().rev())
      .all(|(f, b)| f.eq_ignore_ascii_case(b))
}
