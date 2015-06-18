

//use std::collections::HashMap;

//in memory storage

	//use a hashmap or something similiar
	//trie?
	
use std::collections::HashMap;

use std::io::{Cursor, Result, Error, ErrorKind};

use super::core::{ReadHandle, WriteHandle, KeyValueStorage};
use super::util;

///
/// Memory Storage
///
pub struct MemoryStorage
{
	map: HashMap<Vec<u8>, Vec<u8>>
}

impl MemoryStorage
{
	pub fn new() -> MemoryStorage
	{
		MemoryStorage {
			map: HashMap::new()
		}
	}
}

///
/// Key Value for memory storage
///
impl KeyValueStorage for MemoryStorage
{
	///
	/// retrieve a value from storage, the key is read through the key_handle
	/// the content is written to the write handle 
	///
	fn get(&self, key_handle: &mut ReadHandle, output_handle: &mut WriteHandle) -> Result<()>
	{
		let mut key = Vec::new();
		let key_len = key_handle.len().unwrap();
		try!(util::copy(&mut key_handle.get_reader(), &mut key, key_len as u64));
		
		match self.map.get(&key)
		{
			Some(v) => 
			{
				let mut c = Cursor::new(v.as_slice());
				try!(util::copy(&mut c, &mut output_handle.get_writer(), v.len() as u64));
			}
			None => {
				return Err(Error::new(ErrorKind::Other, "key not found"));
			}
		}
		Ok(())
	}
	
	///
	/// Put a value into the storage the key is read through the key handle
	/// the value content is read by the value_handle
	///
	fn put(&mut self, key_handle: &mut ReadHandle, value_handle: &mut ReadHandle) -> Result<()>
	{
		let mut key = Vec::new();
		let key_len = key_handle.len().unwrap();
		try!(util::copy(&mut key_handle.get_reader(), &mut key, key_len as u64));
		
		let mut value = Vec::new();
		let value_len = value_handle.len().unwrap();
		try!(util::copy(&mut value_handle.get_reader(), &mut value, value_len as u64));
		
		self.map.insert(key, value);
		
		Ok(())
	}
	
	/// 
	/// delete a value from repository by key
	///
	fn delete(&mut self, key_handle: &mut ReadHandle) -> Result<()>
	{
		let mut key = Vec::new();
		let key_len = key_handle.len().unwrap();
		try!(util::copy(&mut key_handle.get_reader(), &mut key, key_len as u64));
		
		self.map.remove(&key);
		
		Ok(())
	}
	
}

