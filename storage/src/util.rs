
use std::io::{self, Result, Read, Write, ErrorKind};


//copy with length limiting
pub fn copy<R: Read, W: Write>(r: &mut R, w: &mut W, len_max: u64) -> io::Result<u64> {
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
