use crate::tools::{
	index::Index,
	path::{root_path, vcs_dir_from_base},
};
use log::{info, warn};
use std::{env, error::Error, fs};

pub fn init(force_rewrite: bool) -> Result<(), Box<dyn Error>> {
	let root = root_path()?;

	let base = match root.as_ref() {
		Some(path) => path.clone(),
		None => env::current_dir()?,
	};
	let vcs_dir = vcs_dir_from_base(&base);

	if root.is_some() && force_rewrite {
		info!("deleting {:?}", vcs_dir);
		fs::remove_dir_all(&vcs_dir)?;
	}

	if root.is_some() && !force_rewrite {
		warn!("repository already exists, use --force to reset");
	} else {
		fs::create_dir_all(&vcs_dir)?;
		Index::create()?;
		info!("created new repository at {:?}", base);
	}
	Ok(())
}
