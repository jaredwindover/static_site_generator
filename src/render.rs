use std::fs;
use std::path::Path;

use horrorshow::prelude::*;
use horrorshow::helper::doctype;

use crate::config::{Config, IndexConfig};

pub fn render_index<'a>(
		path: &'a Path,
		config: &'a Config,
		index_config: &'a IndexConfig
) -> Box<dyn Render +'a> {
		let title = index_config.title.as_ref().map(String::as_str)
				.or_else(|| path.file_name().unwrap().to_str());

		let page_title = index_config.page_title.as_ref()
				.map(String::as_str)
				.or(title);

		let out_path = config.get_relative_out_path(path);

		box_html! {
				: doctype::HTML;
				html {
						head {
								title: title;
								meta(charset="utf-8");
								link(type="text/css", rel="stylesheet", href="/css/main.css");
						}
						body {
								h1: page_title;
								ul {
										: render_list(&out_path, index_config)
								}
						}
				}
		}
}

fn render_list<'a>(path: &Path, index_config: &'a IndexConfig) -> Box<dyn RenderBox + 'a> {
		let default_excludes = &["index.html", ".ssg.toml"];
		let paths = fs::read_dir(path)
				.unwrap()
				.filter(move |entry| {
						let file_name = entry.as_ref().unwrap().file_name();
						let name = file_name.to_str();
						!index_config.exclude.iter().map(String::as_str)
								.chain(default_excludes.iter().map(|s| *s))
								.any(|e| name.map(|n| n == e).unwrap_or(false))
				});
		box_html! {
				@ for entry in paths {
						: render_path(&entry.unwrap().path())
				}
		}
}

fn render_path<'a>(path: &'a Path) -> Box<dyn RenderBox + 'a> {
		let name = path.file_stem().unwrap();
		let file_name = path.file_name().unwrap();
		box_html!{
				li {
						h2 {
								a(href=file_name.to_str()): name.to_str()
						}
				}
		}
}
