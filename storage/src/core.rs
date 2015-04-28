
use std::io::{Result, Read, Write};

//define own result with own error

//options/flags like readonly?
//enum EntryFlags { ReadOnly, Deleteable, }
//set_flags api

// struct ReadHandle(&mut Read, size: usize);
// struct WriteHandle(&mut Write, size: Option<usize>);

/**
* The absolute minimum interface for a key-value-storage
*/
pub trait KeyValueStorage
{
	
	//get a value from repository by key, data is written to writer
		//read key from reader
		//write output to writer or deliver IoResult
	fn get(&self, key_reader: &mut Read, key_size: usize, value_out: &mut Write) -> Result<()>;
	
	//put a value into repository from reader, key is given by result
		// read key from key reader
		// read value from value reader
		// size for key and value
	fn put(&mut self, key_reader: &mut Read, key_size: usize, value_reader: &mut Read, value_size: usize) -> Result<()>;
	
	//delete a value from repository by key								
	fn delete(&mut self, key_reader: &mut Read, key_size: usize) -> Result<()>;
}
