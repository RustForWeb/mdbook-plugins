use std::{
    error::Error,
    io::{self, Read},
};

use clap::{Args, Parser, Subcommand};
use mdbook::{
    preprocess::{CmdPreprocessor, Preprocessor},
    renderer::{HtmlHandlebars, RenderContext},
    Renderer,
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
    Supports(SupportsArgs),
}

#[derive(Args)]
struct SupportsArgs {
    renderer: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let cli = Cli::parse();
    let preprocessor = TrunkPreprocessor::new();
    let renderer = TrunkRenderer::new();
    let html_renderer = HtmlHandlebars::new();

    match &cli.command {
        Some(subcommand) => match subcommand {
            Commands::Supports(args) => handle_supports(&preprocessor, args),
        },
        None => {
            let mut reader = BufPeekReader::new(io::stdin());

            let mut buffer = [0; 1];
            reader.peek().read_exact(&mut buffer)?;

            if buffer[0] == b'[' {
                handle_preprocessing(&preprocessor, reader)
            } else {
                handle_renderer(&renderer, &html_renderer, reader)
            }
        }
    }
}

fn handle_supports(
    preprocessor: &dyn Preprocessor,
    SupportsArgs { renderer }: &SupportsArgs,
) -> Result<(), Box<dyn Error>> {
    match preprocessor.supports_renderer(renderer) {
        true => Ok(()),
        false => Err(format!("Renderer `{renderer}` is not supported.").into()),
    }
}

fn handle_preprocessing<R: Read>(
    preprocessor: &dyn Preprocessor,
    reader: R,
) -> Result<(), Box<dyn Error>> {
    let (ctx, book) = CmdPreprocessor::parse_input(reader)?;

    let book_version = Version::parse(&ctx.mdbook_version)?;
    let version_req = VersionReq::parse(mdbook::MDBOOK_VERSION)?;

    if !version_req.matches(&book_version) {
        eprintln!(
            "Warning: The {} plugin was built against version {} of mdbook, but we're being called from version {}",
            preprocessor.name(),
            mdbook::MDBOOK_VERSION,
            ctx.mdbook_version
        );
    }

    let processed_book = preprocessor.run(&ctx, book)?;
    serde_json::to_writer(io::stdout(), &processed_book)?;

    Ok(())
}

fn handle_renderer<R: Read>(
    renderer: &dyn Renderer,
    html_renderer: &dyn Renderer,
    reader: R,
) -> Result<(), Box<dyn Error>> {
    let ctx = RenderContext::from_json(reader).unwrap();

    renderer.render(&ctx)?;
    html_renderer.render(&ctx)?;

    Ok(())
}
