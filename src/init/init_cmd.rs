use super::{init_do::init, init_parse::Args};
use log::error;

pub fn cmd_init<'a, I, J>(argv_it: I)
where
	I: IntoIterator<Item = &'a J>,
	J: AsRef<str> + 'a,
{
	let args = Args::from_cmd(argv_it);
	let force_rewrite = args.flag_force;

	if let Err(e) = init(force_rewrite) {
		error!("Failed init: {:?}", e);
	}
}
