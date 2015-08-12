


use std::io::{Result, Read, Write, ErrorKind};


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

//copy with length limiting
pub fn copy<R: Read, W: Write>(r: &mut R, w: &mut W, len_max: u64) -> Result<u64> {
	let mut buf = [0; 1024];
	let mut written : u64 = 0;
	
	while written < len_max {
		let len = match r.read(&mut buf) {
			Ok(0) => return Ok(written),
			Ok(len) => len,
			Err(ref e) if e.kind() == ErrorKind::Interrupted => continue,
			Err(e) => return Err(e),
		};
		
		if (written+len as u64) < len_max {
			try!(w.write_all(&buf[..len]));
			written += len as u64;
		}
		else {
			let to_write : usize = len_max as usize - written as usize;
			let to_write = if to_write > len {len} else {to_write}; //required?
			try!(w.write_all(&buf[..to_write]));
			written += to_write as u64;
		}
	}
	Ok(written)
}


