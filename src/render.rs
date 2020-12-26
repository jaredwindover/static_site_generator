use std::fs;
use std::path::Path;

use horrorshow::prelude::*;
use horrorshow::helper::doctype;

use crate::config::Config;

pub fn render_index<'a>(path: &'a Path, config: &'a Config) -> Box<dyn Render +'a> {
		let out_path = config.get_relative_out_path(path).unwrap();
		let directory_name = path.file_name().unwrap().to_str();
		box_html! {
				: doctype::HTML;
				html {
						head {
								title:& directory_name;
								meta(charset="utf-8");
								link(type="text/css", rel="stylesheet", href="/css/main.css");
						}
						body {
								h1: &directory_name;
								ul {
										: render_list(&out_path)
								}
						}
				}
		}
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
