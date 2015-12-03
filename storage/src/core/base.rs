
use std::io::{Result}; //TODO use custom error
use super::{ReadHandle, WriteHandle};


///
/// The absolute minimum interface for a key-value-storage
///
pub trait KeyValueStorage
{
	///
	/// retrieve a value from storage, the key is read through the key_handle
	/// the content is written to the write handle 
	///
	fn get(&self, key_handle: &mut ReadHandle, output_handle: &mut WriteHandle) -> Result<()>;
	
	///
	/// Put a value into the storage the key is read through the key handle
	/// the value content is read by the value_handle
	///
	fn put(&mut self, key_handle: &mut ReadHandle, value_handle: &mut ReadHandle) -> Result<()>;
	
	/// 
	/// delete a value from repository by key
	///
	fn delete(&mut self, key_handle: &mut ReadHandle) -> Result<()>;
	
	//fn has_flag(&self, key_handle: &mut ReadHandle, flag: StoreFlags) -> Result<bool>
	//fn toggle_flag(&mut self, key_handle: &mut ReadHandle, flag: StoreFlags) -> Result<bool>
}

///
/// A content adressed storage (CAS)
///
pub trait ContentStorage
{
	///
	/// put content into storage
	///
	fn put(&mut self, content: &mut ReadHandle, key_output: &mut WriteHandle) -> Result<()>;
	
	///
	/// get content with key
	///
	fn get(&self, key: &mut ReadHandle, output: &mut WriteHandle) -> Result<()>;
	
	///
	/// delete content
	///
	fn delete(&mut self, key: &mut ReadHandle) -> Result<()>;
	
	//TODO ?verify function -> check hash(value) == key
}
