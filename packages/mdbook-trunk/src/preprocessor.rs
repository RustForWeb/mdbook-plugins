use std::{path::PathBuf, process::Command};

use fs_extra::{
    copy_items,
    dir::{copy, CopyOptions},
};
use log::debug;
use mdbook::{
    book::{Book, Chapter},
    preprocess::{Preprocessor, PreprocessorContext},
    BookItem,
};
use pulldown_cmark::{CodeBlockKind, Event, Tag, TagEnd};
use tempfile::tempdir;

use crate::{config::Config, parser::parse_blocks};

pub struct TrunkPreprocessor;

impl TrunkPreprocessor {
    pub fn new() -> Self {
        Self
    }
}

impl Default for TrunkPreprocessor {
    fn default() -> Self {
        Self::new()
    }
}

impl Preprocessor for TrunkPreprocessor {
    fn name(&self) -> &str {
        "trunk"
    }

    fn run(&self, ctx: &PreprocessorContext, book: Book) -> Result<Book, mdbook::errors::Error> {
        let mut book = book.clone();

        let src_dir = ctx.root.join(&ctx.config.book.src);
        let dest_dir = ctx.root.join(&ctx.config.build.build_dir);

        for section in &mut book.sections {
            if let BookItem::Chapter(chapter) = section {
                let src_path = src_dir.join(
                    chapter
                        .source_path
                        .as_ref()
                        .expect("Chapter should have source path."),
                );
                let dest_path =
                    dest_dir.join(chapter.path.as_ref().expect("Chapter should have path."));

                process_chapter(chapter, src_path, dest_path)?;
            }
        }

        Ok(book)
    }

    fn supports_renderer(&self, renderer: &str) -> bool {
        renderer == "html"
    }
}

fn is_start_event(event: &Event) -> bool {
    if let Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(tag))) = event {
        let tags = tag
            .split(',')
            .map(|tag| tag.trim().to_lowercase())
            .collect::<Vec<_>>();
        tags.len() >= 2 && tags[0] == "toml" && tags[1] == "trunk"
    } else {
        false
    }
}

fn is_end_event(event: &Event) -> bool {
    matches!(event, Event::End(TagEnd::CodeBlock))
}

fn process_chapter(
    chapter: &mut Chapter,
    src_path: PathBuf,
    dest_path: PathBuf,
) -> Result<(), mdbook::errors::Error> {
    let src_dir = src_path
        .parent()
        .expect("Source path should have a parent.");
    let _dest_dir = dest_path
        .parent()
        .expect("Destination path should have a parent.");

    let blocks = parse_blocks(&chapter.content, is_start_event, is_end_event)?;
    debug!("{:?}", blocks);

    for block in blocks {
        let config = Config::parse(&chapter.content[block.inner_span])?;

        let temp = tempdir()?;
        copy(
            std::fs::canonicalize(src_dir.join(config.template))?,
            temp.path(),
            &CopyOptions::new().content_only(true),
        )?;
        copy_items(
            &config
                .files
                .iter()
                .map(|file| src_dir.join(file))
                .collect::<Vec<_>>(),
            temp.path().join("src"),
            &CopyOptions::new(),
        )?;

        debug!(
            "{}",
            std::str::from_utf8(&Command::new("tree").arg(temp.path()).output()?.stdout)?
        );

        // Command::new("trunk").arg("--dist").arg();

        // chapter.content[block.span] = ;
    }

    Ok(())
}
