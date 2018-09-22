use crate::common::*;

#[derive(Debug, Clone)]
pub(crate) struct Slug {
  text: String,
}

impl Slug {
  pub(crate) fn from_text(text: String) -> Result<Slug, Error> {
    let mut dash = false;

    for (i, c) in text.chars().enumerate() {
      if c.is_ascii_lowercase() {
        dash = false;
      } else if c == '-' {
        if i == 0 || dash {
          return Err(Error::Slug { text });
        }
        dash = true;
      } else {
        return Err(Error::Slug { text });
      }
    }

    if dash {
      Err(Error::Slug { text })
    } else {
      Ok(Slug { text })
    }
  }

  pub(crate) fn text(&self) -> &str {
    &self.text
  }
}

impl<S: AsRef<str>> PartialEq<S> for Slug {
  fn eq(&self, s: &S) -> bool {
    s.as_ref() == self.text
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn number() {
    assert!(Slug::from_text("0".to_string()).is_err());
  }

  #[test]
  fn uppercase() {
    assert!(Slug::from_text("A".to_string()).is_err());
  }

  #[test]
  fn double_dash() {
    assert!(Slug::from_text("a--b".to_string()).is_err());
  }

  #[test]
  fn trailing_dash() {
    assert!(Slug::from_text("a-".to_string()).is_err());
  }

  #[test]
  fn leading_dash() {
    assert!(Slug::from_text("-a".to_string()).is_err());
  }

  #[test]
  fn lowercase() -> Result<(), Error> {
    Slug::from_text("abcdefghijklmnopqrstuvwxyz".to_string())?;
    Ok(())
  }

  #[test]
  fn dashes() -> Result<(), Error> {
    Slug::from_text("a-b-c-d".to_string())?;
    Ok(())
  }
}
