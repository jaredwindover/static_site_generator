use std::path::Path;
use std::fs::{self, File};
use std::io::BufReader;
use std::io::prelude::*;

use comrak::{self, Arena, ComrakOptions};
use horrorshow::prelude::*;
use horrorshow::helper::doctype;

use crate::config::Config;
use crate::err::Result;
use crate::frontmatter::strip_yaml;

pub fn handle_markdown_file(path: &Path, config: &Config) -> Result<()> {
		let arena = Arena::new();

    let file = File::open(path)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    let (index_config, rest) = strip_yaml(&contents);
		let index_config = index_config.as_ref();
		let mut options = ComrakOptions::default();
		options.render.unsafe_ = true;

    let root = comrak::parse_document(
        &arena,
        &rest,
        &options,
    );

    let mut html_out = vec![];
    comrak::format_html(root, &options, &mut html_out).unwrap();
    let html_out = String::from_utf8(html_out).unwrap();

    let file_name = path.file_stem().unwrap().to_str();

    let title: Option<String> = index_config
        .map(|ic| { ic.title.clone() })
        .flatten()
        .or(file_name.map(String::from));

		let is_draft: bool = index_config
				.map(|ic| { ic.is_draft })
				.flatten()
				.unwrap_or(false);

		let build_draft: bool = config.build_draft();

		// Just exit if this is a draft file and we aren't building the draft.
		if is_draft && !build_draft {
				return Ok(());
		}

		let full_out = format!("{}", html! {
				: doctype::HTML;
				html {
						head {
								title: title.as_ref();
								meta(charset="utf-8");
								link(type="text/css", rel="stylesheet", href="/css/main.css");
						}
						body {
								: Raw(&html_out)
						}
				}
		});

		if is_draft {
				let draft_path = config.get_relative_out_draft_path(path).map(|it| it.with_extension("html")).unwrap();
				println!("{:?}", &draft_path);
		}

		if build_draft {
				let draft_path = config.get_relative_out_draft_path(path).map(|it| it.with_extension("html")).unwrap();
				fs::write(draft_path, &full_out).unwrap();
		}

		if !is_draft {
				let new_path = config.get_relative_out_path(path).with_extension("html");
				fs::write(new_path, full_out).unwrap();
		}

    Ok(())
}
