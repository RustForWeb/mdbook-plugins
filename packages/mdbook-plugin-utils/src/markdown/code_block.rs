use anyhow::Result;
use pulldown_cmark::{CodeBlockKind, Event, Tag, TagEnd};

use crate::markdown::block::{parse_blocks, Block};

fn is_code_block_start<IsTagsFn>(is_tags: IsTagsFn) -> Box<dyn Fn(&Event) -> bool>
where
    IsTagsFn: Fn(Vec<String>) -> bool + 'static,
{
    Box::new(move |event: &Event| match event {
        Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(tag))) => {
            let tags = tag
                .split(',')
                .map(|tag| tag.trim().to_lowercase())
                .collect::<Vec<_>>();

            is_tags(tags)
        }
        _ => false,
    })
}

fn is_code_block_end(event: &Event) -> bool {
    matches!(event, Event::End(TagEnd::CodeBlock))
}

pub fn parse_code_blocks<IsTagsFn>(content: &str, is_tags: IsTagsFn) -> Result<Vec<Block>>
where
    IsTagsFn: Fn(Vec<String>) -> bool + 'static,
{
    parse_blocks(content, is_code_block_start(is_tags), is_code_block_end)
}
