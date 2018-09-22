use crate::common::*;

#[derive(StructOpt)]
pub(crate) enum Opt {
  Post { slug: String },
  Render,
}

impl Opt {
  pub(crate) fn run(self) -> Result<(), Error> {
    use Opt::*;

    match self {
      Post { slug } => Self::post(slug),
      Render => Self::render(),
    }
  }

  pub(crate) fn render() -> Result<(), Error> {
    if Path::new(OUT_PATH).is_dir() {
      fs::remove_dir_all(OUT_PATH)?;
    }

    let items = WalkDir::new(IN_PATH)
      .into_iter()
      .filter_entry(|entry| !entry.file_name().to_string_lossy().starts_with("."))
      .map(Result::unwrap)
      .map(|item| Item::from_path(item.path()))
      .collect::<Result<Vec<Item>, Error>>()?;

    let mut posts = items
      .into_iter()
      .map(Item::render)
      .collect::<Result<Vec<Option<Post>>, Error>>()?
      .into_iter()
      .flatten()
      .collect::<Vec<Post>>();

    posts.sort_by_key(|post| post.published);

    fs::render_index(Path::new(OUT_PATH), None, &posts)?;

    let tags_dir = Path::new(OUT_PATH).join("tag");
    fs::create_dir(&tags_dir)?;

    for tag in Tag::all() {
      fs::render_index(
        &tags_dir.join(tag.name()),
        Some(tag.name()),
        posts.iter().filter(|post| post.is_tagged_with(tag)),
      )?;
    }

    fs::render_index(
      &tags_dir.join("smol"),
      Some("smol"),
      posts.iter().filter(|post| post.is_small()),
    )?;

    let feed_items = posts
      .iter()
      .map(Post::feed_item)
      .collect::<Result<Vec<rss::Item>, Error>>()?;

    let channel = rss::ChannelBuilder::default()
      .title("Casey Rodarmor's Blog")
      .link("https://rodarmor.com/blog")
      .language(Some("en".into()))
      .copyright(Some("Copyright and related rights waived via CC0".into()))
      .description("smoods")
      .last_build_date(Local::now().to_rfc2822())
      .namespaces(
        [(
          "content".to_owned(),
          "http://purl.org/rss/1.0/modules/content/".to_owned(),
        )]
        .iter()
        .cloned()
        .collect::<HashMap<String, String>>(),
      )
      .items(feed_items)
      .build()
      .map_err(|message| Error::FeedBuild { message })?;

    let feed_path = Path::new(OUT_PATH).join("feed.xml");
    let feed = File::create(&feed_path).context(error::FilesystemIo { path: feed_path })?;

    channel
      .pretty_write_to(feed, b' ', 2)
      .context(error::FeedRender)?;

    Ok(())
  }

  pub(crate) fn post(slug: String) -> Result<(), Error> {
    let slug = Slug::from_text(slug)?;

    let name = slug.text();

    let path = format!("{}/{}.md", IN_PATH, name);

    if !Path::new(&path).exists() {
      let template = Post::template()?;
      fs::write(&path, template)?;
    }

    let editor = env::var("EDITOR").context(error::Editor)?;

    let status = Command::new(&editor)
      .arg(&path)
      .status()
      .map_err(|source| Error::CommandIo {
        command: vec![editor.clone(), path.clone()],
        source,
      })?;

    if !status.success() {
      return Err(Error::CommandStatus {
        status,
        command: vec![editor, path.clone()],
      });
    }

    Ok(())
  }
}
