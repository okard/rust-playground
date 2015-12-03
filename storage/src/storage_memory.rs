

//use std::collections::HashMap;

//in memory storage

	//use a hashmap or something similiar
	//trie?

use std::collections::HashMap;

use std::io::{Cursor, Result, Error, ErrorKind};

use super::core::{ReadHandle, WriteHandle, KeyValueStorage};

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
				let bytes_written = try!(output_handle.write_from_read(&mut c, v.len()));
				if bytes_written == v.len() {
					return Ok(())
				 } else {
					 return Err(Error::new(ErrorKind::Other, "can't write all bytes to output_handle"));
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

