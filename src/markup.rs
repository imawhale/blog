use crate::common::*;

use pulldown_cmark;

#[derive(Debug, Copy, Clone)]
pub(crate) enum Markup {
  Markdown,
  Html,
  AsciiDoc,
}

impl Markup {
  pub(crate) fn render(self, text: &str) -> Result<String, Error> {
    match self {
      Markup::Html => Ok(text.to_string()),
      Markup::AsciiDoc => {
        let command: &[&str] = &["asciidoctor", "-s", "-o", "-", "-"];

        let mut child = Command::new(&command[0])
          .args(&command[1..])
          .stdin(Stdio::piped())
          .stdout(Stdio::piped())
          .spawn()
          .context(Error::command_io(command))?;

        child
          .stdin
          .as_mut()
          .unwrap()
          .write_all(text.as_bytes())
          .context(Error::command_io(command))?;

        let status = child.wait().context(Error::command_io(command))?;

        if !status.success() {
          return Err(Error::command_status(command, status));
        }

        let mut html = String::new();

        child
          .stdout
          .unwrap()
          .read_to_string(&mut html)
          .context(Error::command_io(command))?;

        Ok(html)
      }
      Markup::Markdown => {
        let mut html = String::new();
        pulldown_cmark::html::push_html(&mut html, pulldown_cmark::Parser::new(text));
        Ok(html)
      }
    }
  }
}
