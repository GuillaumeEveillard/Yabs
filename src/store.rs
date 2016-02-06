use std::path::Path;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use filetime::FileTime;
use filetime;

use std::io::Read;

use crypto::digest::Digest;
use crypto::sha2::Sha256;

pub fn store_file(store_path: &Path, source_file: &Path) -> String {
	let tmp_path = store_path.join("tmp");

	fs::copy(&source_file, &tmp_path);

	let hash = hash(&source_file);

	let final_path = store_path.join(&hash);

	fs::rename(&tmp_path, &final_path);

	hash
}

pub fn extract_file(store_path: &Path, hash: &String, data_path: &Path, filename: &String, timestamp: u64) {
	let file_in_store = store_path.join(hash);
	let file_in_wd = Path::new(filename);
	let tmp_path = data_path.join("tmp");

	println!("Extract from {} to {} ", file_in_store.to_str().unwrap(), file_in_wd.to_str().unwrap());

	fs::copy(&file_in_store, &tmp_path);
	fs::rename(&tmp_path, &file_in_wd);	

	let seconds_since_1970 = FileTime::from_seconds_since_1970(timestamp, 0);
	filetime::set_file_times(&file_in_wd, seconds_since_1970, seconds_since_1970);
}

fn hash(file: &Path) -> String {
	let f = File::open(&file).unwrap();
	let mut reader = BufReader::new(f);
	let mut buffer = [0; 512];

	let mut hasher = Sha256::new();

	loop {
		let bytes_read = reader.read(&mut buffer).unwrap();
		if bytes_read == 512 {
			hasher.input(&buffer);
		} else {
			let v : Vec<u8> = buffer.iter().cloned().collect();
			let (x, _) = v.split_at(bytes_read);
			hasher.input(x);
		}
		if bytes_read == 0 { break; }
	}

	let hex = hasher.result_str();
	hex
}
