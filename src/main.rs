mod error;
mod model;
mod slug;

use std::fs;
use std::io;
use std::ffi;
use std::path;
use std::process;

use handlebars::{self, handlebars_helper};
use clap::{Parser, Subcommand, Args};

use model::{Content, Route};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Options {
  #[clap(long)]
  debug: bool,
  #[clap(long)]
  verbose: bool,
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
  #[clap(long, short='o', help="The output document root to write files under")]
  output: Option<String>,
  #[clap(long, short='x', help="The path to write an index document to; if omitted, no index is generated")]
  index: Option<String>,
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
  handlebars_helper!(slug: |v: Route| v.slug());
  
  hdl.register_helper("render", Box::new(render));
  hdl.register_helper("slug", Box::new(slug));
  hdl.register_template_string("suite", tmpl)?;
  
  for input in &cmd.docs {
    if opt.verbose {
      println!("----> {}", input);
    }
    
    let data = fs::read_to_string(input)?;
    let mut writer: Box<dyn io::Write> = match &cmd.output {
      Some(output) => Box::new(fs::OpenOptions::new().write(true).create(true).truncate(true).open(output_path(input, output, "html")?)?),
      None => Box::new(io::stdout()),
    };
    
    let mut suite: model::Suite = serde_json::from_str(&data)?;
    suite.process(model::Meta{
      generated: chrono::Utc::now(),
    });
    
    if opt.debug {
      println!("{}", serde_json::to_string_pretty(&suite)?);
    }
    
    writer.write(hdl.render("suite", &suite)?.as_bytes())?;
  }
  
  Ok(())
}

fn output_path<P: AsRef<path::Path>>(input: P, root: &str, ext: &str) -> Result<ffi::OsString, error::Error> {
  fs::create_dir_all(root)?;
  
  let name = match input.as_ref().file_name() {
    Some(name) => name,
    None => ffi::OsStr::new("api"),
  };
  
  let mut buf = path::PathBuf::new();
  buf.push(root);
  buf.push(name);
  buf.set_extension(ext);
  
  Ok(buf.into_os_string())
}
