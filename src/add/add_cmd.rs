use super::{add_do::add, add_parse::Args};

pub fn cmd_add<'a, I, J>(argv_it: I)
where
	I: IntoIterator<Item = &'a J>,
	J: AsRef<str> + 'a,
{
	let args = Args::from_cmd(argv_it);

	let paths = args.arg_file;
	add(&paths);
}
