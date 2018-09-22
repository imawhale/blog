use crate::common::*;

use Tag::*;

#[derive(Serialize, Deserialize, Copy, Clone, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub(crate) enum Tag {
  Cryptocurrency,
  Programming,
  Fiction,
}

impl Tag {
  pub(crate) fn name(self) -> &'static str {
    match self {
      Cryptocurrency => "cryptocurrency",
      Programming => "programming",
      Fiction => "fiction",
    }
  }

  pub(crate) fn path(self) -> String {
    format!("{}tag/{}/", URL_PATH_PREFIX, self.name())
  }

  pub(crate) fn all() -> &'static [Tag] {
    &[Cryptocurrency, Programming, Fiction]
  }
}
