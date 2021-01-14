use log::error;
use std::process::exit;

pub fn failed(msg: &str) -> ! {
	error!("{}, aborting.", msg);
	exit(1);
}
