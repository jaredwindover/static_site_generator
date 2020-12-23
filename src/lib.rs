use std::fs;
use std::path::Path;

#[macro_use]
extern crate horrorshow;
use horrorshow::prelude::*;
use horrorshow::helper::doctype;

#[macro_use]
extern crate derive_error;

pub mod config;
use config::Config;

pub mod err;
use err::Result;

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
				let directory_name = path.file_name().unwrap().to_str();

				let index_output = format!("{}", html! {
						: doctype::HTML;
						html {
								head {
										title: directory_name;
										meta(charset="utf-8");
										link(type="text/css", rel="stylesheet", href="/css/main.css");
								}
								body {
										h1: directory_name;
										ul {
												: render_list(&out_path)
										}
								}
						}
				});

				out_path.push("index.html");
				fs::write(out_path, index_output).unwrap();
		}

		Ok(())
}

fn render_list(path: &Path) -> Box<dyn RenderBox> {
		let paths = fs::read_dir(path)
				.unwrap()
				.filter(|entry| entry.as_ref().unwrap().file_name() != "index.html");
		box_html! {
				@ for entry in paths {
						: Raw(render_entry(entry.unwrap()))
				}
		}
}

fn render_entry(entry: fs::DirEntry) -> String {
		let path = entry.path();
		let name = path.file_stem().unwrap();
		let file_name = path.file_name().unwrap();
		(html!{
				li {
						h2 {
								a(href=file_name.to_str()): name.to_str()
						}
				}
		})
				.into_string()
				.unwrap()
}

pub fn run(config: Config) -> Result<()> {
		let in_path = config.get_in_path()?;
		handle_dir(&in_path, &config)?;
		Ok(())
}
