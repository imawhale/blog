use crate::common::*;

pub(crate) fn copy(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> Result<(), Error> {
  let src = src.as_ref();
  let dst = dst.as_ref();

  if fs::exists(dst)? {
    return Err(Error::OutputExists {
      path: dst.to_owned(),
    });
  }

  std::fs::copy(src, dst).context(error::FilesystemCopy { src, dst })?;
  Ok(())
}

pub(crate) fn create_dir(path: impl AsRef<Path>) -> Result<(), Error> {
  let path = path.as_ref();
  std::fs::create_dir(path).context(error::FilesystemIo { path })?;
  Ok(())
}

pub(crate) fn create_dir_all(path: impl AsRef<Path>) -> Result<(), Error> {
  let path = path.as_ref();
  std::fs::create_dir_all(path).context(error::FilesystemIo { path })?;
  Ok(())
}

pub(crate) fn read_to_string(path: impl AsRef<Path>) -> Result<String, Error> {
  let path = path.as_ref();
  Ok(std::fs::read_to_string(path).context(error::FilesystemIo { path })?)
}

pub(crate) fn remove_dir_all(path: impl AsRef<Path>) -> Result<(), Error> {
  let path = path.as_ref();
  std::fs::remove_dir_all(path).context(error::FilesystemIo { path })?;
  Ok(())
}

pub(crate) fn write(path: impl AsRef<Path>, content: impl AsRef<[u8]>) -> Result<(), Error> {
  let path = path.as_ref();

  if fs::exists(path)? {
    return Err(Error::OutputExists {
      path: path.to_owned(),
    });
  }

  std::fs::write(path, content).context(error::FilesystemIo { path })?;
  Ok(())
}

pub(crate) fn metadata(path: impl AsRef<Path>) -> Result<std::fs::Metadata, Error> {
  let path = path.as_ref();
  Ok(std::fs::metadata(path).context(error::FilesystemIo { path })?)
}

pub(crate) fn exists(path: impl AsRef<Path>) -> Result<bool, Error> {
  let path = path.as_ref();
  match std::fs::metadata(path) {
    Err(err) => {
      if err.kind() == io::ErrorKind::NotFound {
        Ok(false)
      } else {
        Err(Error::FilesystemIo {
          source: err,
          path: path.to_owned(),
        })
      }
    }
    Ok(_) => Ok(true),
  }
}

pub(crate) fn render_template<T: Template>(
  path: impl AsRef<Path>,
  template: T,
) -> Result<(), Error> {
  let path = path.as_ref();
  let text = template.render().context(error::Template)?;

  let command = &[
    "tidy",
    "-indent",
    "-quiet",
    "-wrap",
    "100",
    "--tidy-mark",
    "no",
  ]
  .iter()
  .cloned()
  .map(str::to_string)
  .collect::<Vec<String>>();

  let mut child = Command::new(command[0].clone())
    .args(&command[1..])
    .stdin(Stdio::piped())
    .stdout(Stdio::piped())
    .stderr(Stdio::piped())
    .spawn()
    .context(error::CommandIo {
      command: command.clone(),
    })?;

  child
    .stdin
    .as_mut()
    .unwrap()
    .write_all(text.as_bytes())
    .context(error::CommandIo {
      command: command.clone(),
    })?;

  let output = child.wait_with_output().context(error::CommandIo {
    command: command.clone(),
  })?;

  if !output.status.success() {
    return Err(Error::CommandStatus {
      status: output.status,
      command: command.clone(),
    });
  }

  fs::write(path, output.stdout)?;

  Ok(())
}

pub(crate) fn render_index<'posts>(
  dir: &Path,
  title: Option<&str>,
  posts: impl IntoIterator<Item = &'posts Post>,
) -> Result<(), Error> {
  let index = Index::new(title, posts);

  fs::create_dir_all(&dir)?;

  let index_path = dir.join("index.html");

  fs::render_template(index_path, index)?;

  Ok(())
}
