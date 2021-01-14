use crate::tools::{
	failed::failed,
	index::{Index, MergeStatus},
};
use log::{info, warn};
use std::path::Path;

pub fn add<'a, I, J>(paths: I)
where
	I: IntoIterator<Item = &'a J>,
	J: AsRef<Path> + 'a,
{
	let mut index = Index::read()
		.unwrap_or_else(|e| failed(&format!("failed to read index because of {:?}", e)));

	for path in paths {
		let path = path.as_ref();
		if let Err(e) = index.add_change(MergeStatus::Regular, path) {
			warn!("Adding changes to {:?} failed: {:?}", path, e);
		} else {
			info!("Adding changes to {:?} succeeded", path);
		}
	}

	index
		.write()
		.unwrap_or_else(|e| failed(&format!("failed to write index because of {:?}", e)));
}
