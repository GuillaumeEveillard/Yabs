use std::collections::HashMap;
use filetime::FileTime;
use std::path;
use std::path::Path;
use std::fs;

use model::MetaData;

pub fn read_working_directory(root: &Path) -> HashMap<String, MetaData> {
	let mut hierarchy : HashMap<String, MetaData> = HashMap::new();
	read_folder_rec(root, &mut hierarchy);
	hierarchy
}

fn read_folder_rec(root: &Path, hierarchy: &mut HashMap<String, MetaData>) {
	let paths = fs::read_dir(root).unwrap();

	for entry in paths {
		let entry = entry.unwrap();
		let path_buf: path::PathBuf = entry.path();
		let path = path_buf.as_path();

		let info = entry.metadata().unwrap();

		if info.is_dir() {
			read_folder_rec(path, hierarchy);
		} else if info.is_file() {

			let path_str = path.to_str().expect("cannot convert path to utf8 string").to_string();

			let last_change = FileTime::from_last_modification_time(&info).seconds_relative_to_1970();

			let metadata = MetaData::new_without_hash(last_change, info.len());

			hierarchy.insert(path_str, metadata);
		} else {

		}
    }    
}