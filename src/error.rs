use crate::common::*;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub(crate) enum Error {
  DuplicateExcerpt {
    path: PathBuf,
  },
  FilesystemIo {
    source: io::Error,
    path: PathBuf,
  },
  FilesystemCopy {
    src: PathBuf,
    dst: PathBuf,
    source: io::Error,
  },
  ItemMissingStem {
    path: PathBuf,
  },
  NonUtf8Path {
    path: PathBuf,
  },
  Template {
    source: askama::Error,
  },
  Deserialize {
    source: serde_yaml::Error,
    path: PathBuf,
  },
  Serialize {
    source: serde_yaml::Error,
  },
  Sass {
    sass_error: String,
    path: PathBuf,
  },
  MissingFrontmatter {
    path: PathBuf,
  },
  MissingExtension {
    path: PathBuf,
  },
  UnrecognizedExtension {
    path: PathBuf,
  },
  CommandIo {
    command: Vec<String>,
    source: io::Error,
  },
  CommandStatus {
    command: Vec<String>,
    status: ExitStatus,
  },
  CommandOutput {
    command: Vec<String>,
    stderr: String,
    stdout: String,
    status: ExitStatus,
  },
  Slug {
    text: String,
  },
  OutputExists {
    path: PathBuf,
  },
  Git {
    path: PathBuf,
    source: git2::Error,
  },
  Head {
    head: Option<String>,
  },
  Dirty,
  Notify {
    source: notify::Error,
  },
  ThreadJoin,
  Editor {
    source: env::VarError,
  },
  FeedBuild {
    message: String,
  },
  FeedItemBuild {
    message: String,
  },
  FeedGuidBuild {
    message: String,
  },
  FeedRender {
    source: rss::Error,
  },
}

pub(crate) struct CommandArgs<I> {
  command: I,
}

impl<I, T> Into<Vec<String>> for CommandArgs<I>
where
  I: IntoIterator<Item = T>,
  T: AsRef<str>,
{
  fn into(self) -> Vec<String> {
    self
      .command
      .into_iter()
      .map(|arg| arg.as_ref().to_string())
      .collect()
  }
}

impl Error {
  pub(crate) fn command_io<I, T>(command: I) -> CommandIo<CommandArgs<I>>
  where
    I: IntoIterator<Item = T>,
    T: AsRef<str>,
  {
    CommandIo {
      command: CommandArgs { command },
    }
  }

  pub(crate) fn command_status<I, T>(command: I, status: ExitStatus) -> Error
  where
    I: IntoIterator<Item = T>,
    T: AsRef<str>,
  {
    Error::CommandStatus {
      command: CommandArgs { command }.into(),
      status,
    }
  }
}
