use crate::common::*;

use Tag::*;

#[derive(Serialize, Deserialize, Copy, Clone, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub(crate) enum Tag {
  Cryptocurrency,
  Fiction,
  Programming,
  Sharing,
}

impl Tag {
  pub(crate) fn name(self) -> &'static str {
    match self {
      Cryptocurrency => "cryptocurrency",
      Fiction => "fiction",
      Programming => "programming",
      Sharing => "sharing",
    }
  }

  pub(crate) fn path(self) -> String {
    format!("{}tag/{}/", URL_PATH_PREFIX, self.name())
  }

  pub(crate) fn all() -> &'static [Tag] {
    &[Cryptocurrency, Fiction, Programming, Sharing]
  }
}
