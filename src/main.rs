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
  #[clap(long, short='T', help="The title of this documentation suite")]
  title: String,
  #[clap(long, short='t', help="The template document to use for rendering suites")]
  template: String,
  #[clap(long, short='o', help="The output document root to write files under")]
  output: Option<String>,
  #[clap(long, short='x', help="The template document to use for rendering an index; if omitted, no index is generated")]
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
  let mut hdl = handlebars::Handlebars::new();
  let suite_tmpl = fs::read_to_string(&cmd.template)?;
  let index_tmpl = match &cmd.index {
    Some(indx) => Some(fs::read_to_string(indx)?),
    None => None,
  };
  
  handlebars_helper!(render: |v: Content| v.text());
  handlebars_helper!(slug: |v: Route| v.slug());
  
  hdl.register_helper("render", Box::new(render));
  hdl.register_helper("slug", Box::new(slug));
  hdl.register_template_string("suite", suite_tmpl)?;
  
  let mut entries: Option<Vec<model::Entry>> = match &index_tmpl {
    Some(index_tmpl) => {
      hdl.register_template_string("index", index_tmpl)?;
      Some(Vec::new())
    },
    None => None,
  };
  let index = match &cmd.output {
    Some(output) => Some(output_path("index", output, "html")?),
    None => None,
  };
  
  for input in &cmd.docs {
    if opt.verbose {
      println!("----> {}", input);
    }
    
    let data = fs::read_to_string(input)?;
    let output = match &cmd.output {
      Some(output) => Some(output_path(input, output, "html")?),
      None => None,
    };
    let mut writer: Box<dyn io::Write> = match &output {
      Some(output) => Box::new(fs::OpenOptions::new().write(true).create(true).truncate(true).open(output)?),
      None => Box::new(io::stdout()),
    };
    
    let mut suite: model::Suite = serde_json::from_str(&data)?;
    suite.process(model::Meta{
      index: convert_osstr(&index)?,
      generated: chrono::Utc::now(),
    });
    
    if let Some(output) = output {
      if let Some(entries) = &mut entries {
        let title = match &suite.title {
          Some(title) => title.to_owned(),
          None => format_path(input)?,
        };
        let url = match output.to_str() {
          Some(unwrap) => unwrap.to_owned(),
          None => "#invalid".to_owned(),
        };
        entries.push(model::Entry{
          link: model::Link{
            title: Some(title),
            url: url,
          },
        });
      }
    }
    
    if opt.debug {
      println!("{}", serde_json::to_string_pretty(&suite)?);
    }
    
    writer.write(hdl.render("suite", &suite)?.as_bytes())?;
  }
  
  if let Some(entries) = entries {
    let output = match &cmd.output {
      Some(output) => Some(output_path("index", output, "html")?),
      None => None,
    };
    let mut writer: Box<dyn io::Write> = match output {
      Some(output) => Box::new(fs::OpenOptions::new().write(true).create(true).truncate(true).open(output)?),
      None => Box::new(io::stdout()),
    };
    let context = model::Index{
      title: Some(cmd.title.to_owned()),
      detail: None,
      entries: entries,
      meta: None,
    };
    writer.write(hdl.render("index", &context)?.as_bytes())?;
  }
  
  Ok(())
}

fn convert_osstr(input: &Option<ffi::OsString>) -> Result<Option<String>, error::Error> {
  let input = match input {
    Some(input) => input,
    None => return Ok(None),
  };
  match input.to_str() {
    Some(input) => Ok(Some(input.to_owned())),
    None => Err(error::Error::ConversionError(format!("Invalid string: {:?}", input))),
  }
}

fn format_path<P: AsRef<path::Path>>(input: P) -> Result<String, error::Error> {
  let name = match input.as_ref().file_name() {
    Some(name) => name,
    None => return Err(error::Error::ConversionError("Invalid path".to_string())),
  };
  match name.to_str() {
    Some(name) => Ok(name.to_owned()),
    None => return Err(error::Error::ConversionError("Invalid path".to_string())),
  }
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
