// This is free and unencumbered software released into the public domain.

use clap::Parser;
use clientele::StandardOptions;
use std::io::Write as _;

/// asimov-solana-fetcher
#[derive(Debug, Parser)]
#[command(arg_required_else_help = true)]
struct Options {
    #[clap(flatten)]
    flags: StandardOptions,

    /// The output format.
    #[arg(value_name = "FORMAT", short = 'o', long)]
    output: Option<String>,

    urls: Vec<String>,
}

#[cfg(feature = "std")]
fn main() -> Result<clientele::SysexitsError, Box<dyn std::error::Error>> {
    use asimov_solana_module::BlockUrl;
    use clientele::SysexitsError::*;
    use serde_json;
    use std::io::stdout;

    clientele::dotenv().ok();

    let args = asimov_module::args_os()?;
    let options = Options::parse_from(args);

    if options.flags.version {
        println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
        return Ok(EX_OK);
    }

    if options.flags.license {
        print!("{}", include_str!("../../UNLICENSE"));
        return Ok(EX_OK);
    }

    #[cfg(feature = "tracing")]
    asimov_module::init_tracing_subscriber(&options.flags).expect("failed to initialize logging");

    if options.urls.is_empty() {
        return Ok(EX_OK);
    }

    let mut stdout = stdout().lock();
    let output_jsonl = options.output.as_deref() == Some("jsonl");

    for url in options.urls {
        let block: BlockUrl = url.parse()?;
        let block_json = block.fetch()?;

        let line = if output_jsonl {
            let value: serde_json::Value = serde_json::from_str(&block_json)?;
            serde_json::to_string(&value)?
        } else {
            block_json
        };

        match writeln!(&mut stdout, "{}", &line) {
            Ok(_) => (),
            Err(err) if err.kind() == std::io::ErrorKind::BrokenPipe => break,
            Err(err) => return Err(err.into()),
        }
    }

    Ok(EX_OK)
}

#[cfg(not(feature = "std"))]
fn main() {
    unimplemented!("asimov-solana-fetcher requires the 'std' feature")
}
