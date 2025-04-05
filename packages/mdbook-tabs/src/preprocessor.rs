use std::str;

use anyhow::{Result, bail};
use mdbook::{
    BookItem,
    book::Book,
    preprocess::{Preprocessor, PreprocessorContext},
};

use crate::{parser::tabs::parse_tabs, tabs::tabs};

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
            loop {
                let (configs, has_nested) = parse_tabs(chapter)?;

                let mut offset: isize = 0;

                for (span, config) in configs {
                    let replacement = tabs(&config);

                    let start = span.start as isize + offset;
                    let end = span.end as isize + offset;
                    if start < 0 || end < 0 {
                        bail!("Negative range {}..{}.", start, end);
                    }

                    chapter
                        .content
                        .replace_range(start as usize..end as usize, &replacement);

                    offset += replacement.len() as isize - span.len() as isize;
                }

                if !has_nested {
                    break;
                }
            }

            process_items(&mut chapter.sub_items)?;
        }
    }

    Ok(())
}
