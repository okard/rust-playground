
use std::io::{Read, Write, Result};

pub enum ReadHandle<'a> 
{ 
	Reader(&'a mut Read, Option<usize>), 
	Slice(&'a [u8]) 
}

pub enum WriteHandle<'a> 
{ 
	Writer(&'a mut Write, Option<usize>), 
	Slice(&'a mut [u8]) 
}

mod base;	//base implementations
mod meta;	//meta data interface
mod seek;	//seek interface


pub use self::base::KeyValueStorage;
pub use self::base::ContentStorage;
//meta
//seek


//TODO maintenance trait with verify,repair functions?


impl<'a> ReadHandle<'a>
{
	pub fn to_vec(&mut self) -> Result<Vec<u8>>
	{
		match self 
		{
			&mut ReadHandle::Reader(ref mut reader, size_opt) => 
			{
				//read key complete
				let mut read_buf : Vec<u8> = if let Some(len) = size_opt { Vec::with_capacity(len) } else { Vec::new() };
				try!(reader.read_to_end(&mut read_buf));
				let read_buf = read_buf; //make readonly
				if let Some(len) = size_opt {
					assert!(read_buf.len() == len);
				}
				Ok(read_buf)
			}
			&mut ReadHandle::Slice(slice) => { 
				let mut read_buf : Vec<u8> = Vec::with_capacity(slice.len());
				read_buf.push_all(slice);
				Ok(read_buf)
			}
		}
	}
}


