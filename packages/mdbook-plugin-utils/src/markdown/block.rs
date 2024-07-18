use std::ops::Range;

use anyhow::{bail, Result};
use log::debug;
use pulldown_cmark::{Event, Parser};

#[derive(Clone, Debug, PartialEq)]
pub struct Block<'a> {
    pub closed: bool,
    pub events: Vec<(Event<'a>, Range<usize>)>,
    pub span: Range<usize>,
    pub inner_span: Range<usize>,
}

impl<'a> Block<'a> {
    pub fn new(first_event: Event<'a>, first_span: Range<usize>) -> Self {
        let span = first_span.clone();
        let inner_span = 0..0;

        Block {
            closed: false,
            events: vec![(first_event, first_span)],
            span,
            inner_span,
        }
    }
}

pub fn parse_blocks<IsStartFn, IsEndFn>(
    content: &str,
    is_start: IsStartFn,
    is_end: IsEndFn,
) -> Result<Vec<Block>>
where
    IsStartFn: Fn(&Event) -> bool,
    IsEndFn: Fn(&Event) -> bool,
{
    let mut blocks: Vec<Block> = vec![];

    for (event, span) in Parser::new(content).into_offset_iter() {
        debug!("{:?} {:?}", event, span);

        if is_start(&event) {
            if let Some(block) = blocks.last_mut() {
                if !block.closed {
                    bail!("Block is not closed. Nested blocks are not supported.");
                }
            }

            blocks.push(Block::new(event, span));
        } else if is_end(&event) {
            if let Some(block) = blocks.last_mut() {
                if !block.closed {
                    block.closed = true;
                    block.span = block.span.start..span.end;
                    block.events.push((event, span));

                    let mut seen_first = false;
                    block.events.retain(|(_, span)| {
                        if !seen_first {
                            seen_first = true;
                            true
                        } else if span.start == block.span.start && span.end != block.span.end {
                            false
                        } else {
                            span.start >= block.span.start && span.end <= block.span.end
                        }
                    });

                    if let (Some((_, first)), Some((_, last))) = (
                        block.events.get(1),
                        block.events.get(block.events.len() - 2),
                    ) {
                        block.inner_span = first.start..last.end;
                    }
                }
            }
        } else if let Some(block) = blocks.last_mut() {
            if !block.closed && span.start >= block.span.start {
                block.events.push((event, span));
            }
        }
    }

    Ok(blocks)
}

#[cfg(test)]
mod test {
    use pulldown_cmark::{CodeBlockKind, CowStr, Tag, TagEnd};
    use test_log::test;

    use super::*;

    #[test]
    fn test_parse_blocks() -> Result<()> {
        let content = "\
        ```toml\n\
        key1 = \"value1\"\n\
        key2 = \"value2\"\n\
        ```";
        let expected: Vec<Block> = vec![Block {
            closed: true,
            events: vec![
                (
                    Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(CowStr::from("toml")))),
                    0..43,
                ),
                (
                    Event::Text(CowStr::from("key1 = \"value1\"\nkey2 = \"value2\"\n")),
                    8..40,
                ),
                (Event::End(TagEnd::CodeBlock), 0..43),
            ],
            span: 0..43,
            inner_span: 8..40,
        }];

        let actual = parse_blocks(
            content,
            |event| matches!(event, Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(tag))) if tag == &CowStr::from("toml")),
            |event| matches!(event, Event::End(TagEnd::CodeBlock)),
        )?;

        assert_eq!(expected, actual);

        Ok(())
    }

    #[test]
    fn test_parse_blocks_surrounded() -> Result<()> {
        let content = "\
        Some text before the code block.\n\
        \n\
        ```toml\n\
        key1 = \"value1\"\n\
        key2 = \"value2\"\n\
        ```\n\
        \n\
        Some text after the code block.";
        let expected: Vec<Block> = vec![Block {
            closed: true,
            events: vec![
                (
                    Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(CowStr::from("toml")))),
                    34..77,
                ),
                (
                    Event::Text(CowStr::from("key1 = \"value1\"\nkey2 = \"value2\"\n")),
                    42..74,
                ),
                (Event::End(TagEnd::CodeBlock), 34..77),
            ],
            span: 34..77,
            inner_span: 42..74,
        }];

        let actual = parse_blocks(
            content,
            |event| matches!(event, Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(tag))) if tag == &CowStr::from("toml")),
            |event| matches!(event, Event::End(TagEnd::CodeBlock)),
        )?;

        assert_eq!(expected, actual);

        Ok(())
    }

    #[test]
    fn test_parse_blocks_multiple() -> Result<()> {
        let content = "\
        First TOML block:\n\
        ```toml\n\
        key1 = \"value1\"\n\
        key2 = \"value2\"\n\
        ```\n\
        First non-TOML block:\n\
        ```shell\n\
        echo test\n\
        ```\n\
        Second TOML block:\n\
        ```toml\n\
        key3 = \"value3\"\n\
        key4 = \"value4\"\n\
        ```";
        let expected: Vec<Block> = vec![
            Block {
                closed: true,
                events: vec![
                    (
                        Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(CowStr::from("toml")))),
                        18..61,
                    ),
                    (
                        Event::Text(CowStr::from("key1 = \"value1\"\nkey2 = \"value2\"\n")),
                        26..58,
                    ),
                    (Event::End(TagEnd::CodeBlock), 18..61),
                ],
                span: 18..61,
                inner_span: 26..58,
            },
            Block {
                closed: true,
                events: vec![
                    (
                        Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(CowStr::from("toml")))),
                        126..169,
                    ),
                    (
                        Event::Text(CowStr::from("key3 = \"value3\"\nkey4 = \"value4\"\n")),
                        134..166,
                    ),
                    (Event::End(TagEnd::CodeBlock), 126..169),
                ],
                span: 126..169,
                inner_span: 134..166,
            },
        ];

        let actual = parse_blocks(
            content,
            |event| matches!(event, Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(tag))) if tag == &CowStr::from("toml")),
            |event| matches!(event, Event::End(TagEnd::CodeBlock)),
        )?;

        assert_eq!(expected, actual);

        Ok(())
    }

    #[test]
    fn test_parse_blocks_nested() -> Result<()> {
        let content = "*a **sentence** with **some** words*";

        let actual = parse_blocks(
            content,
            |event| {
                matches!(
                    event,
                    Event::Start(Tag::Emphasis) | Event::Start(Tag::Strong)
                )
            },
            |event| {
                matches!(
                    event,
                    Event::End(TagEnd::Emphasis) | Event::End(TagEnd::Strong)
                )
            },
        );

        assert_eq!(
            "Block is not closed. Nested blocks are not supported.",
            format!("{}", actual.unwrap_err().root_cause())
        );

        Ok(())
    }

    #[test]
    fn test_parse_blocks_text() -> Result<()> {
        let content = "\
        {{#tab }}\n\
        Some content.\n\
        {{#endtab }}\n\
        {{#tab }}\n\
        \n\
        ```rust\n\
        let a = 1 + 2;\n\
        ```\n\
        \n\
        {{#endtab }}\n\
        ";
        let expected: Vec<Block> = vec![
            Block {
                closed: true,
                events: vec![
                    (Event::Text(CowStr::from("{{#tab }}")), 0..9),
                    (Event::SoftBreak, 9..10),
                    (Event::Text(CowStr::from("Some content.")), 10..23),
                    (Event::SoftBreak, 23..24),
                    (Event::Text(CowStr::from("{{#endtab }}")), 24..36),
                ],
                span: 0..36,
                inner_span: 9..24,
            },
            Block {
                closed: true,
                events: vec![
                    (Event::Text(CowStr::from("{{#tab }}")), 37..46),
                    (
                        Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(CowStr::from("rust")))),
                        48..74,
                    ),
                    (Event::Text(CowStr::from("let a = 1 + 2;\n")), 56..71),
                    (Event::End(TagEnd::CodeBlock), 48..74),
                    (Event::Text(CowStr::from("{{#endtab }}")), 76..88),
                ],
                span: 37..88,
                inner_span: 48..74,
            },
        ];

        let actual = parse_blocks(
            content,
            |event| matches!(event, Event::Text(text) if text.starts_with("{{#tab ")),
            |event| matches!(event, Event::Text(text) if text.starts_with("{{#endtab ")),
        )?;

        assert_eq!(expected, actual);

        Ok(())
    }
}
