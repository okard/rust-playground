

//use std::collections::HashMap;

//in memory storage

	//use a hashmap or something similiar
	//trie?

extern crate crypto;

use std::collections::HashMap;

use std::io::{Cursor, Result, Error, ErrorKind};

use super::core::{ReadHandle, WriteHandle, KeyValueStorage, ContentStorage};

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


	fn internal_get(&self, key_handle: &mut ReadHandle, output_handle: &mut WriteHandle) -> Result<()>
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

	fn internal_delete(&mut self, key_handle: &mut ReadHandle) -> Result<()>
	{
		let key = try!(key_handle.to_vec());
		self.map.remove(&key);
		Ok(())
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
		self.internal_get(key_handle, output_handle)
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
		self.internal_delete(key_handle)
	}

}


impl ContentStorage for MemoryStorage
{
	///
	/// put content into storage
	///
	fn put(&mut self, content_handle: &mut ReadHandle, key_output: &mut WriteHandle) -> Result<()>
	{
		use crypto::digest::{Digest};
		use crypto::blake2b::{Blake2b};

		let value = try!(content_handle.to_vec());

		//create hash key from content
		const KEY_SIZE : usize = 32;
		let mut blake2b : &mut Digest = &mut Blake2b::new(KEY_SIZE); //32 or 64?
		blake2b.input(&value);
		let mut content_hash = vec![0u8; KEY_SIZE];
		blake2b.result(&mut content_hash);
		//write key back
		{
			let len = content_hash.len();
			let mut reader = &mut Cursor::new(&content_hash[..]);
			let bytes_written = try!(key_output.write_from_read(reader, len));
			assert_eq!(bytes_written, len);
		}
		//safe in memory
		self.map.insert(content_hash, value);
		Ok(())
	}

	///
	/// get content with key
	///
	fn get(&self, key_handle: &mut ReadHandle, output_handle: &mut WriteHandle) -> Result<()>
	{
		self.internal_get(key_handle, output_handle)
	}


	///
	/// delete content
	///
	fn delete(&mut self, key_handle: &mut ReadHandle) -> Result<()>
	{
		self.internal_delete(key_handle)
	}

}

