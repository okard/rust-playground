
use std::io::{Result, Read, Write};

//define own result with own error

enum Flags
{
	ReadOnly,
	Deleteable
}

//options/flags like readonly?
//enum EntryFlags { ReadOnly, Deleteable, }
//set_flags api


/// A ReadHandle that wraps a Read and length
pub struct ReadHandle<'a>
{
	pub reader: &'a mut Read,
	pub len: Option<usize>
}

impl<'a> ReadHandle<'a>
{
	pub fn new_with_len(reader: &mut Read, len: usize) -> ReadHandle
	{
		ReadHandle {
			reader: reader,
			len: Some(len)
		}
	}
	
	pub fn new(reader: &mut Read) -> ReadHandle
	{
		ReadHandle {
			reader: reader,
			len: None
		}
	}
}

/// A write handle that wraps std::io::Write and a length
pub struct WriteHandle<'a>
{
	pub writer: &'a mut Write,
	pub len: Option<usize>
}

impl<'a> WriteHandle<'a>
{
	pub fn new_with_len(writer: &mut Write, len: usize) -> WriteHandle
	{
		WriteHandle {
			writer: writer,
			len: Some(len)
		}
	}
	
	pub fn new(writer: &mut Write) -> WriteHandle
	{
		WriteHandle {
			writer: writer,
			len: None
		}
	}
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
