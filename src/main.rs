use std::env;
use std::process;

use static_site_generator::config::Config;

fn main() {
		let config = Config::new(env::args()).unwrap_or_else(|e| {
				eprintln!("Problem parsing arguments: {}", e);
				process::exit(1);
		});

		if let Err(e) = static_site_generator::run(config) {
				eprintln!("Application error: {}", e);

				process::exit(1);
		}
}
