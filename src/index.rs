use crate::common::*;

use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
pub(crate) struct Index<'posts> {
  title: String,
  posts: Vec<&'posts Post>,
}

impl<'posts> Index<'posts> {
  pub(crate) fn new(
    title: Option<&str>,
    posts: impl IntoIterator<Item = &'posts Post>,
  ) -> Index<'posts> {
    let mut posts = posts.into_iter().collect::<Vec<&Post>>();

    posts.sort_by_key(|post| post.published);

    posts.reverse();

    let title = if let Some(title) = title {
      format!("{} – Casey Rodarmor's Blog", title)
    } else {
      "Casey Rodarmor's Blog".to_string()
    };

    Index { title, posts }
  }

  pub(crate) fn style(&self) -> Option<&str> {
    if self.posts.len() == 1 {
      self.posts[0].style()
    } else {
      None
    }
  }
}
