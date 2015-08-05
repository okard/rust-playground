


use std::io::{Result, Read};


//TODO inline hint?
///
/// Reads a specific length from a Read trait into given Vec
///
pub fn read_to_vec(reader: &mut Read, size: usize, buf: &mut Vec<u8>) -> Result<usize>
{
	if buf.len() < size {
		buf.reserve(size);
		while buf.len() < size {
			buf.push(0);
		}
	}
	assert!(buf.len() >= size);
	let result = try!(reader.read(buf.as_mut_slice()));
	assert_eq!(result, size);
	
	return Ok(result);
}


