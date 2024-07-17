use std::ops::Range;

use anyhow::Result;
use html_parser::{Dom, Node};
use htmlentity::entity::{decode, ICodedDataTrait};
use log::debug;
use mdbook::book::Chapter;
use pulldown_cmark::{Event, TagEnd};

use crate::{config::Config, parser::block::parse_blocks};

fn is_iframe_start(event: &Event) -> bool {
    match event {
        Event::Html(html) => html.contains("<iframe data-mdbook-trunk=\""),
        _ => false,
    }
}

fn is_iframe_end(event: &Event) -> bool {
    matches!(event, Event::End(TagEnd::HtmlBlock))
}

pub fn parse_iframes(chapter: &Chapter) -> Result<Vec<(Range<usize>, Config)>> {
    let mut configs: Vec<(Range<usize>, Config)> = vec![];

    let blocks = parse_blocks(&chapter.content, is_iframe_start, is_iframe_end)?;
    debug!("{:?}", blocks);

    for block in blocks {
        let dom = Dom::parse(&chapter.content[block.span.clone()])?;
        let element = dom
            .children
            .iter()
            .find_map(|child| match child {
                Node::Element(element) if element.name == "iframe" => Some(element),
                _ => None,
            })
            .expect("HTML content should have an iframe.");

        let config = Config::parse_from_json(
            &decode(
                element
                    .attributes
                    .get("data-mdbook-trunk")
                    .expect("Iframe should have config attribute.")
                    .as_ref()
                    .expect("Config attribute should have value.")
                    .as_bytes(),
            )
            .to_string()?,
        )?;
        configs.push((block.span, config));
    }

    Ok(configs)
}
