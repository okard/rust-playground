
//filesystem storage 
extern crate msgpackio;
extern crate rustc_serialize;


use std::io::{Result, Read, Error, ErrorKind, Cursor};
use std::path::{Path, PathBuf};
use std::fs::{self, File, PathExt};
use std::env;

use self::rustc_serialize::hex::ToHex;

use crypto::digest::{Digest};
use crypto::blake2b::{Blake2b};

use self::msgpackio::read::{MsgPackReader, Value};
use self::msgpackio::write::{MsgPackWriter};

use super::core::{ReadHandle, WriteHandle, KeyValueStorage};
use super::util;


bitflags! {
	flags Flags: u32 {
		const FLAG_READONLY 	= 0b00000001,
		const FLAG_DELETEABLE 	= 0b00000010,
		const FLAG_ENCRYPTED	= 0b00000100,
		const FLAG_COMPRESSED	= 0b00001000,
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
		
		//read key complete
		let mut key_buf : Vec<u8> = if let Some(len) = key_handle.len() { Vec::with_capacity(len) } else { Vec::new() };
		try!(key_handle.get_reader().read_to_end(&mut key_buf));
		let key_buf = key_buf; //make readonly
		if let Some(len) = key_handle.len() {
			assert!(key_buf.len() == len);
		}
		
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
				
		Ok(FilesystemStorage {
			repo_path : path,
		})
	}
}


impl KeyValueStorage for FilesystemStorage
{
	
	
	//type Key = Reader; //type is byte slice trait? 
	
	//use internal a sha256 hash of key for indexing content
	//safe original key internal in object structure together with hash, etc
	
	
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
		
		//value
		let (part, _) = try!(file.read_msgpack_value());
		if let Value::Bin(x) = part {
			let len = x.len() as u64;
			let mut source = Cursor::new(x);
			try!(util::copy(&mut source, &mut output_handle.get_writer(), len)); //copy only x bytes to value
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
		
		Ok(())
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
		
		let value_length = try!(value_handle.len().ok_or(Error::new(ErrorKind::Other, "requires value length")));
		
		//check for exist and read only flag
		if key_info.data_file_path.exists() {
			//if read only error
			//else go to overwrite value
			debug!("[fs-storage-put]: file already exists going to overwrite");
		}
		
		//create dir if it not exists
		if !key_info.data_file_path.as_path().parent().unwrap().is_dir() {
			try!(fs::create_dir_all(key_info.data_file_path.as_path().parent().unwrap()));
		}
		
		//write new file
		let mut file = try!(File::create(key_info.data_file_path.as_path()));
		
		try!(file.write_msgpack_pos_fixint(1)); //version
		//flags?
		let bytes_written = try!(file.write_msgpack_bin_read(&mut value_handle.get_reader(), value_length));
		
		assert_eq!(bytes_written as usize, value_length);
		
		try!(file.write_msgpack_bin(key_info.key_buf.as_slice()));	//key complete
		
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





