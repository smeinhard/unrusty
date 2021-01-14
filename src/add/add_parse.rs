use docopt::Docopt;
use serde::Deserialize;
use std::path::PathBuf;

pub const USAGE: &str = "
Usage:   unrusty add <file>...
         unrusty add --help

Options:
         -h, --help                   Shows this help message
";

#[derive(Deserialize, Debug)]

pub struct Args {
	pub arg_file: Vec<PathBuf>,
}

impl Args {
	pub fn from_cmd<'a, I, J>(argv_it: I) -> Args
	where
		I: IntoIterator<Item = &'a J>,
		J: AsRef<str> + 'a,
	{
		Docopt::new(USAGE)
			.and_then(|d| d.argv(argv_it).deserialize())
			.unwrap_or_else(|e| e.exit())
	}
}
