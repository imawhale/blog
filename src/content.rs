use crate::*;

pub(crate) struct Content {
  pub(crate) frontmatter: Frontmatter,
  pub(crate) body: String,
}

impl Content {
  pub(crate) fn load(path: &Path) -> Result<Self> {
    let text = fs::read_to_string(path)?;

    let delimiter = Regex::new(r"(?m)^[+][+][+]$").unwrap();

    let frontmatter_delimiter = delimiter
      .find(&text)
      .ok_or_else(|| Error::MissingFrontmatter {
        path: path.to_path_buf(),
      })?;

    let mut frontmatter = Frontmatter::from_yaml(&text[..frontmatter_delimiter.start()])
      .context(error::Deserialize { path })?;

    let rest = &text[frontmatter_delimiter.end()..];

    let body = if let Some(excerpt_delimiter) = delimiter.find(&rest) {
      if frontmatter.excerpt.is_some() {
        return Err(Error::DuplicateExcerpt {
          path: path.to_path_buf(),
        });
      }

      let excerpt = &rest[..excerpt_delimiter.start()];
      let rest = &rest[excerpt_delimiter.end()..];

      frontmatter.excerpt = Some(excerpt.to_owned());

      format!("{}\n{}", excerpt, rest)
    } else {
      rest.to_owned()
    };

    Ok(Content { frontmatter, body })
  }
}
