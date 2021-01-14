use crate::tools::db::{read, read_with_invalid, Object, ObjectRepr, ObjectType};
use log::error;
use std::{error::Error, process::exit};

pub fn cat_file_type(
	object_repr: &ObjectRepr,
	allow_invalid_type: bool,
) -> Result<ObjectType, Box<dyn Error>> {
	let obj = read_with_invalid(&object_repr, allow_invalid_type)?;
	cat_file_type_prepared(&obj)
}

pub fn cat_file_type_prepared(obj: &Object) -> Result<ObjectType, Box<dyn Error>> {
	Ok(obj.object_type)
}

pub fn cat_file_size(
	object_repr: &ObjectRepr,
	allow_invalid_type: bool,
) -> Result<usize, Box<dyn Error>> {
	let obj = read_with_invalid(&object_repr, allow_invalid_type)?;
	cat_file_size_prepared(&obj)
}

pub fn cat_file_size_prepared(obj: &Object) -> Result<usize, Box<dyn Error>> {
	Ok(obj.data.len())
}

pub fn cat_file_print(object_repr: &ObjectRepr) -> Result<String, Box<dyn Error>> {
	let obj = read_with_invalid(&object_repr, false)?;
	cat_file_print_prepared(&obj)
}

pub fn cat_file_print_prepared(obj: &Object) -> Result<String, Box<dyn Error>> {
	Ok(std::string::String::from_utf8_lossy(&obj.data).to_string())
}

pub fn cat_file_check(object_repr: &ObjectRepr) -> ! {
	exit(match read(&object_repr) {
		Ok(_) => 0,
		Err(e) => {
			error!("entry has invalid format: {:?}", e);
			1
		},
	});
}
