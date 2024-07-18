use std::{
    env, fs,
    io::{self, Read},
};

use anyhow::{anyhow, Result};
use clap::{Args, Parser, Subcommand};
use log::warn;
use mdbook::{
    preprocess::{CmdPreprocessor, Preprocessor},
    MDBook,
};
use mdbook_tabs::TabsPreprocessor;
use semver::{Version, VersionReq};

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Install,
    Supports(SupportsArgs),
}

#[derive(Args)]
struct SupportsArgs {
    renderer: String,
}

fn main() -> Result<()> {
    let mut logger = env_logger::builder();
    if env::var("RUST_LOG").is_err() {
        logger.filter_module("mdbook_tabs", log::LevelFilter::Info);
    }
    logger.init();

    let cli = Cli::parse();
    let preprocessor = TabsPreprocessor::new();

    match &cli.command {
        Some(subcommand) => match subcommand {
            Commands::Install => handle_install(),
            Commands::Supports(args) => handle_supports(&preprocessor, args),
        },
        None => handle_preprocessing(&preprocessor, io::stdin()),
    }
}

fn handle_install() -> Result<()> {
    let book = MDBook::load(env::current_dir()?)?;
    let directory = book.root.join("theme");

    if !directory.exists() {
        fs::create_dir(&directory)?;
    }

    let css_content = include_str!("../theme/tabs.css");
    let js_content = include_str!("../theme/tabs.js");

    fs::write(directory.join("tabs.css"), css_content)?;
    fs::write(directory.join("tabs.js"), js_content)?;

    Ok(())
}

fn handle_supports(
    preprocessor: &dyn Preprocessor,
    SupportsArgs { renderer }: &SupportsArgs,
) -> Result<()> {
    match preprocessor.supports_renderer(renderer) {
        true => Ok(()),
        false => Err(anyhow!("Renderer `{renderer}` is not supported.")),
    }
}

fn handle_preprocessing<R: Read>(preprocessor: &dyn Preprocessor, reader: R) -> Result<()> {
    let (ctx, book) = CmdPreprocessor::parse_input(reader)?;

    let book_version = Version::parse(&ctx.mdbook_version)?;
    let version_req = VersionReq::parse(mdbook::MDBOOK_VERSION)?;

    if !version_req.matches(&book_version) {
        warn!(
            "The {} plugin was built against version {} of mdbook, but we're being called from version {}",
            preprocessor.name(),
            mdbook::MDBOOK_VERSION,
            ctx.mdbook_version
        );
    }

    let processed_book = preprocessor.run(&ctx, book)?;
    serde_json::to_writer(io::stdout(), &processed_book)?;

    Ok(())
}
