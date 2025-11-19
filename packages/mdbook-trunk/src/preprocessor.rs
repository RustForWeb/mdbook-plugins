use std::{env, str};

use anyhow::Result;
use cargo::{GlobalContext, core::Workspace, util::important_paths::find_root_manifest_for_wd};
use mdbook_preprocessor::{
    Preprocessor, PreprocessorContext,
    book::{Book, BookItem},
};

use crate::{parser::definition::parse_definitions, trunk::trunk};

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

    fn run(&self, _ctx: &PreprocessorContext, book: Book) -> Result<Book> {
        let mut book = book.clone();

        let gctx = GlobalContext::default()?;
        let workspace = Workspace::new(&find_root_manifest_for_wd(&env::current_dir()?)?, &gctx)?;

        process_items(&workspace, &mut book.items)?;

        Ok(book)
    }

    fn supports_renderer(&self, _renderer: &str) -> Result<bool> {
        Ok(true)
    }
}

fn process_items(workspace: &Workspace, items: &mut Vec<BookItem>) -> Result<()> {
    for section in items {
        if let BookItem::Chapter(chapter) = section {
            let blocks = parse_definitions(chapter)?;

            let mut offset: usize = 0;

            for (span, config) in blocks {
                let replacement = trunk(workspace, &config)?;

                chapter
                    .content
                    .replace_range((span.start + offset)..(span.end + offset), &replacement);

                offset += replacement.len() - span.len();
            }

            process_items(workspace, &mut chapter.sub_items)?;
        }
    }

    Ok(())
}
