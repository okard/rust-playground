
//filesystem storage 
extern crate msgpackio;
extern crate rustc_serialize;


use std::io::{self, Result, Read, Write, Error, ErrorKind};
use std::path::{Path, PathBuf};
use std::fs::{self, File, PathExt};
use std::env;

use self::rustc_serialize::hex::ToHex;

use crypto::digest::{Digest};
use crypto::blake2b::{Blake2b};

use self::msgpackio::{MsgPackReader, MsgPackWriter};

use super::core::{ReadHandle, WriteHandle, KeyValueStorage};

//copy with length limiting
pub fn copy<R: Read, W: Write>(r: &mut R, w: &mut W, len_max: u64) -> io::Result<u64> {
	let mut buf = [0; 1024];
	let mut written : u64 = 0;
	
	while written < len_max {
		let len = match r.read(&mut buf) {
			Ok(0) => return Ok(written),
			Ok(len) => len,
			Err(ref e) if e.kind() == ErrorKind::Interrupted => continue,
			Err(e) => return Err(e),
		};
		
		if (written+len as u64) < len_max {
			try!(w.write_all(&buf[..len]));
			written += len as u64;
		}
		else {
			let to_write : usize = len_max as usize - written as usize;
			let to_write = if to_write > len {len} else {to_write}; //required?
			try!(w.write_all(&buf[..to_write]));
			written += to_write as u64;
		}
	}
	Ok(written)
}


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
		let mut key_buf : Vec<u8> = if let Some(len) = key_handle.len { Vec::with_capacity(len) } else { Vec::new() };
		try!(key_handle.reader.read_to_end(&mut key_buf));
		let key_buf = key_buf; //make readonly
		if let Some(len) = key_handle.len {
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
	repo_path: PathBuf
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
		
		//println!("repo_path: {}", path.display());
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
		let key_info = try!(KeyInfo::new(&self, &mut key_handle));
		
		println!("key-hash: {}", key_info.key_hash.to_hex());
		println!("path: {}", key_info.data_file_path.display());
		
		//check if file exists
		if !key_info.data_file_path.exists() {
			return Err(Error::new(ErrorKind::Other, "key does not exist"));
		}
		
		//open file
		let mut file = try!(File::open(key_info.data_file_path));
		
		//version
		let part = try!(file.read_value());
		if let msgpackio::Value::UInt8(x) = part {
			if x != 1 {
				return Err(Error::new(ErrorKind::Other, "storage file: at the moment only version 1 is supported"));
			}
		}
		else { return Err(Error::new(ErrorKind::Other, "storage file has the wrong type at version pos")); }
		
		//value
		let part = try!(file.read_value());
		if let msgpackio::Value::BinStart(x) = part {
			try!(copy(&mut file, &mut output_handle.writer, x as u64)); //copy only x bytes to value
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
		let key_info = try!(KeyInfo::new(&self, &mut key_handle));
		
		assert!(key_info.data_file_path.is_absolute());
		
		println!("key-hash: {}", key_info.key_hash.to_hex());
		println!("path: {}", key_info.data_file_path.display());
		
		let value_length = try!(value_handle.len.ok_or(Error::new(ErrorKind::Other, "requires value length")));
		
		//check for exist and read only flag
		if key_info.data_file_path.exists() {
			//if read only error
			//else go to overwrite value
			println!("file already exists going to overwrite"); //debug info
		}
		
		//create dir if it not exists
		if !key_info.data_file_path.as_path().parent().unwrap().is_dir() {
			try!(fs::create_dir_all(key_info.data_file_path.as_path().parent().unwrap()));
		}
		
		//write new file
		let mut file = try!(File::create(key_info.data_file_path.as_path()));
		
		try!(file.write_msgpack_pos_fixint(1)); //version
		//flags?
		try!(file.write_msgpack_bin_header(value_length));	//value bin header
		let bytes_written = try!(copy(&mut value_handle.reader, &mut file, value_length as u64)); //limit to given value bytes
		assert_eq!(bytes_written as usize, value_length);
		
		try!(file.write_msgpack_bin(key_info.key_buf.as_slice()));	//key complete
		
		Ok(())
	}
	
	//delete a value from repository by key								
	fn delete(&mut self, key_handle: &mut ReadHandle) -> Result<()>
	{
		let mut key_handle = key_handle;
		let key_info = try!(KeyInfo::new(&self, &mut key_handle));
		
		assert!(key_info.data_file_path.is_absolute());
		if key_info.data_file_path.exists() {
			//delete file
		}
		
		Ok(())
	}
	
	
}





