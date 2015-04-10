
//filesystem storage 
extern crate msgpackio;

use std::io::{self, Result, Read, Write, Seek, Error, ErrorKind};
use std::path::{Path, PathBuf};
use std::fs::{self, File, PathExt};
use std::env;

use crypto::digest::{Digest};
use crypto::blake2b::{Blake2b};

use self::msgpackio::{MsgPackReader, MsgPackWriter};

use super::core::{KeyValueStorage};


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
	fn get(&self, key_reader : &mut Read, key_size: usize, value_out: &mut Write) -> Result<()>
	{
		const KEY_SIZE : usize = 32;
		
		//read key complete
		let mut key_buf : Vec<u8> = Vec::with_capacity(key_size);
		try!(key_reader.read_to_end(&mut key_buf));
		
		//compute hash from key
		let mut blake2b : &mut Digest = &mut Blake2b::new(KEY_SIZE);
		blake2b.input(&key_buf);
		let key_hash = blake2b.result_str();
		
		//build up path
		let mut path = PathBuf::from(&self.repo_path);
		path.push(&key_hash[0..2]);
		path.push(&key_hash[2..]);
		
		println!("key-hash: {}", key_hash);
		println!("path: {}", path.display());
		
		//check if file exists
		if !path.exists() {
			return Err(Error::new(ErrorKind::Other, "key does not exist"));
		}
		
		//open file
		let mut file = try!(File::open(path));
		
		//version
		if let msgpackio::Value::UInt8(x) = file.read_value().unwrap() {
			if x != 1 {
				return Err(Error::new(ErrorKind::Other, "storage file: at the moment only version 1 is supported"));
			}
		}
		else { return Err(Error::new(ErrorKind::Other, "storage file has the wrong type at version pos")); }
		
		
		//key / compare?
		if let msgpackio::Value::BinStart(x) = file.read_value().unwrap() {
			try!(file.seek(io::SeekFrom::Current(x as i64))); //skip key
		}
		else { return Err(Error::new(ErrorKind::Other, "wrong entry at key position")); }
		
		//value
		if let msgpackio::Value::BinStart(x) = file.read_value().unwrap() {
			let mut vw = value_out;
			try!(io::copy(&mut file, &mut vw)); //copy only x bytes to value
		}
		else { return Err(Error::new(ErrorKind::Other, "wrong entry as value position")); }
		
		
		Ok(())
	}
	
	//put a value into repository from reader, key is given by result
	fn put(&mut self, key_reader : &mut Read, key_size: usize, value_reader: &mut Read, value_size: usize) -> Result<()>
	{
		const KEY_SIZE : usize = 32;
		
		//read key complete
		let mut key_buf : Vec<u8> = Vec::with_capacity(key_size);
		try!(key_reader.read_to_end(&mut key_buf));
		let key_buf = key_buf;
		assert!(key_buf.len() == key_size);
		
		//create hash from key
		let mut blake2b : &mut Digest = &mut Blake2b::new(KEY_SIZE); //32 or 64?
		blake2b.input(&key_buf);
		let mut key_hash = [0u8; KEY_SIZE];
		blake2b.result(&mut key_hash);
		
		//get hex representation:
		let key_hash_str = blake2b.result_str();
		
		//build up path
		let mut path = PathBuf::from(&self.repo_path);
		path.push(&key_hash_str[0..2]);
		path.push(&key_hash_str[2..]);
		
		assert!(path.is_absolute());
		
		println!("key-hash: {}", key_hash_str);
		println!("path: {}", path.display());
		
		//check for exist and read only flag
		if path.exists() {
			//if read only error
			//else go to overwrite value
			println!("file already exists going to overwrite"); //debug info
		}
		
		//create dir if it not exists
		if !path.as_path().parent().unwrap().is_dir() {
			try!(fs::create_dir_all(path.as_path().parent().unwrap()));
		}
		
		//write new file
		let mut file = try!(File::create(path.as_path()));
		
		try!(file.write_msgpack_pos_fixint(1)); //version
		//flags?
		try!(file.write_msgpack_bin(key_buf.as_slice()));	//key
		try!(file.write_msgpack_bin_header(value_size));	//header
		let mut vr = value_reader;
		let bytes_written = try!(io::copy(&mut vr, &mut file));
		assert_eq!(bytes_written as usize, value_size);
		
		Ok(())
	}
	
	//delete a value from repository by key								
	fn delete(&mut self, key_reader : &mut Read, key_size: usize) -> Result<()>
	{
		let mut key_buf : Vec<u8> = Vec::with_capacity(key_size);
		try!(key_reader.read_to_end(&mut key_buf));
		
		Ok(())
	}
	
	
}





