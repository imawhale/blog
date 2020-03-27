use crate::common::*;

pub(crate) struct Post {
  pub(crate) html: String,
  pub(crate) published: DateTime<FixedOffset>,
  pub(crate) path: String,
  pub(crate) frontmatter: Frontmatter,
  pub(crate) slug: Slug,
  pub(crate) excerpt_html: Option<String>,
}

impl Post {
  pub(crate) fn from_markup(
    path: &Path,
    markup: Markup,
    slug: &Slug,
    url_path: String,
  ) -> Result<Post, Error> {
    lazy_static! {
      static ref DELIMITER: Regex = Regex::new(r"(?m)^[+][+][+]$").unwrap();
    }

    let text = fs::read_to_string(path)?;

    let delimiter = DELIMITER
      .find(&text)
      .ok_or_else(|| Error::MissingFrontmatter {
        path: path.to_path_buf(),
      })?;

    let frontmatter =
      Frontmatter::from_yaml(&text[..delimiter.start()]).context(error::Deserialize { path })?;

    let content = &text[delimiter.end()..];

    let excerpt_html = match frontmatter.excerpt.as_ref() {
      Some(excerpt) => Some(markup.render(excerpt)?),
      None => None,
    };

    Ok(Post {
      published: frontmatter.published,
      path: url_path,
      html: markup.render(content)?,
      slug: slug.clone(),
      excerpt_html,
      frontmatter,
    })
  }

  pub(crate) fn is_hidden(&self) -> bool {
    self.frontmatter.hidden
  }

  pub(crate) fn tags(&self) -> &[Tag] {
    &self.frontmatter.tags
  }

  pub(crate) fn is_tagged_with(&self, tag: &Tag) -> bool {
    self.frontmatter.tags.contains(&tag)
  }

  pub(crate) fn title(&self) -> String {
    self
      .frontmatter
      .title
      .as_ref()
      .map(|title| title.to_owned())
      .unwrap_or_else(|| self.slug.text().to_title_case())
  }

  pub(crate) fn link(&self) -> String {
    format!("https://rodarmor.com/blog/{}", self.slug.text())
  }

  pub(crate) fn is_small(&self) -> bool {
    self.frontmatter.small
  }

  pub(crate) fn style(&self) -> Option<&str> {
    match &self.frontmatter.style {
      Some(style) => Some(style.as_str()),
      None => None,
    }
  }

  pub(crate) fn template() -> Result<String, Error> {
    let mut template = Frontmatter::template().to_yaml()?;
    template.push_str("\n+++\n");
    Ok(template)
  }

  pub(crate) fn feed_item(&self) -> Result<rss::Item, Error> {
    let guid = rss::GuidBuilder::default()
      .value(self.link())
      .permalink(true)
      .build()
      .map_err(|message| Error::FeedGuidBuild { message })?;

    rss::ItemBuilder::default()
      .title(Some(self.title()))
      .link(self.link())
      .content(self.html.clone())
      .guid(guid)
      .pub_date(self.published.to_rfc2822())
      .build()
      .map_err(|message| Error::FeedItemBuild { message })
  }
}
