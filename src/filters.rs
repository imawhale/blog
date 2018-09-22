#[allow(dead_code)]
pub(crate) fn indent(s: &str, width: &usize) -> askama::Result<String> {
  let mut indented = String::new();

  for (i, c) in s.char_indices() {
    indented.push(c);

    if c == '\n' && i != s.len() - 1 {
      for _ in 0..*width {
        indented.push(' ');
      }
    }
  }

  Ok(indented)
}
