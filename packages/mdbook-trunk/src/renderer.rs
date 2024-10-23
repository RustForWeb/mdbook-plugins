use std::{collections::HashSet, env, thread};

use anyhow::{anyhow, Result};
use cargo::{core::Workspace, util::important_paths::find_root_manifest_for_wd, GlobalContext};
use mdbook::{renderer::RenderContext, BookItem, Renderer};

use crate::{config::BuildConfig, parser::iframe::parse_iframes, trunk::build};

pub struct TrunkRenderer;

impl TrunkRenderer {
    pub fn new() -> Self {
        Self
    }
}

impl Default for TrunkRenderer {
    fn default() -> Self {
        TrunkRenderer::new()
    }
}

impl Renderer for TrunkRenderer {
    fn name(&self) -> &str {
        "trunk"
    }

    fn render(&self, ctx: &RenderContext) -> Result<()> {
        let gctx = GlobalContext::default()?;
        let workspace = Workspace::new(&find_root_manifest_for_wd(&env::current_dir()?)?, &gctx)?;

        let builds = process_items(&ctx.book.sections)?;

        let mut handles = vec![];
        for build_config in builds {
            let package_root = build_config.package_root(&workspace)?;
            let dest_dir = ctx.destination.join(build_config.dest_name());

            handles.push(thread::spawn(move || {
                build(build_config, &package_root, &dest_dir)
            }));
        }

        for handle in handles {
            handle.join().map_err(|err| anyhow!("{:?}", err))??;
        }

        Ok(())
    }
}

fn process_items(items: &Vec<BookItem>) -> Result<HashSet<BuildConfig>> {
    let mut builds = HashSet::new();

    for section in items {
        if let BookItem::Chapter(chapter) = section {
            let blocks = parse_iframes(chapter)?;
            for (_, config) in blocks {
                builds.insert(config.build_config());
            }

            builds.extend(process_items(&chapter.sub_items)?);
        }
    }

    Ok(builds)
}
