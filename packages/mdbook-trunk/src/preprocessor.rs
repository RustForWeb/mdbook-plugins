use std::str;

use anyhow::Result;
use mdbook::{
    book::Book,
    preprocess::{Preprocessor, PreprocessorContext},
    BookItem,
};

use crate::{parser::definition::parse_definitions, trunk::iframe};

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

        process_items(&mut book.sections)?;

        Ok(book)
    }

    fn supports_renderer(&self, _renderer: &str) -> bool {
        true
    }
}

fn process_items(items: &mut Vec<BookItem>) -> Result<()> {
    for section in items {
        if let BookItem::Chapter(chapter) = section {
            let blocks = parse_definitions(chapter)?;

            let mut offset: usize = 0;

            for (span, config) in blocks {
                let replacement = iframe(&config)?;

                chapter
                    .content
                    .replace_range((span.start + offset)..(span.end + offset), &replacement);

                offset += replacement.len() - span.len();
            }

            process_items(&mut chapter.sub_items)?;
        }
    }

    Ok(())
}
