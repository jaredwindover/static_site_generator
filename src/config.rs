use std::env;
use std::path::{PathBuf, Path};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct IndexConfig {
    pub page_title: Option<String>,
    pub title: Option<String>,
    pub exclude: Vec<String>,
}

impl IndexConfig {
    pub fn default() -> IndexConfig {
        IndexConfig {
            page_title: Some(String::from("dir")),
            title: Some(String::from("dir")),
            exclude: vec![String::from("")],
		}
    }
}

#[derive(Debug, Deserialize)]
pub struct DirectoryConfig {
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
}

impl Config {
    pub fn new(mut args: env::Args) -> std::result::Result<Config, &'static str> {
        // Ignore path to exe.
        args.next();

        let in_directory = args.next();
        let out_directory = args.next();

        Ok(Config { in_directory, out_directory })
    }

    pub fn get_in_path(&self) -> PathBuf {
        Config::get_path_or_default(&self.in_directory, "src")
    }

    pub fn get_out_path(&self) -> PathBuf {
        Config::get_path_or_default(&self.out_directory, "out")
    }

    pub fn get_relative_out_path(&self, path: &Path) -> PathBuf {
        let in_path = self.get_in_path();
        let out_path = self.get_out_path();
        let relative_path = path.strip_prefix(in_path).unwrap();
        out_path.join(relative_path)
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
