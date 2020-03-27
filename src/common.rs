pub(crate) use std::{
  collections::HashMap,
  env,
  fs::File,
  io::{self, Read, Write},
  iter::IntoIterator,
  path::{Path, PathBuf},
  process::{self, Command, ExitStatus, Stdio},
};

pub(crate) use askama::Template;
pub(crate) use chrono::{DateTime, FixedOffset, Local};
pub(crate) use inflector::Inflector;
pub(crate) use libc::EXIT_FAILURE;
pub(crate) use regex::Regex;
pub(crate) use serde::{Deserialize, Serialize};
pub(crate) use snafu::{ResultExt, Snafu};
pub(crate) use structopt::StructOpt;
pub(crate) use walkdir::WalkDir;

pub(crate) const IN_PATH: &str = "in";
pub(crate) const IN_PREFIX: &str = "in/";
pub(crate) const OUT_PATH: &str = "www/blog";
pub(crate) const OUT_PREFIX: &str = "www/blog/";
pub(crate) const URL_PATH_PREFIX: &str = "/blog/";

pub(crate) use crate::{error, fs};

pub(crate) use crate::{
  content::Content, error::Error, frontmatter::Frontmatter, index::Index, item::Item,
  item_kind::ItemKind, markup::Markup, opt::Opt, post::Post, slug::Slug, tag::Tag,
};

pub(crate) type Result<T, E = Error> = std::result::Result<T, E>;
