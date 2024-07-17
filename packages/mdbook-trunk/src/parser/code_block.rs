use std::ops::Range;

use anyhow::Result;
use log::debug;
use mdbook::book::Chapter;
use pulldown_cmark::{CodeBlockKind, Event, Tag, TagEnd};

use crate::{config::Config, parser::block::parse_blocks};

fn is_code_block_start(event: &Event) -> bool {
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

fn is_code_block_end(event: &Event) -> bool {
    matches!(event, Event::End(TagEnd::CodeBlock))
}

pub fn parse_code_blocks(chapter: &Chapter) -> Result<Vec<(Range<usize>, Config)>> {
    let mut configs: Vec<(Range<usize>, Config)> = vec![];

    let blocks = parse_blocks(&chapter.content, is_code_block_start, is_code_block_end)?;
    debug!("{:?}", blocks);

    for block in blocks {
        let config = Config::parse_from_toml(&chapter.content[block.inner_span.clone()])?;
        configs.push((block.span, config));
    }

    debug!("{:?}", configs);

    Ok(configs)
}
