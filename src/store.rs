use std::path::Path;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::io::BufWriter;
use filetime::FileTime;
use filetime;

use std::io;
use std::io::Result;
use std::io::Read;
use std::io::Write;

use crypto::digest::Digest;
use crypto::sha2::Sha256;

pub fn store_file(store_path: &Path, source_file: &Path) -> String {
	let tmp_path = store_path.join("tmp");

	let mut file_reader = BufReader::new(File::open(&source_file).unwrap());
	let mut file_writer = BufWriter::new(File::create(&tmp_path).unwrap());

	let mut hash_file_write = HashWriter::new(file_writer);

	io::copy(&mut file_reader, &mut hash_file_write);

	let hash = hash_file_write.get_hash();

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

struct HashWriter<W: Write> {
	hasher: Sha256,
	writer: W
}

impl <W: Write> HashWriter<W>  {
	fn new(inner: W) -> HashWriter<W> {
		HashWriter {hasher: Sha256::new(), writer: inner}
	}
	fn get_hash(&mut self) -> String {
		self.hasher.result_str()
	}
}

impl <W: Write> Write for HashWriter<W>  {
	fn write(&mut self, buf: &[u8]) -> Result<usize> {
		self.hasher.input(&buf);
		self.writer.write(buf)
	}

	fn flush(&mut self) -> Result<()> {
		self.writer.flush()
	}
}
