use std::{ops::Range, path::Path, process::Command, str};

use anyhow::{bail, Result};
use cargo::core::Workspace;
use htmlentity::entity::{encode, EncodeType, ICodedDataTrait};
use log::{debug, error};
use mdbook::book::Chapter;
use pulldown_cmark::{CodeBlockKind, Event, Tag, TagEnd};

use crate::{config::Config, parser::parse_blocks};

fn is_start_event(event: &Event) -> bool {
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

fn is_end_event(event: &Event) -> bool {
    matches!(event, Event::End(TagEnd::CodeBlock))
}

pub fn parse_chapter(chapter: &Chapter) -> Result<Vec<(Range<usize>, Config)>> {
    let mut configs: Vec<(Range<usize>, Config)> = vec![];

    let blocks = parse_blocks(&chapter.content, is_start_event, is_end_event)?;
    debug!("{:?}", blocks);

    for block in blocks {
        let config = Config::parse(&chapter.content[block.inner_span.clone()])?;
        configs.push((block.span, config));
    }

    Ok(configs)
}

pub fn iframe(config: &Config) -> Result<String> {
    Ok(format!(
        "<iframe src=\"/{}/index.html\" data-mdbook-trunk=\"{}\" style=\"border: none;\"></iframe>",
        config.dest_name(),
        encode(
            serde_json::to_string(config)?.as_bytes(),
            &EncodeType::Named,
            &htmlentity::entity::CharacterSet::SpecialChars
        )
        .to_string()?
    ))
}

pub fn build(workspace: &Workspace, config: Config, dest_dir: &Path) -> Result<()> {
    let package_root = config.package_root(workspace)?;

    let output = Command::new("trunk")
        .arg("build")
        .arg("--dist")
        .arg(dest_dir)
        .current_dir(package_root)
        .output()?;

    if !output.status.success() {
        error!("{}", str::from_utf8(&output.stdout)?);
        bail!("Trunk build of package `{}` failed.", config.package);
    }

    Ok(())
}
