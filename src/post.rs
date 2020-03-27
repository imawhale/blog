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
    let content = Content::load(path)?;

    let excerpt_html = if let Some(excerpt) = &content.frontmatter.excerpt {
      Some(markup.render(&excerpt)?)
    } else {
      None
    };

    Ok(Post {
      published: content.frontmatter.published,
      path: url_path,
      html: markup.render(&content.body)?,
      slug: slug.clone(),
      frontmatter: content.frontmatter,
      excerpt_html,
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
