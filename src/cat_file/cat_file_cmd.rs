use super::{
	cat_file_do::{cat_file_check, cat_file_print, cat_file_size, cat_file_type},
	cat_file_parse::Args,
};
use log::error;

// TODO, implement identifiers, see https://git-scm.com/docs/gitrevisions
pub fn cmd_cat_file<'a, I, J>(argv_it: I)
where
	I: IntoIterator<Item = &'a J>,
	J: AsRef<str> + 'a,
{
	let args = Args::from_cmd(argv_it);
	let object_repr = args.arg_object;

	if args.flag_type {
		let res = cat_file_type(&object_repr, args.flag_allow_unknown_type);
		match res {
			Ok(object_type) => println!("{}", object_type),
			Err(err) => error!("failed finding type, {:?}", err),
		}
	} else if args.flag_size {
		let res = cat_file_size(&object_repr, args.flag_allow_unknown_type);
		match res {
			Ok(size) => println!("{}", size),
			Err(err) => error!("failed finding size, {:?}", err),
		}
	} else if args.flag_print {
		let res = cat_file_print(&object_repr);
		match res {
			Ok(res) => println!("{}", res),
			Err(err) => error!("failed printing entry, {:?}", err),
		}
	} else if args.flag_error {
		cat_file_check(&object_repr);
	}
}
