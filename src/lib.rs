use std::fs;
use std::path::Path;

#[macro_use]
extern crate horrorshow;
use horrorshow::prelude::*;

#[macro_use]
extern crate derive_error;

pub mod config;
use config::Config;

pub mod err;
use err::Result;

mod render;
use render::render_index;

mod markdown;
use markdown::handle_markdown_file;

fn handle_file(path: &Path, config: &Config) -> Result<()> {
		match path
				.extension()
				.and_then(|ext| ext.to_str()) {
				Some("md") => handle_markdown_file(path, config),
				_ => copy_file(path, config)
		}
}

fn copy_file(path: &Path, config: &Config) -> Result<()> {
		let new_path = config.get_relative_out_path(path)?;
		fs::copy(path, new_path)?;
		Ok(())
}

fn handle_dir(path: &Path, config: &Config) -> Result<()> {
		let mut out_path = config.get_relative_out_path(path).unwrap();
		fs::create_dir_all(&out_path)?;
		let paths = fs::read_dir(path)?;
		let mut has_index = false;
		for entry in paths {
				if let Ok(entry) = entry {
						let t = entry.file_type()?;
						if t.is_file() {
								let file_name = entry.file_name();
								has_index |= file_name == "index.html";
								handle_file(&entry.path(), config).unwrap();
						} else if t.is_dir() {
								handle_dir(&entry.path(), config).unwrap();
						}
				}
		}

		if !has_index {
				let index_output = format!("{}", render_index(path, config).into_string().unwrap());

				out_path.push("index.html");
				fs::write(out_path, index_output).unwrap();
		}

		Ok(())
}

pub fn run(config: Config) -> Result<()> {
		let in_path = config.get_in_path()?;
		handle_dir(&in_path, &config)?;
		Ok(())
}
