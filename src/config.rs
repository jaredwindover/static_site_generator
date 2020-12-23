use std::env;
use std::path::{PathBuf, Path};

use crate::err::Result;

pub struct Config {
		in_directory: Option<String>,
		out_directory: Option<String>
}

impl Config {
		pub fn new(mut args: env::Args) -> std::result::Result<Config, &'static str> {
				// Ignore path to exe.
				args.next();

				let in_directory = args.next();
				let out_directory = args.next();

				Ok(Config {in_directory, out_directory})
		}

		pub fn get_in_path(&self) -> Result<PathBuf> {
				Config::get_path_or_default(&self.in_directory, "src")
		}

		pub fn get_out_path(&self) -> Result<PathBuf> {
				Config::get_path_or_default(&self.out_directory, "out")
		}

		pub fn get_relative_out_path(&self, path: &Path) -> Result<PathBuf> {
				let in_path = self.get_in_path()?;
				let out_path = self.get_out_path()?;
				let relative_path = path.strip_prefix(in_path)?;
				Ok(out_path.join(relative_path))
		}

		fn get_path_or_default(path: &Option<String>, postfix: &str) -> Result<PathBuf> {
				match path {
						Some(s) => Ok(PathBuf::from(&s)),
						None => {
								let mut path = env::current_dir().unwrap();
								path.push(postfix);
								Ok(path)
						}
				}
		}

}
