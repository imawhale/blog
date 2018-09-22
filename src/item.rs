use crate::common::*;

#[derive(Debug)]
pub(crate) struct Item {
  pub(crate) in_path: PathBuf,
  pub(crate) out_path: PathBuf,
  pub(crate) slug: Slug,
  pub(crate) kind: ItemKind,
}

impl Item {
  pub(crate) fn from_path(path: &Path) -> Result<Item, Error> {
    let file_stem = path
      .file_stem()
      .ok_or_else(|| Error::ItemMissingStem {
        path: path.to_path_buf(),
      })?
      .to_str()
      .ok_or_else(|| Error::NonUtf8Path {
        path: path.to_path_buf(),
      })?
      .to_string();

    let slug = Slug::from_text(file_stem)?;

    let out_path = Path::new(OUT_PATH).join(path.strip_prefix(IN_PREFIX).unwrap());

    Ok(Item {
      in_path: path.to_path_buf(),
      kind: ItemKind::from_path(path)?,
      out_path,
      slug,
    })
  }

  pub(crate) fn render(self) -> Result<Option<Post>, Error> {
    use ItemKind::*;
    match self.kind {
      Markup { kind } => {
        let parent_out_directory = self.out_path.parent().unwrap();

        let output_path = if self.slug == "index" {
          parent_out_directory.join("index.html")
        } else {
          let output_directory = parent_out_directory.join(self.slug.text()).to_path_buf();
          fs::create_dir(&output_directory)?;
          output_directory.join("index.html")
        };

        let url_path = {
          let relative_path = output_path
            .parent()
            .unwrap()
            .strip_prefix(OUT_PREFIX)
            .iter()
            .map(|component| {
              component.to_str().ok_or_else(|| Error::NonUtf8Path {
                path: self.in_path.to_path_buf(),
              })
            })
            .collect::<Result<Vec<&str>, Error>>()?
            .join("/");

          format!("{}{}/", URL_PATH_PREFIX, relative_path)
        };

        let post = Post::from_markup(&self.in_path, kind, &self.slug, url_path)?;

        {
          let index = Index::new(Some(&post.title()), vec![&post]);

          fs::render_template(output_path, index)?;
        }

        if !post.is_hidden() {
          Ok(Some(post))
        } else {
          Ok(None)
        }
      }
      Sass => {
        let options = sass_rs::Options {
          output_style: sass_rs::OutputStyle::Compressed,
          precision: 6,
          indented_syntax: false,
          include_paths: vec!["sass".to_string()],
        };

        let css =
          sass_rs::compile_file(&self.in_path, options).map_err(|sass_error| Error::Sass {
            sass_error,
            path: self.in_path.to_path_buf(),
          })?;

        let mut out_filename = self.out_path.file_stem().unwrap().to_os_string();
        out_filename.push(".css");

        let out_path = self.out_path.parent().unwrap().join(&out_filename);

        fs::write(&out_path, &css)?;

        Ok(None)
      }
      Blob => {
        fs::copy(&self.in_path, &self.out_path)?;

        Ok(None)
      }
      Directory => {
        fs::create_dir(&self.out_path)?;

        Ok(None)
      }
    }
  }
}
