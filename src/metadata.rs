
use std::collections::HashMap;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;

use filetime::FileTime;
use rustc_serialize::json;

use model::Hierarchy;
use model::MetaDataSet;

pub fn create_emty_metadata_file<P: AsRef<Path>>(path: P) {
	println!("Creating empty metadata file in {}", path.as_ref().to_str().unwrap());  

	let hierarchy = Hierarchy::new_empty();

//	println!("hey : {:?}", hierarchy);

	let json_hierarchy =   json::encode(&hierarchy).unwrap();

//	println!("hey in json: {:?}", json_hierarchy);    

	let mut file = File::create(&path).unwrap();

    let u8_vec = json_hierarchy.into_bytes();
	let u8_slice = &u8_vec[..];
    file.write_all(u8_slice);

    file.sync_all();
}

pub fn read_metadata_file<P: AsRef<Path>>(path: P) -> Hierarchy {
//	println!("Reading metadata file from {}", path.as_ref().to_str().unwrap());  
 
	let mut file = File::open(&path).unwrap();
	let mut json = String::new();
    file.read_to_string(&mut json);

    let hierarchy: Hierarchy = json::decode(&json).unwrap();
    hierarchy
}

pub fn write_metadata_file<P: AsRef<Path>>(path: P, hierarchy: Hierarchy) {
	let jsonHierarchy = json::encode(&hierarchy).unwrap();
    let u8_vec = jsonHierarchy.into_bytes();
	let u8_slice = &u8_vec[..];

	let mut file = File::create(&path).unwrap();
    file.write_all(u8_slice);
    file.sync_all();
}

