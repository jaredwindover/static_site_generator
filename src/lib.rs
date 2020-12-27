use std::fs;
use std::io::prelude::*;
use std::path::Path;

#[macro_use]
extern crate horrorshow;
use horrorshow::prelude::*;

#[macro_use]
extern crate derive_error;

pub mod config;
use config::{Config, DirectoryConfig};

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

fn get_dir_config_from_path(path: &Path) -> Result<DirectoryConfig> {
		let mut dir_config_contents = String::new();
		let dir_config = path.join(".ssg.toml");
		let mut dir_config = fs::File::open(dir_config)?;
		dir_config.read_to_string(&mut dir_config_contents)?;
		let dir_config = toml::from_str(&dir_config_contents)?;
		Ok(dir_config)
}

fn get_dir_config_or_default(path: &Path) -> DirectoryConfig {
		get_dir_config_from_path(path)
				.unwrap_or_else(|_| DirectoryConfig::default())
}

fn build_index(path: &Path, config: &Config, out_path: &Path) {
		let index_config = get_dir_config_or_default(path).index;
		let index_output = format!(
				"{}",
				render_index(path, config, &index_config).into_string().unwrap());

		fs::write(out_path.join("index.html"), index_output).unwrap();
}

fn handle_dir(path: &Path, config: &Config) -> Result<()> {
		let out_path = config.get_relative_out_path(path).unwrap();
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
				build_index(path, config, &out_path);
		}

		Ok(())
}

pub fn run(config: Config) -> Result<()> {
		let in_path = config.get_in_path()?;
		handle_dir(&in_path, &config)?;
		Ok(())
}
