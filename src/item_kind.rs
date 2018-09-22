use crate::common::*;

#[derive(Debug)]
pub(crate) enum ItemKind {
  Markup { kind: Markup },
  Sass,
  Directory,
  Blob,
}

impl ItemKind {
  pub(crate) fn from_path(path: &Path) -> Result<ItemKind, Error> {
    let metadata = fs::metadata(path)?;

    if metadata.is_dir() {
      return Ok(ItemKind::Directory);
    }

    match path.extension() {
      None => Err(Error::MissingExtension {
        path: path.to_path_buf(),
      }),
      Some(extension) => match extension.to_str().unwrap() {
        "scss" => Ok(ItemKind::Sass),
        "md" => Ok(ItemKind::Markup {
          kind: Markup::Markdown,
        }),
        "html" if path.file_stem().unwrap() != Path::new("index") => {
          Ok(ItemKind::Markup { kind: Markup::Html })
        }
        "adoc" => Ok(ItemKind::Markup {
          kind: Markup::AsciiDoc,
        }),
        "svg" => Ok(ItemKind::Blob),
        _ => Err(Error::UnrecognizedExtension {
          path: path.to_path_buf(),
        }),
      },
    }
  }
}
