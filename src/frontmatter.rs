use crate::common::*;

use serde_yaml;

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub(crate) struct Frontmatter {
  pub(crate) published: DateTime<FixedOffset>,
  #[serde(default, skip_serializing_if = "is_default")]
  pub(crate) title: Option<String>,
  #[serde(default, skip_serializing_if = "is_default")]
  pub(crate) small: bool,
  #[serde(default, skip_serializing_if = "is_default")]
  pub(crate) hidden: bool,
  #[serde(default, skip_serializing_if = "is_default")]
  pub(crate) tags: Vec<Tag>,
  #[serde(default, skip_serializing_if = "is_default")]
  pub(crate) excerpt: Option<String>,
  #[serde(default, skip_serializing_if = "is_default")]
  pub(crate) style: Option<String>,
}

impl Frontmatter {
  pub(crate) fn from_yaml(text: &str) -> Result<Frontmatter, serde_yaml::Error> {
    serde_yaml::from_str(&text)
  }

  pub(crate) fn to_yaml(&self) -> Result<String, Error> {
    let yaml = serde_yaml::to_string(self).context(error::Serialize)?;
    assert_eq!(&yaml[0..4], "---\n");
    Ok(yaml[4..].to_owned())
  }

  pub(crate) fn template() -> Frontmatter {
    Frontmatter {
      published: Local::now().into(),
      title: None,
      small: false,
      hidden: false,
      tags: Vec::new(),
      excerpt: None,
      style: None,
    }
  }
}

fn is_default<T: Default + PartialEq>(t: &T) -> bool {
  t == &T::default()
}
