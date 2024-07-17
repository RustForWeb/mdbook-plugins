use std::str;

use anyhow::Result;
use mdbook::{
    book::Book,
    preprocess::{Preprocessor, PreprocessorContext},
    BookItem,
};

use crate::{parser::code_block::parse_code_blocks, trunk::iframe};

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

        for section in &mut book.sections {
            if let BookItem::Chapter(chapter) = section {
                let blocks = parse_code_blocks(chapter)?;
                for (span, config) in blocks {
                    chapter.content.replace_range(span, &iframe(&config)?);
                }
            }
        }

        Ok(book)
    }

    fn supports_renderer(&self, _renderer: &str) -> bool {
        true
    }
}
