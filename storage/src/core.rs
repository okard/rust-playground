
use std::io::{Result, Read, Write};

//define own result with own error

enum StoreFlags
{
	ReadOnly, //no overwrite or delete
	//DeleteAble //overwrite allowed?
}

//TODO trait as read_handles?

///
/// Read handle wraps std::io::Read and a length
///
pub trait ReadHandle
{
	fn get_reader(&mut self) -> &mut Read;
	fn len(&self) -> Option<usize>;
}

///
/// A write handle that wraps std::io::Write and a length
///
pub trait WriteHandle
{
	fn get_writer(&mut self) -> &mut Write;
	fn len(&self) -> Option<usize>;
}


///
/// The absolute minimum interface for a key-value-storage
///
pub trait KeyValueStorage
{
	
	//get a value from repository by key, data is written to writer
		//read key from reader
		//write output to writer or deliver IoResult
	fn get(&self, key_handle: &mut ReadHandle, output_handle: &mut WriteHandle) -> Result<()>;
	
	//put a value into repository from reader, key is given by result
		// read key from key reader
		// read value from value reader
		// size for key and value
	fn put(&mut self, key_handle: &mut ReadHandle, value_handle: &mut ReadHandle) -> Result<()>;
	
	//delete a value from repository by key								
	fn delete(&mut self, key_handle: &mut ReadHandle) -> Result<()>;
}

///
/// A content adressed storage (CAS)
///
pub trait ContentStorage
{
	fn put(&mut self, content: &mut ReadHandle) -> Result<&mut ReadHandle>;
	fn get(&self, key: &mut ReadHandle, output: &mut WriteHandle) -> Result<()>;
	fn delete(&mut self, key: &mut ReadHandle) -> Result<()>;
}
