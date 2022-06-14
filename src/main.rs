mod error;

use std::process;

use handlebars;
use clap::{Parser, Subcommand, Args};
use serde_json::json;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Options {
  #[clap(long)]
  debug: bool,
  #[clap(subcommand)]
  command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
  #[clap(about="Generate documentation from an API description")]
  Generate(GenerateOptions),
}

#[derive(Args, Debug)]
struct GenerateOptions {
  #[clap(long, short='b', help="The base URL to resolve against")]
  base: String,
  #[clap(help="The URL to resolve against the base; if a URL is not provided it is read from STDIN")]
  url: Option<String>,
}

fn main() {
  match cmd() {
    Ok(_)     => {},
    Err(err)  => {
      println!("* * * {}", err);
      process::exit(1);
    },
  };
}

fn cmd() -> Result<(), error::Error> {
  let opts = Options::parse();
  match &opts.command {
    Command::Generate(sub) => generate(&opts, &sub),
  }
}

fn generate(_: &Options, cmd: &GenerateOptions) -> Result<(), error::Error> {
  let mut hdl = handlebars::Handlebars::new();
  println!("{}", hdl.render_template("{{ yo }} duder, ok.", &json!({"yo": 100}))?);
  Ok(())
}
