use crate::tools::db::ObjectRepr;
use docopt::Docopt;
use serde::Deserialize;

pub const USAGE: &str = "
Usage:   unrusty cat-file (-t [--allow-unknown-type]| -s [--allow-unknown-type]| -e | -p) \
                         [--path=<path>] <object>
         unrusty cat-file --help

Options:
         <object>                     The object to be considered
         -t, --type                   Show type of object
         -s, --size                   Show size of object
         -e, --error                  Returns 1 if invalid format and print error to stderr
         -p, --print                  (Pretty) prints the contents of the file
         --allow-unknown-type         Allow -s or -t to query broken/corrupt objects of unknown \
                         type
         -h, --help                   Shows this help message
";

#[derive(Deserialize, Debug)]
pub struct Args {
	pub flag_type:               bool,
	pub flag_size:               bool,
	pub flag_error:              bool,
	pub flag_print:              bool,
	pub arg_object:              ObjectRepr,
	pub flag_allow_unknown_type: bool,
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
