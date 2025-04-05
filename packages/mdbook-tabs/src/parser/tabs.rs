use std::ops::Range;

use anyhow::{Result, bail};
use log::debug;
use mdbook::book::Chapter;
use mdbook_plugin_utils::markdown::parse_blocks;
use pulldown_cmark::Event;

use crate::config::{TabConfig, TabsConfig};

fn is_tabs_start(event: &Event) -> bool {
    match event {
        Event::Text(text) => {
            (text.to_string() == "{{#tabs}}" || text.starts_with("{{#tabs"))
                && !text.contains("{{#endtabs")
        }
        _ => false,
    }
}

fn is_tabs_end(event: &Event) -> bool {
    match event {
        Event::Text(text) => {
            (text.to_string() == "{{#endtabs}}" || text.starts_with("{{#endtabs "))
                && !text.contains("{{#tabs ")
        }
        _ => false,
    }
}

fn is_tab_start(event: &Event) -> bool {
    match event {
        Event::Text(text) => text.to_string() == "{{#tab}}" || text.starts_with("{{#tab "),
        _ => false,
    }
}

fn is_tab_end(event: &Event) -> bool {
    match event {
        Event::Text(text) => text.to_string() == "{{#endtab}}" || text.starts_with("{{#endtab "),
        _ => false,
    }
}

type SpanAndTabs = (Range<usize>, TabsConfig);

pub fn parse_tabs(chapter: &Chapter) -> Result<(Vec<SpanAndTabs>, bool)> {
    let mut configs: Vec<(Range<usize>, TabsConfig)> = vec![];

    let blocks = parse_blocks(&chapter.content, is_tabs_start, is_tabs_end, true)?;
    debug!("{:?}", blocks);

    for block in &blocks {
        let start_text = match &block.events[0].0 {
            Event::Text(text) => text.to_string(),
            _ => bail!("First event should be text."),
        };

        let mut tabs = TabsConfig::parse(
            start_text
                .trim_start_matches("{{#tabs")
                .trim_start()
                .trim_end_matches("}}")
                .trim_end(),
        )?;

        let subblocks = parse_blocks(
            &chapter.content[block.inner_span.clone()],
            is_tab_start,
            is_tab_end,
            true,
        )?;
        debug!("{:?}", subblocks);

        for subblock in subblocks {
            let start_text = match &subblock.events[0].0 {
                Event::Text(text) => text.to_string(),
                _ => bail!("First event should be text."),
            };

            tabs.tabs.push((
                TabConfig::parse(
                    start_text
                        .trim_start_matches("{{#tab")
                        .trim_start()
                        .trim_end_matches("}}")
                        .trim_end(),
                )?,
                chapter.content[(block.inner_span.start + subblock.inner_span.start)
                    ..(block.inner_span.start + subblock.inner_span.end)]
                    .to_string(),
            ));
        }

        configs.push((block.span.clone(), tabs));
    }

    debug!("{:?}", configs);

    Ok((configs, blocks.iter().any(|block| block.has_nested)))
}
