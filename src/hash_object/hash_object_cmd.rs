use super::hash_object_parse::Args;
use crate::tools::{
	db::{insert_file_with_simulate, insert_with_simulate, DbError, ObjectRepr},
	failed::failed,
};
use std::{
	io::{self, BufRead, Read},
	path::PathBuf,
};

fn extract_file_paths(args: &Args) -> Vec<PathBuf> {
	if args.flag_stdin_paths {
		io::stdin()
			.lock()
			.lines()
			.map(|p| {
				PathBuf::from(
					p.unwrap_or_else(|e| {
						failed(&format!("Could not read line because of {:?}", e))
					})
					.trim_end(),
				)
			})
			.collect()
	} else {
		args.arg_file.clone()
	}
}

fn handle_hash_object_result(result: Result<ObjectRepr, DbError>) {
	match result {
		Ok(object_repr) => println!("{}", object_repr),
		Err(err) => failed(&format!("failed hash-object {:?}", err)),
	}
}

pub fn cmd_hash_object<'a, I, J>(argv_it: I)
where
	I: IntoIterator<Item = &'a J>,
	J: AsRef<str> + 'a,
{
	let args = Args::from_cmd(argv_it);
	let object_type = args.flag_type;
	let write = args.flag_write;

	if !args.flag_stdin_paths && args.flag_stdin {
		let mut content = Vec::new();
		io::stdin()
			.read_to_end(&mut content)
			.unwrap_or_else(|e| failed(&format!("Failed to read content from stdin {:?}", e)));
		let result = insert_with_simulate(&content, object_type, !write);
		handle_hash_object_result(result);
	}

	let file_paths = extract_file_paths(&args);
	for path in &file_paths {
		let result = insert_file_with_simulate(&path, object_type, !write);
		handle_hash_object_result(result);
	}
}
