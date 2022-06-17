mod error;
mod model;

use std::fs;
use std::io;
use std::process;

use handlebars::{self, handlebars_helper};
use clap::{Parser, Subcommand, Args};
use serde_json::json;

use model::Content;

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
  #[clap(long, short='t', help="The template document to use for rendering")]
  template: String,
  #[clap(long, short='o', help="The output document path")]
  output: Option<String>,
  #[clap(help="Documents to process")]
  docs: Vec<String>,
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

fn generate(opt: &Options, cmd: &GenerateOptions) -> Result<(), error::Error> {
  let tmpl = fs::read_to_string(&cmd.template)?;
  let mut hdl = handlebars::Handlebars::new();
  
  handlebars_helper!(render: |v: Content| v.text());
  hdl.register_helper("render", Box::new(render));
  hdl.register_template_string("suite", tmpl);
  
  let mut out: Box<dyn io::Write> = match &cmd.output {
    Some(path) => Box::new(fs::OpenOptions::new().write(true).open(path)?),
    None => Box::new(io::stdout()),
  };
  
  for path in &cmd.docs {
    let data = fs::read_to_string(path)?;
    let suite: model::Suite = serde_json::from_str(&data)?;
    if opt.debug {
      println!(">>> {:?}", suite);
    }
    out.write(hdl.render("suite", &suite)?.as_bytes())?;
  }
  
  Ok(())
}
