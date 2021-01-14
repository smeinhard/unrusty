use docopt::Docopt;
use serde::Deserialize;

pub const USAGE: &str = "
Usage:   unrusty init [--force]
         unrusty init --help

Options:
         --force                      If the current location is in a repo, reset the VCS instead \
                         of doing nothing
         -h, --help                   Shows this help message
";

#[derive(Deserialize, Debug)]
pub struct Args {
	pub flag_force: bool,
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
