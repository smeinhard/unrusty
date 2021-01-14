use crate::tools::db::ObjectType;
use docopt::Docopt;
use serde::Deserialize;
use std::path::PathBuf;

pub const USAGE: &str = "
Usage:   unrusty hash-object [-t <type>] [-w] [--stdin] [--] <file>...
         unrusty hash-object [-t <type>] [-w] --stdin-paths [--no-filters]
         unrusty hash-object --help

Options:
         -t <type>, --type <type>     type of object [default: blob]
         -w, --write                  write object
         --stdin-paths                Reads file paths from stdin, one per line 
         --stdin                      Read content to hash from stdin
         -h, --help                   Shows this help message
";

#[derive(Deserialize, Debug)]

pub struct Args {
	pub arg_file:         Vec<PathBuf>,
	pub flag_write:       bool,
	pub flag_type:        ObjectType,
	pub flag_stdin:       bool,
	pub flag_stdin_paths: bool,
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
