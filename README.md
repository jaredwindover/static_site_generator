# Static Site Generator
This is a basic static site generator, written in rust.

I'm using [comrak](https://github.com/kivikakk/comrak) for markdown parsing, and [horrorshow](https://github.com/Stebalien/horrorshow-rs) for html templating/generation.

Usage: `cargo run /path/to/src path/to/output`

Currently, all files and directories will be copied from src to output, except for markdown files which are converted to html with a minimal wrapper.
