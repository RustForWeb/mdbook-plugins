use std::env;

use anyhow::Result;
use cargo::{core::Workspace, util::important_paths::find_root_manifest_for_wd, GlobalContext};
use mdbook::{renderer::RenderContext, BookItem, Renderer};

use crate::{parser::iframe::parse_iframes, trunk::build};

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

        for section in &ctx.book.sections {
            if let BookItem::Chapter(chapter) = section {
                let blocks = parse_iframes(chapter)?;
                for (_, config) in blocks {
                    let dest_dir = ctx.destination.join(config.dest_name());

                    build(&workspace, config, &dest_dir)?;
                }
            }
        }

        Ok(())
    }
}
