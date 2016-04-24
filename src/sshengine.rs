
use std::net::TcpStream;
use std::path::Path;
use std::path::PathBuf;
use ssh2::Session;
use std::io;
use std::io::Write;
use std::io::Read;
use std::fs::File;
use std::io::BufReader;
use std::io::BufWriter;


pub fn upload_to_remote(adress: &str, user: &str, password: &str, remote_root_path: &Path, files_path: &Vec<PathBuf>) {
	let tcp = TcpStream::connect(adress).unwrap();
	let mut sess = Session::new().unwrap();
	sess.handshake(&tcp).unwrap();
	sess.userauth_password(user, password).unwrap();

	for file_path in files_path {
		let filename = file_path.file_name().unwrap();

		let mut file = File::open(&file_path).unwrap();
		let file_size = file.metadata().unwrap().len();
		let mut file_reader = BufReader::new(file);

		let mut remote_path = remote_root_path.join(filename);

		println!("Remote path {:?}", remote_path);

		let mut remote_channel = sess.scp_send(&remote_path, 0o644, file_size, None).unwrap();
		let mut remote_file_writer = BufWriter::new(remote_channel);

		io::copy(&mut file_reader, &mut remote_file_writer);
	}
}

pub fn download_file_from_remote(adress: &str, user: &str, password: &str, remote_path: &Path, local_path: &Path) {
	//remote duplicate code
	let tcp = TcpStream::connect(adress).unwrap();
	let mut sess = Session::new().unwrap();
	sess.handshake(&tcp).unwrap();
	sess.userauth_password(user, password).unwrap();

	let sftp = sess.sftp().expect("Cannot create sftp");


	let (mut remote_file, stat) = sess.scp_recv(&remote_path).expect("Cannot open remote file");

	let mut remote_file_reader = BufReader::new(remote_file);


	println!("local path is {:?}", local_path);

	let mut local_file = File::create(&local_path).expect("Cannot open local file");
	let mut local_file_writer = BufWriter::new(local_file);

	io::copy(&mut remote_file_reader, &mut local_file_writer);
}


pub fn download_folder_from_remote(adress: &str, user: &str, password: &str, remote_root_path: &Path, local_root_path: &Path) {
	//remote duplicate code
	let tcp = TcpStream::connect(adress).unwrap();
	let mut sess = Session::new().unwrap();
	sess.handshake(&tcp).unwrap();
	sess.userauth_password(user, password).unwrap();

	let sftp = sess.sftp().expect("Cannot create sftp");
	let elements = sftp.readdir(remote_root_path).expect("Cannot read remote directory");

	println!("Read : {}", elements.len());

	for (path, file_stat) in elements {
		let filename = path.file_name().unwrap();

		println!("filename {:?}", filename);


		let (mut remote_file, stat) = sess.scp_recv(&path).expect("Cannot open remote file");

		let mut remote_file_reader = BufReader::new(remote_file);

		let mut local_path = local_root_path.join(filename);

		println!("local path is {:?}", local_path);

		let mut local_file = File::create(&local_path).expect("Cannot open local file");
		let mut local_file_writer = BufWriter::new(local_file);

		io::copy(&mut remote_file_reader, &mut local_file_writer);
	}
}
	

