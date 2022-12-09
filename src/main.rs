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
use chrono::{DateTime, Utc};

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
  handlebars_helper!(format_date: |v: DateTime<Utc>| v.format("%b %e, %Y %R").to_string());
  
  hdl.register_helper("render", Box::new(render));
  hdl.register_helper("slug", Box::new(slug));
  hdl.register_helper("format_date", Box::new(format_date));
  hdl.register_template_string("suite", suite_tmpl)?;
  
  let mut entries: Option<Vec<model::Entry>> = match &index_tmpl {
    Some(index_tmpl) => {
      hdl.register_template_string("index", index_tmpl)?;
      Some(Vec::new())
    },
    None => None,
  };
  let (base, index) = match &cmd.output {
    Some(output) => (Some(output), Some(output_path("index", output, "html")?)),
    None => (None, None),
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
    
    let mut context: model::Suite = serde_json::from_str(&data)?;
    context.process(model::Meta{
      index: convert_osstr(&index)?,
      generated: chrono::Utc::now(),
    });
    
    if let Some(output) = output {
      if let Some(entries) = &mut entries {
        let title = match &context.title {
          Some(title) => title.to_owned(),
          None => format_path(input)?,
        };
        let url = match (&output).to_str() {
          Some(unwrap) => unwrap.to_owned(),
          None => "#invalid".to_owned(),
        };
        let rel = if let Some(index) = &index {
          let rel = match base {
            Some(base) => relative_path(&ffi::OsString::from(&base), &output),
            None => relative_path(index, &output),
          };
          match rel.to_str() {
            Some(unwrap) => unwrap.to_owned(),
            None => "#invalid".to_owned(),
          }
        }else{
          url
        };
        entries.push(model::Entry{
          link: model::Link{
            title: Some(title),
            url: rel,
          },
        });
      }
    }
    
    if opt.debug {
      println!("{}", serde_json::to_string_pretty(&context)?);
    }
    
    writer.write(hdl.render("suite", &context)?.as_bytes())?;
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
    let mut context = model::Index{
      title: Some(cmd.title.to_owned()),
      detail: None,
      entries: entries,
      meta: None,
    };
    context.process(model::Meta{
      index: convert_osstr(&index)?,
      generated: chrono::Utc::now(),
    });
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

fn relative_path<P: AsRef<path::Path>>(a: P, b: P) -> path::PathBuf {
  let ca: Vec<_> = a.as_ref().components().collect();
  let cb: Vec<_> = b.as_ref().components().collect();
  
  let mut n = 0;
  for i in 0..ca.len() {
    if ca[i] != cb[i] {
      break;
    }
    n = i + 1;
  }
  
  let u = ca.len() - n;
  let mut c = path::PathBuf::new();
  for _ in 0..u {
    c.push("..");
  }
  for i in n..cb.len() {
    c.push(cb[i]);
  }
  
  c
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

#[cfg(test)]
mod tests {
  use super::*;
  
  #[test]
  fn test_relative_path() {
    assert_eq!(path::Path::new("b.foo"), relative_path(path::Path::new("a"), path::Path::new("a/b.foo")).as_path());
    assert_eq!(path::Path::new("b.foo"), relative_path(path::Path::new("/a"), path::Path::new("/a/b.foo")).as_path());
    
    assert_eq!(path::Path::new("b/c/d.foo"), relative_path(path::Path::new("/a"), path::Path::new("/a/b/c/d.foo")).as_path());
    assert_eq!(path::Path::new("c/d.foo"), relative_path(path::Path::new("/a/b"), path::Path::new("/a/b/c/d.foo")).as_path());
    assert_eq!(path::Path::new("d.foo"), relative_path(path::Path::new("/a/b/c"), path::Path::new("/a/b/c/d.foo")).as_path());
    assert_eq!(path::Path::new(""), relative_path(path::Path::new("/a/b/c/d.foo"), path::Path::new("/a/b/c/d.foo")).as_path());
    
    assert_eq!(path::Path::new("../c/d.foo"), relative_path(path::Path::new("/a/b/x"), path::Path::new("/a/b/c/d.foo")).as_path());
    assert_eq!(path::Path::new("../../c/d.foo"), relative_path(path::Path::new("/a/b/x/y"), path::Path::new("/a/b/c/d.foo")).as_path());
    assert_eq!(path::Path::new("../../c/d/e/f.foo"), relative_path(path::Path::new("/a/b/x/y"), path::Path::new("/a/b/c/d/e/f.foo")).as_path());
    assert_eq!(path::Path::new("../../../../f.foo"), relative_path(path::Path::new("/a/b/c/d/e"), path::Path::new("/a/f.foo")).as_path());
  }
  
}
