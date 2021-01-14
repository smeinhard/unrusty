use crate::tools::path::{db_path, PathError};
use flate2::{
	write::{ZlibDecoder, ZlibEncoder},
	Compression,
};
use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize, Serializer};
use sha1::{Digest, Sha1};
use std::{
	convert::{From, TryFrom},
	fmt::{self, Display, Formatter},
	fs::{self, remove_file, File},
	io::{self, ErrorKind, Write},
	path::{Path, PathBuf},
	str::from_utf8,
};
use thiserror::Error;
use DbError::{DeleteError, InputReadError, NoRootError, ReadError, WriteError};

////////////////////////////////////////////
// TYPES
////////////////////////////////////////////

#[derive(Error, Debug)]
pub enum DbError {
	#[error("{source:?}")]
	ReadError { source: io::Error },
	#[error("{source:?}")]
	WriteError { source: io::Error },
	#[error("{source:?}")]
	InputReadError { source: io::Error },
	#[error("{source:?}")]
	DeleteError { source: io::Error },
	#[error("No unrusty root")]
	NoRootError,
}

#[derive(Deserialize, Debug, Clone, Copy)]
pub enum ObjectType {
	Blob,
	Tree,
	Commit,
	Invalid,
}

pub struct Object {
	pub data:        Vec<u8>,
	pub object_type: ObjectType,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct ObjectRepr {
	hash: String,
}

////////////////////////////////////////////
// READ
////////////////////////////////////////////

pub fn read(object_repr: &ObjectRepr) -> Result<Object, DbError> {
	read_with_invalid(object_repr, false)
}

pub fn read_with_invalid(
	object_repr: &ObjectRepr,
	allow_invalid_type: bool,
) -> Result<Object, DbError> {
	let path = object_repr.path().map_err(|_| NoRootError)?;
	let content = fs::read(path).map_err(|e| ReadError { source: e })?;
	let mut decoded = Vec::new();
	let mut z = ZlibDecoder::new(decoded);
	z.write_all(&content[..])
		.map_err(|e| ReadError { source: e })?;
	decoded = z.finish().map_err(|e| ReadError { source: e })?;

	if let Some(index) = decoded.iter().position(|&x| x == b'\0') {
		let header = &decoded[0..index];
		let data = &decoded[(index + 1)..];
		read_prepared(header, data, allow_invalid_type)
	} else {
		Err(read_error_factory("No null byte in object"))
	}
}

fn read_prepared(header: &[u8], data: &[u8], allow_invalid: bool) -> Result<Object, DbError> {
	let header = from_utf8(&header).map_err(|e| read_error_factory(&format!("{:?}", e)))?;

	lazy_static! {
		static ref RE: Regex = Regex::new(r"\A([a-z]*) ([1-9]\d*)\z").unwrap();
	}
	let capture = RE
		.captures(header)
		.ok_or_else(|| read_error_factory("Header not formatted correctly"))?;
	assert!(capture.len() == 3);

	let length: usize = capture[2].parse().unwrap();
	if data.len() != length {
		return Err(read_error_factory(
			"Size of data not equal to description in header",
		));
	}

	let object_type = ObjectType::from(&capture[1]);
	if !allow_invalid && !object_type.is_valid() {
		return Err(read_error_factory("Object has invalid type"));
	}

	Ok(Object {
		data: data.to_vec(),
		object_type,
	})
}

fn read_error_factory(msg: &str) -> DbError {
	ReadError {
		source: io::Error::new(ErrorKind::Other, msg),
	}
}

////////////////////////////////////////////
// WRITE
////////////////////////////////////////////

pub fn simulate_insert_file(path: &Path, object_type: ObjectType) -> Result<ObjectRepr, DbError> {
	insert_file_with_simulate(&path, object_type, true)
}

pub fn insert_file(path: &Path, object_type: ObjectType) -> Result<ObjectRepr, DbError> {
	insert_file_with_simulate(&path, object_type, false)
}

pub fn simulate_insert(content: &[u8], object_type: ObjectType) -> Result<ObjectRepr, DbError> {
	insert_with_simulate(content, object_type, true)
}

pub fn insert(content: &[u8], object_type: ObjectType) -> Result<ObjectRepr, DbError> {
	insert_with_simulate(content, object_type, false)
}

pub fn insert_file_with_simulate(
	path: &Path,
	object_type: ObjectType,
	simulate: bool,
) -> Result<ObjectRepr, DbError> {
	let content = fs::read(&path).map_err(|e| InputReadError { source: e })?;
	insert_with_simulate(&content, object_type, simulate)
}

pub fn insert_with_simulate(
	content: &[u8],
	object_type: ObjectType,
	simulate: bool,
) -> Result<ObjectRepr, DbError> {
	let store = get_store(content, object_type);
	let object_repr = get_object_repr(&store);
	if !simulate {
		write_object_to_file(&store, &object_repr)?;
	}
	Ok(object_repr)
}

fn write_object_to_file(store: &[u8], object_repr: &ObjectRepr) -> Result<(), DbError> {
	let path = object_repr.path().map_err(|_| NoRootError)?;
	fs::create_dir_all(path.parent().unwrap()).map_err(|e| WriteError { source: e })?;
	let mut f = File::create(&path).map_err(|e| WriteError { source: e })?;
	write_object(store, &mut f).map_err(|e| WriteError { source: e })
}

fn get_store(content: &[u8], object_type: ObjectType) -> Vec<u8> {
	let header = format!("{} {}\0", object_type, content.len());
	let mut header = Vec::from(header.as_bytes());
	header.extend_from_slice(content);
	header
}

fn get_object_repr(store: &[u8]) -> ObjectRepr {
	let mut hasher = Sha1::new();
	hasher.input(&store);
	ObjectRepr::try_from(hex::encode(hasher.result()).as_str()).unwrap()
}

fn write_object(store: &[u8], writer: &mut dyn Write) -> Result<(), io::Error> {
	let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
	encoder.write_all(store)?;
	let encoded_content = encoder.finish()?;
	writer.write_all(&encoded_content)?;
	Ok(())
}

////////////////////////////////////////////
// DELETE
////////////////////////////////////////////

pub fn delete(object_repr: &ObjectRepr) -> Result<(), DbError> {
	let path = object_repr.path().map_err(|_| NoRootError)?;
	remove_file(&path).map_err(|e| DeleteError { source: e })
}

////////////////////////////////////////////
// TYPE IMPLEMENTATIONS
////////////////////////////////////////////

impl Display for ObjectType {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		match self {
			ObjectType::Blob => write!(f, "blob"),
			ObjectType::Tree => write!(f, "tree"),
			ObjectType::Commit => write!(f, "commit"),
			ObjectType::Invalid => write!(f, "invalid"),
		}
	}
}

impl From<&str> for ObjectType {
	fn from(repr: &str) -> Self {
		match repr {
			"blob" => ObjectType::Blob,
			"tree" => ObjectType::Tree,
			"commit" => ObjectType::Commit,
			_ => ObjectType::Invalid,
		}
	}
}

impl ObjectType {
	pub fn is_valid(self) -> bool {
		!matches!(self, ObjectType::Invalid)
	}

	pub fn iterator() -> impl Iterator<Item = ObjectType> {
		[
			ObjectType::Blob,
			ObjectType::Commit,
			ObjectType::Tree,
			ObjectType::Invalid,
		]
		.iter()
		.copied()
	}
}

impl Object {
	pub fn is_valid(&self) -> bool {
		self.object_type.is_valid()
	}
}

impl TryFrom<&str> for ObjectRepr {
	type Error = ();

	fn try_from(hash: &str) -> Result<Self, Self::Error> {
		lazy_static! {
			static ref RE: Regex = Regex::new(r"\A[\d|[a-f]]{40}\z").unwrap();
		}
		if RE.is_match(&hash) {
			Ok(ObjectRepr {
				hash: String::from(hash),
			})
		} else {
			Err(())
		}
	}
}

impl Display for ObjectRepr {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "{}", self.hash)
	}
}

impl ObjectRepr {
	pub fn path(&self) -> Result<PathBuf, PathError> {
		db_path(self)
	}

	pub fn hash(&self) -> &str {
		self.hash.as_str()
	}
}

// needed as we want to check that the hash is correct by calling
// ObjectRepr::try_from. Also, we want to ensure that docopt can directly parse
// a hash and use it as an ObjectRepr
struct ObjectReprVisitor;

impl<'de> serde::de::Visitor<'de> for ObjectReprVisitor {
	type Value = ObjectRepr;

	fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
		formatter.write_str("a valid object identifier")
	}

	fn visit_str<E>(self, identifier: &str) -> Result<Self::Value, E>
	where
		E: serde::de::Error,
	{
		ObjectRepr::try_from(identifier).map_err(|_| E::custom("invalid identifier"))
	}
}

impl<'de> serde::de::Deserialize<'de> for ObjectRepr {
	fn deserialize<D>(d: D) -> Result<ObjectRepr, D::Error>
	where
		D: serde::de::Deserializer<'de>,
	{
		d.deserialize_string(ObjectReprVisitor)
	}
}

impl Serialize for ObjectRepr {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		serializer.serialize_str(self.hash())
	}
}
