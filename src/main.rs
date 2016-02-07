extern crate filetime;
extern crate rustc_serialize;
extern crate crypto;

use std::path::Path;
use std::collections::HashMap;
use std::fs::*;


mod metadata;
mod workingdirectory;
mod store;
mod config;

fn main() {
	let mut args = std::env::args();
	match args.nth(1) {
		Some(option) => dispatch_option(&option),
		None => println!("No option")
	}
}

fn dispatch_option(option: &str) {
	match option {
		"new-config" => new_config(),
		"new" => new_repo(),
		"update" => update(),
		"update-remote" => update_remote(),
		"commit" => commit(),
		"commit-remote" => commit_remote(),
		_ => println!("Unknown option {}", option)
	}
}

fn new_config() {
	println!("Creation of a new config file");
	let json_path = Path::new("config.json");
	config::create_default_config_file(json_path);
}

fn load_config() -> config::Config {
	config::read_config_file(Path::new("config.json"))
}

fn new_repo() {
	println!("Creation of a new repo");
	let json_path = load_config().get_local_metadata_path();
	metadata::create_emty_metadata_file(&json_path);
}

fn update() {
	let config = load_config();
	let data_path = config.get_data_path();
	let json_path = config.get_local_metadata_path();
	let store_path = config.get_store_path();

	let wd_hierarchy : HashMap<String, model::MetaData> = workingdirectory::read_working_directory(&data_path);
	println!("{} files in the working directory", wd_hierarchy.len());

	let mt_hierarchy = metadata::read_metadata_file(&json_path);
	println!("{} files in the metadata", mt_hierarchy.get_number_of_files());

	let file_top_update = files_to_update(wd_hierarchy, &mt_hierarchy);

	for (filename, metadata) in file_top_update.iter() {
		store::extract_file(&store_path, &metadata.get_hash(), &data_path, filename, metadata.get_timestamp());
	}
}

fn update_remote() {
	let config = load_config();
	let data_path = config.get_data_path();
	let json_path = config.get_local_metadata_path();
	let json_remote_path = config.get_remote_metadata_path();
	let store_path = config.get_store_path();

	let wd_hierarchy : HashMap<String, model::MetaData> = workingdirectory::read_working_directory(&data_path);
	println!("{} files in the working directory", wd_hierarchy.len());

	let mt_hierarchy = metadata::read_metadata_file(&json_path);
	println!("{} files in the metadata", mt_hierarchy.get_number_of_files());

	let mt_remote_hierarchy = metadata::read_metadata_file(&json_remote_path);
	println!("{} files in the remote metadata", mt_remote_hierarchy.get_number_of_files());

	let file_top_update = files_to_update_remote(wd_hierarchy, &mt_hierarchy, &mt_remote_hierarchy);
	match file_top_update {
		Some(x) => {
			println!("OK to update");
			for (filename, metadata) in x.iter() {
				store::extract_file(&store_path, &metadata.get_hash(), &data_path, filename, metadata.get_timestamp());
			}
			println!("Replace local metadata by remote metadata");
			remove_file(&json_path);
			copy(&json_remote_path, &json_path);
		},
		None => println!("IMPOSSIBLE to update")
	}
}

fn files_to_update(wd_hierarchy: HashMap<String, model::MetaData>, mt_hierarchy: &model::Hierarchy) -> HashMap<String, model::MetaData>  {
	let mut file_to_update : HashMap<String, model::MetaData> = HashMap::new();

	for (filename, metadataset) in mt_hierarchy.get_files().iter() {
		let wd_metadata = wd_hierarchy.get(filename);

		match wd_metadata {
			Some(x) => {
				let wd_timestamp = x.get_timestamp();
				let last_md_timestamp = metadataset.get_last().unwrap().get_timestamp();

				if wd_timestamp == last_md_timestamp {
					println!("- No need to update because timestamps are equal {}", filename);
				} else if metadataset.has_metadata_with_timestamp(wd_timestamp) {
					println!("- Existing file to update {}", filename);
					file_to_update.insert(filename.clone(), metadataset.get_last().unwrap().clone());
				} else { //timestamp unknown
					if wd_timestamp > last_md_timestamp {
						println!("- Working directory file is more recent {}", filename);
					} else {
						print!(" - No !! Working directory file is older. It makes no sense {}", filename);
					}
				}
			    ()},
			None => {
				println!("- New file to update {}", filename);
			    file_to_update.insert(filename.clone(), metadataset.get_last().unwrap().clone());
			    () }
		}
	}

	file_to_update	
}

fn files_to_update_remote(wd_hierarchy: HashMap<String, model::MetaData>, mt_local_hierarchy: &model::Hierarchy, mt_remote_hierarchy: &model::Hierarchy) -> Option<HashMap<String, model::MetaData>>  {
	let mut file_to_update : HashMap<String, model::MetaData> = HashMap::new();

	for (filename, metadataset) in mt_remote_hierarchy.get_files().iter() {
		let wd_metadata = wd_hierarchy.get(filename);

		match wd_metadata {
			Some(x) => {
				let wd_timestamp = x.get_timestamp();
				let last_md_timestamp = metadataset.get_last().unwrap().get_timestamp();

				if wd_timestamp == last_md_timestamp {
					println!("- No need to update because timestamps are equal {}", filename);
				} else if metadataset.has_metadata_with_timestamp(wd_timestamp) {
					println!("- Existing file to update {}", filename);
					file_to_update.insert(filename.clone(), metadataset.get_last().unwrap().clone());
				} else { //timestamp unknown
					
					match mt_local_hierarchy.get_latest_meta_data(&filename) {
						Some(m) => {
							if m.get_timestamp() == last_md_timestamp {
								println!("- No need to update because working directory file is a correct new version {}", filename);
							} else {
								println!("- CONFLICT ! The remove and the working directory file have changed {}", filename);
								return Option::None;
							}
						}
						None => {
							println!("- CONFLICT ! New file in the working directory but a remote version exists {}", filename);
							return Option::None;
						}
					};
				}
			    ()},
			None => {
				println!("- New file to update {}", filename);
			    file_to_update.insert(filename.clone(), metadataset.get_last().unwrap().clone());
			    () }
		}
	}

	Option::Some(file_to_update)
}

fn commit_remote() {
	commit();

	let config = load_config();
	let json_path = config.get_local_metadata_path();
	let json_remote_path = config.get_remote_metadata_path();

	remove_file(&json_remote_path);
	copy(&json_path, &json_remote_path);
}

fn commit() {
	let config = load_config();
	let data_path = config.get_data_path();
	let json_path = config.get_local_metadata_path();
	let store_path = config.get_store_path();

	let wd_hierarchy : HashMap<String, model::MetaData> = workingdirectory::read_working_directory(&data_path);
	println!("{} files in the working directory", wd_hierarchy.len());

	let mt_hierarchy = metadata::read_metadata_file(&json_path);
	println!("{} files in the metadata", mt_hierarchy.get_number_of_files());

	let files_to_commit = files_to_commit(wd_hierarchy, &mt_hierarchy);
	println!("{} files to commit", files_to_commit.len());

	let mut updated_metadata : HashMap<String, model::MetaData> = HashMap::new();
	for (filename, mut metadata) in files_to_commit {
		let hash = store::store_file(&store_path, Path::new(&filename));
		metadata.add_hash(hash);

		updated_metadata.insert(filename, metadata);
	}	
	let updated_metadata = updated_metadata;


	let mut mt_hierarchy = mt_hierarchy;
	mt_hierarchy.update(updated_metadata);

	metadata::write_metadata_file(&json_path, mt_hierarchy);
}

fn files_to_commit(wd_hierarchy: HashMap<String, model::MetaData>, mt_hierarchy: &model::Hierarchy) -> HashMap<String, model::MetaData>  {
	let mut files_to_commit: HashMap<String, model::MetaData> = HashMap::new();

	for (filename, metadata) in wd_hierarchy.iter() {
		let actual_metadata = mt_hierarchy.get_latest_meta_data(&filename);

		match actual_metadata {
			Some(x) => {
				if x.is_more_recent(&metadata) {
					println!("- File to update {}", filename);
					files_to_commit.insert(filename.clone(), metadata.clone());	
				} else {
					println!("- No need to update {}", filename);
				}
			    ()},
			None => {
				println!("- New file {}", filename);
			    files_to_commit.insert(filename.clone(), metadata.clone());
			    () }
		}
	}

	files_to_commit
}

mod model {

	use std::collections::HashMap;

	#[derive(Debug, RustcEncodable, RustcDecodable)]
	pub struct Hierarchy {
		nb_revision: i32,
	    files: HashMap<String, MetaDataSet>
	}

	#[derive(Debug, RustcEncodable, RustcDecodable)]
	pub struct MetaDataSet {
	    metadata: Vec<MetaData>
	}

	#[derive(Debug, RustcEncodable, RustcDecodable, Clone)]
	pub struct MetaData {
	   timestamp: u64,
	   size: u64,
	   hash: String,
	   stored_hash: String
	}

	impl Hierarchy {
		pub fn new_empty() -> Hierarchy {
			let empty_hierarchy_map : HashMap<String, MetaDataSet> = HashMap::new();
			Hierarchy {nb_revision: 1, files: empty_hierarchy_map}
		}
		pub fn get_number_of_files(&self) -> usize {
			self.files.len()
		}	
		pub fn get_latest_meta_data(&self, filename: &String) -> Option<&MetaData> {
			match self.files.get(filename) {
				Some(x) => x.get_last(),
				None => None
			}
		}
		pub fn update(&mut self, new_metadata_map: HashMap<String, MetaData>) {
			self.nb_revision = self.nb_revision + 1;

			for (filename, metadata) in new_metadata_map {
				
				let new_metadata = self.new_metadata(&filename);

				if new_metadata {
					self.files.insert(filename, MetaDataSet::new_simple_meta_data_set(metadata));
				} else {
					let mut m = self.files.get_mut(&filename).unwrap();
					m.add_revision(metadata);
				}
			}
		}

		fn new_metadata(&self, filename: &String) -> bool {
			let actual_metadata = self.files.get(filename);

			match actual_metadata {
				Some(_) => false,
				None => true
			}
		}

		pub fn get_files(&self) -> &HashMap<String, MetaDataSet> {
			&self.files
		}
	}

	impl MetaDataSet {
		pub fn get_last(&self) -> Option<&MetaData> {
			self.metadata.last()
		}
		pub fn new_simple_meta_data_set(m: MetaData) -> MetaDataSet {
			let mut v: Vec<MetaData> = Vec::new();
			v.push(m);
			MetaDataSet {metadata: v }
		}
		pub fn add_revision(&mut self, m: MetaData) {
			self.metadata.push(m);
		}
		pub fn has_metadata_with_timestamp(&self, timestamp: u64) -> bool {
			let mut it = self.metadata.iter();
			match it.find(|&m| m.get_timestamp() == timestamp) {
				Some(_) => true,
				None => false
			}
		}
	}

	impl MetaData {
		pub fn new_without_hash(timestamp: u64, size: u64) -> MetaData {
			MetaData {timestamp: timestamp, size: size, hash: "".to_string(), stored_hash: "".to_string()}
		}
		pub fn add_hash(&mut self, hash: String) {
			self.hash = hash;
		}
		pub fn is_more_recent(&self, other: &MetaData) -> bool {
			self.timestamp < other.timestamp
		}
		pub fn get_timestamp(&self) -> u64 {
			self.timestamp
		}
		pub fn get_hash(&self) -> String {
			self.hash.clone()
		}	
	}

}