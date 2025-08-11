use std::ops::Range;

use anyhow::{Result, bail};
use log::debug;
use pulldown_cmark::{Event, Parser};

#[derive(Clone, Debug, PartialEq)]
pub struct Block<'a> {
    pub closed: bool,
    pub events: Vec<(Event<'a>, Range<usize>)>,
    pub span: Range<usize>,
    pub inner_span: Range<usize>,
    pub has_nested: bool,
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
            has_nested: false,
        }
    }
}

pub fn parse_blocks<IsStartFn, IsEndFn>(
    content: &str,
    is_start: IsStartFn,
    is_end: IsEndFn,
    skip_nested: bool,
) -> Result<Vec<Block<'_>>>
where
    IsStartFn: Fn(&Event) -> bool,
    IsEndFn: Fn(&Event) -> bool,
{
    let mut blocks: Vec<Block> = vec![];
    let mut nested_level = 0;

    for (event, span) in Parser::new(content).into_offset_iter() {
        debug!("{event:?} {span:?}");

        if is_start(&event) {
            if let Some(block) = blocks.last_mut()
                && !block.closed
            {
                if skip_nested {
                    nested_level += 1;
                    block.has_nested = true;
                    block.events.push((event, span));
                    continue;
                } else {
                    bail!("Block is not closed. Nested blocks are not allowed.");
                }
            }

            blocks.push(Block::new(event, span));
        } else if is_end(&event) {
            if let Some(block) = blocks.last_mut()
                && !block.closed
            {
                if nested_level > 0 {
                    nested_level -= 1;
                    block.events.push((event, span));
                    continue;
                }

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
        } else if let Some(block) = blocks.last_mut()
            && !block.closed
            && span.start >= block.span.start
        {
            block.events.push((event, span));
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
            has_nested: false,
        }];

        let actual = parse_blocks(
            content,
            |event| matches!(event, Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(tag))) if tag == &CowStr::from("toml")),
            |event| matches!(event, Event::End(TagEnd::CodeBlock)),
            false,
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
            has_nested: false,
        }];

        let actual = parse_blocks(
            content,
            |event| matches!(event, Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(tag))) if tag == &CowStr::from("toml")),
            |event| matches!(event, Event::End(TagEnd::CodeBlock)),
            false,
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
                has_nested: false,
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
                has_nested: false,
            },
        ];

        let actual = parse_blocks(
            content,
            |event| matches!(event, Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(tag))) if tag == &CowStr::from("toml")),
            |event| matches!(event, Event::End(TagEnd::CodeBlock)),
            false,
        )?;

        assert_eq!(expected, actual);

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
                has_nested: false,
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
                has_nested: false,
            },
        ];

        let actual = parse_blocks(
            content,
            |event| matches!(event, Event::Text(text) if text.starts_with("{{#tab ")),
            |event| matches!(event, Event::Text(text) if text.starts_with("{{#endtab ")),
            false,
        )?;

        assert_eq!(expected, actual);

        Ok(())
    }

    #[test]
    fn test_parse_blocks_nested_error() -> Result<()> {
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
            false,
        );

        assert_eq!(
            "Block is not closed. Nested blocks are not allowed.",
            format!("{}", actual.unwrap_err().root_cause())
        );

        Ok(())
    }

    #[test]
    fn test_parse_blocks_nested() -> Result<()> {
        let content = "\
        {{#tabs }}\n\
        Level 1\n\
        {{#tabs }}\n\
        Level 2\n\
        {{#tabs }}\n\
        Level 3\n\
        {{#endtabs }}\n\
        {{#endtabs }}\n\
        {{#endtabs }}\n\
        ";

        let expected: Vec<Block> = vec![Block {
            closed: true,
            events: vec![
                (Event::Text(CowStr::from("{{#tabs }}")), 0..10),
                (Event::SoftBreak, 10..11),
                (Event::Text(CowStr::from("Level 1")), 11..18),
                (Event::SoftBreak, 18..19),
                (Event::Text(CowStr::from("{{#tabs }}")), 19..29),
                (Event::SoftBreak, 29..30),
                (Event::Text(CowStr::from("Level 2")), 30..37),
                (Event::SoftBreak, 37..38),
                (Event::Text(CowStr::from("{{#tabs }}")), 38..48),
                (Event::SoftBreak, 48..49),
                (Event::Text(CowStr::from("Level 3")), 49..56),
                (Event::SoftBreak, 56..57),
                (Event::Text(CowStr::from("{{#endtabs }}")), 57..70),
                (Event::SoftBreak, 70..71),
                (Event::Text(CowStr::from("{{#endtabs }}")), 71..84),
                (Event::SoftBreak, 84..85),
                (Event::Text(CowStr::from("{{#endtabs }}")), 85..98),
            ],
            span: 0..98,
            inner_span: 10..85,
            has_nested: true,
        }];

        let actual = parse_blocks(
            content,
            |event| matches!(event, Event::Text(text) if text.starts_with("{{#tabs ")),
            |event| matches!(event, Event::Text(text) if text.starts_with("{{#endtabs ")),
            true,
        )?;

        assert_eq!(expected, actual);

        Ok(())
    }
}
