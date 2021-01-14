mod add;
mod cat_file;
mod hash_object;
mod init;
mod tools;

// Command imports
use crate::{
	add::add_cmd::cmd_add, cat_file::cat_file_cmd::cmd_cat_file,
	hash_object::hash_object_cmd::cmd_hash_object, init::init_cmd::cmd_init,
};

// Library function public imports
pub use crate::cat_file::cat_file_do::{
	cat_file_check, cat_file_print, cat_file_print_prepared, cat_file_size, cat_file_size_prepared,
	cat_file_type, cat_file_type_prepared,
};

pub use crate::tools::db::{
	delete, insert, insert_file, insert_file_with_simulate, insert_with_simulate, read,
	read_with_invalid, simulate_insert, simulate_insert_file,
};

pub use crate::tools::path::{db_path, index_path_required, root_path, root_path_required};

pub use crate::init::init_do::init;

// Regular imports
use log::{error, LevelFilter};
use std::{env, iter::Iterator, path::Path};

pub const USAGE: &str = "
Commands:   unrusty hash-object
            unrusty cat-file
            unrusty add
            unrusty init
            unrusty help
";

fn init_logger() {
	env_logger::builder()
		.format_timestamp(None)
		.filter(None, LevelFilter::Info)
		.init();
}

fn retrieve_argv() -> Vec<String> {
	let mut args = env::args().collect::<Vec<_>>();

	let executable_name = args
		.first()
		.and_then(|s| Path::new(s).file_name())
		.and_then(|p| p.to_str())
		.map(|s| s.to_owned());

	if let Some(s) = executable_name {
		args[0] = s;
	} else {
		error!("Could not retrieve executable name");
	}
	args
}

fn main() {
	init_logger();
	let argv = retrieve_argv();

	if let Some(command) = argv.get(1) {
		match command.as_str() {
			"hash-object" => {
				cmd_hash_object(&argv);
			},
			"cat-file" => {
				cmd_cat_file(&argv);
			},
			"add" => {
				cmd_add(&argv);
			},
			"init" => {
				cmd_init(&argv);
			},
			"help" => {
				print!("{}", USAGE);
			},
			_ => {
				error!("Unknown subcommand, see unrusty help");
			},
		};
	} else {
		error!("No specified subcommand, see unrusty help");
	}
}
