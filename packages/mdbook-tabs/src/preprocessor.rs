use std::str;

use anyhow::Result;
use mdbook::{
    book::Book,
    preprocess::{Preprocessor, PreprocessorContext},
    BookItem,
};

// use crate::{parser::definition::parse_definitions, trunk::iframe};

pub struct TabsPreprocessor;

impl TabsPreprocessor {
    pub fn new() -> Self {
        Self
    }
}

impl Default for TabsPreprocessor {
    fn default() -> Self {
        Self::new()
    }
}

impl Preprocessor for TabsPreprocessor {
    fn name(&self) -> &str {
        "tabs"
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
            // let blocks = parse_definitions(chapter)?;
            // for (span, config) in blocks {
            //     chapter.content.replace_range(span, &iframe(&config)?);
            // }

            process_items(&mut chapter.sub_items)?;
        }
    }

    Ok(())
}
