
//filesystem storage
extern crate msgpackio;
extern crate rustc_serialize;


use std::io::{Result,Error, ErrorKind, Cursor};
use std::path::{Path, PathBuf};
use std::fs::{self, File};
use std::env;

use self::rustc_serialize::hex::ToHex;

use crypto::digest::{Digest};
use crypto::blake2b::{Blake2b};

use self::msgpackio::read::{MsgPackReader, Value};
use self::msgpackio::write::{MsgPackWriter};

use super::core::{ReadHandle, WriteHandle, KeyValueStorage};


bitflags! {
	flags Flags: u32 {
		const FLAG_NONE			= 0b00000000,
		const FLAG_READONLY 	= 0b00000001,
		const FLAG_DELETEABLE 	= 0b00000010,
		const FLAG_ENCRYPTED	= 0b00000100,
		const FLAG_COMPRESSED	= 0b00001000,
		const FLAG_CAS			= 0b00010000,
	}
}



struct KeyInfo
{
	key_buf: Vec<u8>, //complete key in memory
	key_hash: Vec<u8>, //key hash
	data_file_path: PathBuf, //data file path
}

impl KeyInfo
{
	fn new(fsstorage: &FilesystemStorage, key_handle: &mut ReadHandle) -> Result<KeyInfo>
	{
		const KEY_SIZE : usize = 32;

		let key_buf : Vec<u8> = try!(key_handle.to_vec());

		//create hash from key
		let mut blake2b : &mut Digest = &mut Blake2b::new(KEY_SIZE); //32 or 64?
		blake2b.input(&key_buf);
		let mut key_hash = vec![0u8; KEY_SIZE];
		blake2b.result(&mut key_hash);

		//get hex representation:
		let key_hash_str = blake2b.result_str();

		//build up path
		let mut path = PathBuf::from(&fsstorage.repo_path);
		path.push(&key_hash_str[0..2]);
		path.push(&key_hash_str[2..]);

		//assert path is subpath from repo_path?

		Ok(KeyInfo {
			key_buf: key_buf,
			key_hash: key_hash,
			data_file_path: path
		})
	}
}

//put in seperate folder

/*struct KeyInfo
{
	//key data
	//hash_hex_string
	//hash_bin_data

	//from key_reader : &mut Read, key_size: usize
}*/

pub struct FilesystemStorage
{
	//path of storage has to exist
	repo_path: PathBuf,
	//crypto key for encryption

	//compression activated bool? enum (GZIP/L4)
	//compression level?
	//encryption activated Option<AES_Handler>
}

impl FilesystemStorage
{
	//init (create config file etc)

	pub fn open<P: AsRef<Path>>(path_in: P) -> Result<FilesystemStorage>
	{
		let mut path = PathBuf::from(path_in.as_ref());
		if !path.exists() || !path.is_dir() {
			return Err(Error::new(ErrorKind::Other, "The base path for storage must exist"));
		}

		//make absolute when not yet is
		if !path.is_absolute() {
			path = env::current_dir().unwrap();
			path.push(path_in);
		}

		debug!("[fs-storage]: opened on path {}", path.display());
		assert!(path.is_absolute() && path.is_dir());

		//TODO read config
			//Compression
			//Encryption

		Ok(FilesystemStorage {
			repo_path : path,
		})
	}

	//pub fn set_encryption_key();
	//compression_config? (level, etc)
}


impl KeyValueStorage for FilesystemStorage
{

	//get a value from repository by key, data is written to writer
	fn get(&self, key_handle: &mut ReadHandle, output_handle: &mut WriteHandle) -> Result<()>
	{
		let mut key_handle = key_handle;
		let mut output_handle = output_handle;
		let key_info = try!(KeyInfo::new(&self, key_handle));

		debug!("[fs-storage-get]: key-hash: {}", key_info.key_hash.to_hex());
		debug!("[fs-storage-get]: path: {}", key_info.data_file_path.display());

		//check if file exists
		if !key_info.data_file_path.exists() {
			return Err(Error::new(ErrorKind::Other, "key does not exist"));
		}

		//open file
		let mut file = try!(File::open(key_info.data_file_path));

		//version
		let (part, _) = try!(file.read_msgpack_value());
		if let Value::UInt8(x) = part {
			if x != 1 {
				return Err(Error::new(ErrorKind::Other, "storage file: at the moment only version 1 is supported"));
			}
		}
		else { return Err(Error::new(ErrorKind::Other, "storage file has the wrong type at version pos")); }

		//flags
		let (part, _) = try!(file.read_msgpack_value());
		if let Value::UInt32(_) = part {}
		else { return Err(Error::new(ErrorKind::Other, "format error: wrong type of flags or not available")); }

		//TODO CAS, Encryption, Compressed Flags

		//value
		let (part, _) = try!(file.read_msgpack_value());
		if let Value::Bin(x) = part {
			let len = x.len() as usize;
			let mut source = Cursor::new(x);

			let bytes_written = try!(output_handle.write_from_read(&mut source, len));
			assert_eq!(bytes_written, len);
			return Ok(());
		}
		else { return Err(Error::new(ErrorKind::Other, "wrong entry as value position")); }

		//key
		/*
		let part = try!(file.read_value());
		if let msgpackio::Value::BinStart(x) = part {
			try!(file.seek(io::SeekFrom::Current(x as i64))); //read or skip
		}
		else { return Err(Error::new(ErrorKind::Other, "wrong entry at key position")); }
		*/

		//Ok(())
	}

	//put a value into repository from reader, key is given by result
	fn put(&mut self, key_handle: &mut ReadHandle, value_handle: &mut ReadHandle) -> Result<()>
	{
		let mut key_handle = key_handle;
		let mut value_handle = value_handle;
		let key_info = try!(KeyInfo::new(&self, key_handle));

		assert!(key_info.data_file_path.is_absolute());

		debug!("[fs-storage-put]: key-hash: {}", key_info.key_hash.to_hex());
		debug!("[fs-storage-put]: path: {}", key_info.data_file_path.display());

		//TODO remove this and integrate it to write code below
		//detect value length
		let value_length = match value_handle {

			&mut ReadHandle::Reader(_, opt_len) => {
				try!(opt_len.ok_or(Error::new(ErrorKind::Other, "requires value length")))
			}
			&mut ReadHandle::Slice(slice) => slice.len()
		};

		//check for exist and read only flag
		if key_info.data_file_path.exists() {
			//if read only error
			//else go to overwrite value
			debug!("[fs-storage-put]: file already exists going to overwrite");

			//read in flags
			//TODO CAS, Encryption, Compressed Flags
		}

		//create dir if it not exists
		if !key_info.data_file_path.as_path().parent().unwrap().is_dir() {
			try!(fs::create_dir_all(key_info.data_file_path.as_path().parent().unwrap()));
		}

		//write new file
		let mut file = try!(File::create(key_info.data_file_path.as_path()));

		try!(file.write_msgpack_pos_fixint(1)); //version
		try!(file.write_msgpack_u32(0)); //flags
		//write the data value
		let bytes_written = match value_handle {
			&mut ReadHandle::Reader(ref mut reader, _) => {
				let mut reader = reader;
				try!(file.write_msgpack_bin_read(&mut reader, value_length))
			}
			&mut ReadHandle::Slice(ref slice) => {
				try!(file.write_msgpack_bin(slice))
			}
		};

		//can be more msgpack header
		assert!(bytes_written as usize >= value_length);

		let bytes_written= try!(file.write_msgpack_bin(key_info.key_buf.as_slice()));	//key complete
		assert_eq!(bytes_written, key_info.key_buf.len());

		Ok(())
	}

	//delete a value from repository by key
	fn delete(&mut self, key_handle: &mut ReadHandle) -> Result<()>
	{
		let mut key_handle = key_handle;
		let key_info = try!(KeyInfo::new(&self, key_handle));

		debug!("[fs-storage-delete]: key-hash: {}", key_info.key_hash.to_hex());
		debug!("[fs-storage-delete]: path: {}", key_info.data_file_path.display());

		assert!(key_info.data_file_path.is_absolute());
		if key_info.data_file_path.exists() {
			//delete file
			try!(fs::remove_file(key_info.data_file_path));
			Ok(())
		}
		else {
			return Err(Error::new(ErrorKind::Other, "no entry with this key found"));
		}
	}
}





