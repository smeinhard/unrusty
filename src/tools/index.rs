use crate::tools::{
	db::{insert_file, DbError, ObjectRepr, ObjectType},
	path::{index_path_required, RelativePathToBase},
};
use log::info;
use serde::{Deserialize, Serialize};
use serde_yaml::{from_reader, to_writer};
use std::{
	collections::HashMap, convert::TryFrom, error::Error, fs, fs::File, io, path::Path,
	time::SystemTime,
};

use thiserror::Error;
use IndexError::{MetadataError, ReadError, WriteError};

// TODO: use closure for error generation

#[derive(Error, Debug)]
pub enum IndexError {
	#[error("Failed reading: {source:?}")]
	ReadError { source: Box<dyn Error> },
	#[error("Failed writing: {source:?}")]
	WriteError { source: Box<dyn Error> },
	#[error("Can't fetch file metadata: {source:?}")]
	MetadataError { source: io::Error },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Index {
	entries: HashMap<RelativePathToBase, IndexEntry>,
}

// CommonAncestor, Head and MergeHead occur during a merge conflict.
// They allow to quickly see the different versions of a file:
// from the common ancestor, the HEAD, and the MERGE_HEAD
#[derive(Serialize, Deserialize, Debug)]
pub enum MergeStatus {
	// check acccess rights
	Regular,
	CommonAncestor,
	Head,
	MergeHead,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IndexEntry {
	metadata: Metadata,
	status:   MergeStatus,
	// flags: u32, // change to struct of bools
	// index: u32, // Find out for what this is used!
	hash:     ObjectRepr,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Metadata {
	c_time: SystemTime,
	m_time: SystemTime,
	// device: u32,
	// inode: u32,
	// uid: u32,
	// gid: u32,
	size:   u64,
}

impl Metadata {
	fn new(path: &Path) -> Result<Metadata, IndexError> {
		let metadata = fs::symlink_metadata(&path).map_err(|e| MetadataError { source: e })?;

		Ok(Metadata {
			// Assume we are on a platform that has this metadata
			c_time: metadata.created().unwrap(),
			m_time: metadata.modified().unwrap(),
			// device:
			// inode:
			// uid:
			// gid:
			size:   metadata.len(),
		})
	}
}

impl IndexEntry {
	fn new(status: MergeStatus, hash: ObjectRepr, metadata: Metadata) -> IndexEntry {
		// Deal with ce_mode and ce_flag creation in add_cacheinfo
		IndexEntry {
			metadata,
			status,
			// flags: 0,
			// index: 0,
			hash,
		}
	}

	fn changed(&self, new_metadata: &Metadata) -> bool {
		self.metadata != *new_metadata
	}
}

// Creating index
impl Index {
	pub fn create_at_path(path: &Path) -> Result<(), IndexError> {
		let index = Index::new();
		Self::write_at_path_helper(&index, &path)
	}

	pub fn create() -> Result<(), Box<dyn Error>> {
		let index_path = index_path_required()?;
		Ok(Self::create_at_path(&index_path)?)
	}

	pub fn new() -> Index {
		Index {
			entries: HashMap::new(),
		}
	}
}

// Reading index
impl Index {
	pub fn read_at_path(path: &Path) -> Result<Index, IndexError> {
		let f = File::open(path).map_err(|e| ReadError {
			source: Box::new(e),
		})?;
		let index = from_reader(f).map_err(|e| ReadError {
			source: Box::new(e),
		})?;
		Ok(index)
	}

	pub fn read() -> Result<Index, Box<dyn Error>> {
		let index_path = index_path_required()?;
		Ok(Self::read_at_path(&index_path)?)
	}
}

// Writing index
impl Index {
	fn write_at_path_helper(&self, path: &Path) -> Result<(), IndexError> {
		let f = File::create(path).map_err(|e| WriteError {
			source: Box::new(e),
		})?;
		to_writer(f, self).map_err(|e| WriteError {
			source: Box::new(e),
		})
	}

	pub fn write_at_path(&self, path: &Path) -> Result<(), Box<dyn Error>> {
		Ok(self.write_at_path_helper(path)?)
	}

	pub fn write(&self) -> Result<(), Box<dyn Error>> {
		let index_path = index_path_required()?;
		self.write_at_path(&index_path)
	}
}

// Modifying index
impl Index {
	fn add_change_helper(
		&mut self,
		path: &Path,
		metadata: Metadata,
		status: MergeStatus,
		key: RelativePathToBase,
	) -> Result<(), DbError> {
		let hash = insert_file(path, ObjectType::Blob)?;
		info!("hash for {:?} in index is now {:?}", path, hash.hash());
		let entry = IndexEntry::new(status, hash, metadata);
		self.entries.insert(key, entry);
		Ok(())
	}

	pub fn add_change(&mut self, status: MergeStatus, path: &Path) -> Result<(), Box<dyn Error>> {
		let key = RelativePathToBase::try_from(path)?;
		let metadata = Metadata::new(path)?;

		match self.entries.get(&key) {
			Some(entry) => {
				if entry.changed(&metadata) {
					info!("updating {:?} in index", path);
					self.add_change_helper(path, metadata, status, key)?;
				} else {
					info!("file {:?} in index is up to date", path);
				}
			},
			None => {
				info!("adding {:?} to index", path);
				self.add_change_helper(path, metadata, status, key)?;
			},
		}
		Ok(())
	}
}
