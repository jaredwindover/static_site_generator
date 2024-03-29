use std::env;
use std::path::{PathBuf, Path};
use serde::Deserialize;

#[derive(Debug, Deserialize, Default)]
pub struct IndexConfig {
    #[serde(default)]
    pub page_title: Option<String>,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub exclude: Vec<String>,
    #[serde(default)]
		pub no_index: bool
}

impl IndexConfig {
    pub fn default() -> IndexConfig {
        IndexConfig {
            page_title: Some(String::from("dir")),
            title: Some(String::from("dir")),
            exclude: vec![String::from("")],
						no_index: false
				}
    }
}

#[derive(Debug, Deserialize)]
pub struct DirectoryConfig {
    #[serde(default)]
    pub index: IndexConfig,
}

impl DirectoryConfig {
    pub fn default() -> DirectoryConfig {
        DirectoryConfig {
            index: IndexConfig::default()
        }
    }
}

pub struct Config {
		in_directory: Option<String>,
		out_directory: Option<String>,
		draft_directory: Option<String>
}

impl Config {
    pub fn new(mut args: env::Args) -> std::result::Result<Config, &'static str> {
        // Ignore path to exe.
        args.next();

				// Literally just arguments in order.
				// This could be made more sophisticated.
				let in_directory = args.next();
				let out_directory = args.next();
				let draft_directory = args.next();

				Ok(Config {in_directory, out_directory, draft_directory})
		}

    pub fn get_in_path(&self) -> PathBuf {
        Config::get_path_or_default(&self.in_directory, "src")
    }

    pub fn get_out_path(&self) -> PathBuf {
        Config::get_path_or_default(&self.out_directory, "out")
    }

		pub fn get_draft_path(&self) -> Option<PathBuf> {
				let mut out_path = env::current_dir()
						.expect("How can we not get a directory?");
				self.draft_directory.as_ref().map(|s| {
						out_path.push(s);
						out_path
				})
		}

		pub fn build_draft(&self) -> bool {
				self.draft_directory.is_some()
		}

		pub fn get_relative_out_path(&self, path: &Path) -> PathBuf {
				let out_path = self.get_out_path();
				let relative_path = self.get_relative_path(path);
				out_path.join(relative_path)
		}

		pub fn get_relative_out_draft_path(&self, path: &Path) -> Option<PathBuf> {
				let draft_path = self.get_draft_path();
				let relative_path = self.get_relative_path(path);
				draft_path.map(|dp| {
						dp.join(relative_path)
				})
		}

		pub fn get_relative_path<'a>(&self, path: &'a Path) -> &'a Path {
				let in_path = self.get_in_path();
				path.strip_prefix(in_path).unwrap()
		}

		fn get_path_or_default(path: &Option<String>, postfix: &str) -> PathBuf {
				path.as_ref().map_or_else(|| {
						let mut path = env::current_dir()
								.expect("How can we not get a directory?");
						path.push(postfix);
						path
				}, |s| PathBuf::from(&s))
		}
}
