use std::{
    env, fs,
    io::{self, Read},
};

use anyhow::{Result, anyhow};
use clap::{Args, Parser, Subcommand};
use fs_extra::dir::{CopyOptions, DirOptions, copy, get_dir_content2};
use log::warn;
use mdbook::{
    MDBook, Renderer,
    preprocess::{CmdPreprocessor, Preprocessor},
    renderer::RenderContext,
};
use mdbook_trunk::{TrunkPreprocessor, TrunkRenderer};
use peekread::{BufPeekReader, PeekRead};
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
    Combine,
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
        logger.filter_module("mdbook_trunk", log::LevelFilter::Info);
    }
    logger.init();

    let cli = Cli::parse();
    let preprocessor = TrunkPreprocessor::new();
    let renderer = TrunkRenderer::new();

    match &cli.command {
        Some(subcommand) => match subcommand {
            Commands::Combine => handle_combine(),
            Commands::Install => handle_install(),
            Commands::Supports(args) => handle_supports(&preprocessor, args),
        },
        None => {
            let mut reader = BufPeekReader::new(io::stdin());

            let mut buffer = [0; 1];
            reader.peek().read_exact(&mut buffer)?;

            if buffer[0] == b'[' {
                handle_preprocessing(&preprocessor, reader)
            } else {
                handle_renderer(&renderer, reader)
            }
        }
    }
}

fn handle_combine() -> Result<()> {
    let book = MDBook::load(env::current_dir()?)?;

    let build_dir = book.root.join(&book.config.build.build_dir);
    let build_dir_str = build_dir.to_str().unwrap();
    let dest_dir = book.root.join("dist");
    let dest_dir_str = dest_dir.to_str().unwrap();

    log::info!("Combining into directory `{dest_dir_str}`.");

    if dest_dir.exists() {
        log::info!("Directory exists, recreating.");
        fs::remove_dir_all(&dest_dir)?;
    }
    fs::create_dir(&dest_dir)?;

    let mut dir_options = DirOptions::new();
    dir_options.depth = 1;

    for directory in get_dir_content2(&build_dir, &dir_options)?.directories {
        if directory == build_dir_str {
            continue;
        }

        log::info!("Adding directory `{directory}`.");
        copy(
            build_dir.join(directory),
            &dest_dir,
            &CopyOptions::new().content_only(true),
        )?;
    }

    Ok(())
}

fn handle_install() -> Result<()> {
    let book = MDBook::load(env::current_dir()?)?;
    let directory = book.root.join("theme");

    if !directory.exists() {
        fs::create_dir(&directory)?;
    }

    let css_content = include_str!("../theme/trunk.css");
    let js_content = include_str!("../theme/trunk.js");

    fs::write(directory.join("trunk.css"), css_content)?;
    fs::write(directory.join("trunk.js"), js_content)?;

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

fn handle_renderer<R: Read>(renderer: &dyn Renderer, reader: R) -> Result<()> {
    let ctx = RenderContext::from_json(reader).unwrap();

    let book_version = Version::parse(&ctx.version)?;
    let version_req = VersionReq::parse(mdbook::MDBOOK_VERSION)?;

    if !version_req.matches(&book_version) {
        warn!(
            "The {} plugin was built against version {} of mdbook, but we're being called from version {}",
            renderer.name(),
            mdbook::MDBOOK_VERSION,
            ctx.version
        );
    }

    renderer.render(&ctx)?;

    Ok(())
}
