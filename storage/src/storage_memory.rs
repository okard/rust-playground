

//use std::collections::HashMap;

//in memory storage

	//use a hashmap or something similiar
	//trie?
	
use std::collections::HashMap;

use std::io::{Cursor, Result, Read, Error, ErrorKind};

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
		let key = try!(key_handle.to_vec());
		
		match self.map.get(&key)
		{
			Some(v) =>
			{
				let mut c = Cursor::new(v.as_slice());
				
				match output_handle {
					&mut WriteHandle::Writer(ref mut writer, opt_size) => {
						let mut writer = writer;
						assert_eq!(v.len(), opt_size.unwrap_or(v.len()));
						try!(util::copy(&mut c, &mut writer, v.len() as u64));
					}
					&mut WriteHandle::Slice(ref mut slice) => {
						let mut slice = slice;
						if v.len() == slice.len() {
							return Err(Error::new(ErrorKind::Other, "target slice must have the right size"));
						}
						let bytes_written = try!(c.read(&mut slice));
						assert_eq!(v.len(), bytes_written);
					}
				}
			
				
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
		let key = try!(key_handle.to_vec());
		let value = try!(value_handle.to_vec());
		self.map.insert(key, value);
		Ok(())
	}
	
	/// 
	/// delete a value from repository by key
	///
	fn delete(&mut self, key_handle: &mut ReadHandle) -> Result<()>
	{
		let key = try!(key_handle.to_vec());		
		self.map.remove(&key);
		
		Ok(())
	}
	
}

