use std::ops::Range;

use anyhow::Result;
use log::debug;
use mdbook::book::Chapter;
use mdbook_plugin_utils::markdown::parse_code_blocks;

use crate::config::Config;

fn is_tags(tags: Vec<String>) -> bool {
    tags.len() >= 2 && tags[0] == "toml" && tags[1] == "trunk"
}

pub fn parse_definitions(chapter: &Chapter) -> Result<Vec<(Range<usize>, Config)>> {
    let mut configs: Vec<(Range<usize>, Config)> = vec![];

    let blocks = parse_code_blocks(&chapter.content, is_tags)?;
    debug!("{:?}", blocks);

    for block in blocks {
        let config = Config::parse_from_toml(&chapter.content[block.inner_span.clone()])?;
        configs.push((block.span, config));
    }

    debug!("{:?}", configs);

    Ok(configs)
}
