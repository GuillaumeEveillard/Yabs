use std::path::Path;
use std::path::PathBuf;
use std::fs::File;
use std::io::prelude::*;
use rustc_serialize::json;

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct Config {
	local_metadata_path: String,
	remote_metadata_path: String,
	data_path: String,
	store_path: String,
	storage_mode: StorageMode,
	ssh_address: String,
	ssh_user: String,
	ssh_password: String,
	ssh_root_path: String
}

#[derive(Debug, RustcEncodable, RustcDecodable, Clone)]
pub enum StorageMode {
    COPY,
    GZIP,
    GZIPAES
}

impl Config {
	fn new_default() -> Config {
		Config {
			local_metadata_path: String::from("local-metadata.json"),
			remote_metadata_path: String::from("remote-metadata.json"),
			data_path: String::from("data"),
			store_path: String::from("store"),
			storage_mode: StorageMode::COPY,
			ssh_address: String::from("127.0.0.1:22"),
			ssh_user: String::from("root"),
			ssh_password: String::from("password"),
			ssh_root_path: String::from("/root/Yabs"),
		}
	}
	pub fn get_local_metadata_path(&self) -> PathBuf {
		PathBuf::from(&self.local_metadata_path)
	}
	pub fn get_remote_metadata_path(&self) -> PathBuf {
		PathBuf::from(&self.remote_metadata_path)
	}
	pub fn get_data_path(&self) -> PathBuf {
		PathBuf::from(&self.data_path)
	}
	pub fn get_store_path(&self) -> PathBuf {
		PathBuf::from(&self.store_path)
	}
	pub fn get_storage_mode(&self) -> StorageMode {
		self.storage_mode.clone()
	}
	pub fn get_ssh_address(&self) ->  String {
		self.ssh_address.clone()
	}
	pub fn get_ssh_user(&self) ->  String {
		self.ssh_user.clone()
	}
	pub fn get_ssh_password(&self) ->  String {
		self.ssh_password.clone()
	}
	pub fn get_ssh_root_path(&self) -> PathBuf {
		PathBuf::from(&self.ssh_root_path)
	}
}

pub fn create_default_config_file<P: AsRef<Path>>(path: P) {
	println!("Creating empty config file in {}", path.as_ref().to_str().unwrap());  

	let config = Config::new_default();

	let json_config = json::encode(&config).unwrap();

	let mut file = File::create(&path).unwrap();

    let u8_vec = json_config.into_bytes();
	let u8_slice = &u8_vec[..];
    file.write_all(u8_slice);

    file.sync_all();
}

pub fn read_config_file<P: AsRef<Path>>(path: P) -> Config {
	let mut file = File::open(&path).unwrap();
	let mut json = String::new();
    file.read_to_string(&mut json);

    let config: Config = json::decode(&json).unwrap();
    config
}