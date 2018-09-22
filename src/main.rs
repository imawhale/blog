mod common;
mod error;
mod filters;
mod frontmatter;
mod fs;
mod index;
mod item;
mod item_kind;
mod markup;
mod opt;
mod post;
mod slug;
mod tag;

use crate::common::*;

fn main() {
  if let Err(error) = Opt::from_args().run() {
    eprintln!("error: {:?}", error);
    process::exit(EXIT_FAILURE);
  }
}
