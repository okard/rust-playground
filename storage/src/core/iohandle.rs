
use std::io::{Read, Write, Result, Cursor, Error, ErrorKind};
use super::super::util;

///
/// Interface for reading a byte key
///
pub enum ReadHandle<'a>
{
	Reader(&'a mut Read, Option<usize>),
	Slice(&'a [u8])
}

///
/// Interface to write binary data
///
pub enum WriteHandle<'a>
{
	Writer(&'a mut Write, Option<usize>),
	Slice(&'a mut [u8])
}


impl<'a> ReadHandle<'a>
{
	///
	/// Read the complete key into a Vec<u8>
	///
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
					assert_eq!(read_buf.len(), len);
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


impl<'a> WriteHandle<'a>
{
	///
	/// Write the data with a reader as source
	///
	pub fn write_from_read(&mut self, reader: &mut Read, length: usize) -> Result<usize>
	{
		let mut reader = reader;
		match *self
		{
			//the case of a write trait
			WriteHandle::Writer(ref mut writer, opt_size) => {
				let mut writer = writer;
				assert_eq!(length, opt_size.unwrap_or(length));
				let bytes_written = try!(util::copy(&mut reader, &mut writer, length as u64));
				return Ok(bytes_written as usize);
			}
			//the case of a target slice
			WriteHandle::Slice(ref mut target_slice) => {
				let mut target_slice = target_slice;
				if length != target_slice.len() {
					return Err(Error::new(ErrorKind::Other, "target slice must have the right size"));
				}
				let bytes_written = try!(reader.read(&mut target_slice));
				assert_eq!(length, bytes_written);
				return Ok(bytes_written);
			}
		}
	}

	///
	/// Write the data from a slice
	///
	pub fn write_slice(&mut self, slice: &[u8]) -> Result<usize>
	{
		let mut c = Cursor::new(slice);

		match *self
		{
			//the case of a write trait
			WriteHandle::Writer(ref mut writer, opt_size) => {
				let mut writer = writer;
				assert_eq!(slice.len(), opt_size.unwrap_or(slice.len()));
				let bytes_written = try!(util::copy(&mut c, &mut writer, slice.len() as u64));
				return Ok(bytes_written as usize);
			}
			//the case of a target slice
			WriteHandle::Slice(ref mut target_slice) => {
				let mut target_slice = target_slice;
				if slice.len() != target_slice.len() {
					return Err(Error::new(ErrorKind::Other, "target slice must have the right size"));
				}
				let bytes_written = try!(c.read(&mut target_slice));
				assert_eq!(slice.len(), bytes_written);
				return Ok(bytes_written);
			}
		}
	}
}
