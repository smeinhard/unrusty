use crate::tools::db::ObjectRepr;
use serde::{Deserialize, Serialize};
use std::{
	assert,
	cmp::Eq,
	convert::TryFrom,
	env::current_dir,
	hash::Hash,
	io,
	path::{Path, PathBuf},
};
use thiserror::Error;
use PathError::{IllegalPathError, NoRepoError, NotInRepoError};

#[derive(Error, Debug)]
pub enum PathError {
	#[error("Not a valid repository")]
	NoRepoError,
	#[error("{source:?}")]
	IllegalPathError { source: io::Error },
	#[error("Not in repo. Repo: {base:?}, file: {file:?}")]
	NotInRepoError { base: PathBuf, file: PathBuf },
}

const VCS_DIR: &str = ".unrusty";
const INDEX_FILE: &str = "index";
const OBJECTS_DIR: &str = "objects";

pub fn vcs_dir_from_base(path: &Path) -> PathBuf {
	path.join(Path::new(VCS_DIR))
}

fn is_repo_root_dir(path: &Path) -> bool {
	let vcs_dir = vcs_dir_from_base(path);
	vcs_dir.is_dir()
}

pub fn root_path() -> Result<Option<PathBuf>, PathError> {
	let path = current_dir()
		.and_then(|p| p.canonicalize())
		.map_err(|e| IllegalPathError { source: e })?;

	for parent in path.ancestors() {
		if is_repo_root_dir(&parent) {
			return Ok(Some(parent.to_path_buf()));
		}
	}
	Ok(None)
}

pub fn root_path_required() -> Result<PathBuf, PathError> {
	match root_path()? {
		Some(path) => Ok(path),
		None => Err(NoRepoError),
	}
}

pub fn index_path_required() -> Result<PathBuf, PathError> {
	let mut root = root_path_required()?;
	root.push(PathBuf::from(&format!("{}/{}", VCS_DIR, INDEX_FILE,)));
	Ok(root)
}

fn relative_path(path: &Path) -> Result<PathBuf, PathError> {
	let path = path
		.canonicalize()
		.map_err(|e| IllegalPathError { source: e })?;
	let base = root_path_required()?;
	match path.strip_prefix(&base) {
		Ok(p) => Ok(p.to_path_buf()),
		Err(_) => Err(NotInRepoError { base, file: path }),
	}
}

pub fn db_path(object: &ObjectRepr) -> Result<PathBuf, PathError> {
	let hash = object.hash();
	assert!(hash.len() == 40);
	let rel_path = PathBuf::from(format!("{}/{}/{}", VCS_DIR, OBJECTS_DIR, &hash));
	let mut base = root_path_required()?;
	base.push(rel_path);
	Ok(base)
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct RelativePathToBase {
	path: PathBuf,
}

impl TryFrom<&Path> for RelativePathToBase {
	type Error = PathError;

	fn try_from(path: &Path) -> Result<Self, Self::Error> {
		relative_path(&path).map(|p| RelativePathToBase { path: p })
	}
}
