use std::path::Path;
use std::fs::{self, File};
use std::io::BufReader;
use std::io::prelude::*;

use comrak::{self, Arena, ComrakOptions};
use horrorshow::prelude::*;
use horrorshow::helper::doctype;

use crate::config::Config;
use crate::err::Result;

pub fn handle_markdown_file(path: &Path, config: &Config) -> Result<()> {
		let new_path = config.get_relative_out_path(path)?.with_extension("html");

		let arena = Arena::new();

		let file = File::open(path)?;
		let mut buf_reader = BufReader::new(file);
		let mut contents = String::new();
		buf_reader.read_to_string(&mut contents)?;

		let root = comrak::parse_document(
				&arena,
				&contents,
				&ComrakOptions::default()
		);

		let mut html_out = vec![];
		comrak::format_html(root, &ComrakOptions::default(), &mut html_out).unwrap();
		let html_out = String::from_utf8(html_out).unwrap();

		let file_name = path.file_stem().unwrap().to_str();

		let full_out = format!("{}", html! {
				: doctype::HTML;
				html {
						head {
								title: file_name;
								meta(charset="utf-8");
								link(type="text/css", rel="stylesheet", href="/css/main.css");
						}
						body {
								: Raw(&html_out)
						}
				}
		});

		fs::write(new_path, full_out).unwrap();

		Ok(())
}
