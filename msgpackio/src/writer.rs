

use std::io::{Result, Error, ErrorKind, Write};

use byteorder::{WriteBytesExt, BigEndian};

/**
* The MsgPackWriter trait to write msgpack data
*/
pub trait MsgPackWriter : Write
{
	/**
	* Write nil
	*/
	fn write_msgpack_nil(&mut self) -> Result<usize>
	{
		return self.write(&[0xc0]);
	}

	/**
	* Write a boolean value
	*/
	fn write_msgpack_bool(&mut self, value : bool) -> Result<usize>
	{
		if value {
			return self.write(&[0xc3]); //true
		}
		else {
			return self.write(&[0xc2]); //false
		}
	}
	
	fn write_msgpack_pos_fixint(&mut self, value: u8) -> Result<usize>
	{
		if value > 128 {
			return Err(Error::new(ErrorKind::Other, "can only store max of 128 as pos fixint"));
		}
		return self.write(&[value]);
	}
	
	fn write_msgpack_neg_fixint(&mut self, value: i8) -> Result<usize>
	{
		if value < -32 || value >=0 {
			return Err(Error::new(ErrorKind::Other, "can only store from 0 to -32 as neg fixint"));
		}
		return self.write(&[(0xe0u8|-value as u8) as u8]); //right or make pos first?
	}
	
	fn write_msgpack_u8(&mut self, value : u8) -> Result<usize>
	{
		//pos fixint check here?
		return self.write(&[0xcc, value]);
	}
	
	fn write_msgpack_u16(&mut self, value : u16) -> Result<usize>
	{
		try!(self.write(&[0xcd]));
		try!(self.write_u16::<BigEndian>(value));
		Ok(3)
	}
	
	fn write_msgpack_u32(&mut self, value : u32) -> Result<usize>
	{
		try!(self.write(&[0xce]));
		try!(self.write_u32::<BigEndian>(value));
		Ok(5)
	}
	
	fn write_msgpack_u64(&mut self, value : u64) -> Result<usize>
	{
		try!(self.write(&[0xcf]));
		try!(self.write_u64::<BigEndian>(value));
		Ok(9)
	}
	
	fn write_msgpack_i8(&mut self, value: i8) -> Result<usize>
	{
		//neg fixint check here?
		return self.write(&[0xd0, value as u8]);
	}
	
	fn write_msgpack_i16(&mut self, value: i16) -> Result<usize>
	{
		try!(self.write(&[0xd1]));
		try!(self.write_i16::<BigEndian>(value));
		return Ok(3);
	}
	
	fn write_msgpack_i32(&mut self, value: i32) -> Result<usize>
	{
		try!(self.write(&[0xd2]));
		try!(self.write_i32::<BigEndian>(value));
		return Ok(5);
	}
	
	fn write_msgpack_i64(&mut self, value: i64) -> Result<usize>
	{
		try!(self.write(&[0xd3]));
		try!(self.write_i64::<BigEndian>(value));
		return Ok(9);
	}
	
	fn write_msgpack_f32(&mut self, value: f32) -> Result<usize>
	{
		try!(self.write(&[0xca]));
		try!(self.write_f32::<BigEndian>(value));
		return Ok(5);
	}
	
	fn write_msgpack_f64(&mut self, value: f64) -> Result<usize>
	{
		try!(self.write(&[0xcb]));
		try!(self.write_f64::<BigEndian>(value));
		return Ok(9);
	}
	
	/* writes a str header chooses right format with size
    * fixstr
	* str 8
	* str 16
	* str 32
	*/
	fn write_msgpack_str_header(&mut self, length: usize) -> Result<usize>
	{
		//write with right size
		match length
		{
			//fixtsr 101XXXXX  stores a byte array whose length is upto 31 bytes
			x if x <= 31 => 
			{
				return self.write(&[(0xa0 | x) as u8]);
			},
			
			//str 8 stores a byte array whose length is upto (2^8)-1 bytes start with 0xd9
			x if x <= 2usize.pow(8)-1 => 
			{
				return self.write(&[0xd9, x as u8]);
			}
			
			//str 16 stores a byte array whose length is upto (2^16)-1 bytes starts with 0xda
			x if x <= 2usize.pow(16)-1 => 
			{
				try!(self.write(&[0xda]));
				try!(self.write_u16::<BigEndian>(x as u16));
				return Ok(3);
			}
			
			//str 32 stores a byte array whose length is upto (2^32)-1 bytes starts with 0xdb
			x if x <= 2usize.pow(32)-1 => 
			{
				try!(self.write(&[0xda]));
				try!(self.write_u32::<BigEndian>(x as u32));
				return Ok(5);
			}
			_ => { return Err(Error::new(ErrorKind::Other, "string is to big to write")); }
		}
	}
	
	
	/*
	write str from data
	*/
	fn write_msgpack_str(&mut self, value: &str) -> Result<usize>
	{
		let mut len = value.len();
		len = try!(self.write_msgpack_str_header(len));
		len += try!(self.write(value.as_bytes()));
		return Ok(len);
	}
	
	//extended interface fn write_str_read(&mut self, reader: &mut Read, length: usize)
	
	/**
	* write a binary header
	*/
	fn write_msgpack_bin_header(&mut self, length: usize) -> Result<usize>
	{
		match length
		{
			//bin 8 stores a byte array whose length is upto (2^8)-1 bytes:
			x if x <= 2usize.pow(8)-1 => 
			{
				try!(self.write(&[0xc4, x as u8]));
				return Ok(2);
			}
			//bin 16 stores a byte array whose length is upto (2^16)-1 bytes:
			x if x <= 2usize.pow(16)-1 => 
			{
				try!(self.write(&[0xc5 ]));
				try!(self.write_u16::<BigEndian>(x as u16));
				return Ok(3);
			}
			
			//bin 32 stores a byte array whose length is upto (2^32)-1 bytes:
			x if x <= 2usize.pow(32)-1 => 
			{
				try!(self.write(&[0xc6]));
				try!(self.write_u32::<BigEndian>(x as u32));
				return Ok(5);
				
			}
			_ => { return Err(Error::new(ErrorKind::Other, "binary data is too big to write")); }
		}
	}
	
	//bin 8
	//bin 16
	//bin 32
	fn write_msgpack_bin(&mut self, data: &[u8]) -> Result<usize>
	{
		let mut len = try!(self.write_msgpack_bin_header(data.len()));
		len += try!(self.write(data));
		return Ok(len);
	}
	

	/*
	* fixarray
	* array 16
	* array 32
	*/
	fn write_msgpack_array_start(&mut self, element_count: usize) -> Result<usize>
	{
		match element_count
		{
			//fixarray stores an array whose length is upto 15 elements:
			x if x <= 15 => 
			{
				return self.write(&[
					(0x90 | x) as u8,
				]);
			}
			//array 16 stores an array whose length is upto (2^16)-1 elements:
			x if x <= 2usize.pow(16)-1 => 
			{
				try!(self.write(&[0xdc]));
				try!(self.write_u16::<BigEndian>(x as u16));
				return Ok(3);
			}
			
			//array 32 stores an array whose length is upto (2^32)-1 elements:
			x if x <= 2usize.pow(32)-1 => 
			{
				try!(self.write(&[0xdd]));
				try!(self.write_u32::<BigEndian>(x as u32));
				return Ok(5);
			}
		
			_ => { return Err(Error::new(ErrorKind::Other, "too many elements for array")); }
		}
	}
	//write simple starts
	
	/*
	* fixmap
	* map 16
	* map 32 
	*/
	fn write_msgpack_map_start(&mut self, pair_count: usize) -> Result<usize>
	{
		match pair_count
		{
			//fixmap stores a map whose length is upto 15 elements
			x if x <= 15 => {
				return self.write(&[(0x80|x) as u8]);
			}
			
			//map 16 stores a map whose length is upto (2^16)-1 elements
			x if x <= 2usize.pow(16)-1 => {
				try!(self.write(&[0xde]));
				try!(self.write_u16::<BigEndian>(x as u16));
				return Ok(3);
			}
			//map 32 stores a map whose length is upto (2^32)-1 elements
			x if x <= 2usize.pow(32)-1 => {
				try!(self.write(&[0xdf]));
				try!(self.write_u32::<BigEndian>(x as u32));
				return Ok(5);
			}
			
			_ => { return Err(Error::new(ErrorKind::Other, "too many elements for map")); }
		}
	}
	
	/**
	* fixext 1
	* fixext 2
	* fixext 4
	* fixext 8
	* fixext 16
	* ext 8
	* ext 16
	* ext 32
	*/
	fn write_msgpack_ext(&mut self, ty:i8, data: &[u8]) -> Result<usize>
	{
		//type is a signed 8-bit signed integer
		//type < 0 is reserved for future extension including 2-byte type information
		let len = data.len();
		match len
		{
			// 1 byte data
			x if x == 1 => {
				return self.write(&[0xd4, ty as u8, data[0]]);
			}
			
			//2 byte data
			x if x == 2 => {
				return self.write(&[0xd5, ty as u8, data[0], data[1]]);
			}
			
			//4 bytes
			x if x == 4 => {
				try!(self.write(&[0xd6, ty as u8]));
				try!(self.write(data));
				return Ok(6);
			}
			
			//8 bytes 0xd7 
			x if x == 8 => {
				try!(self.write(&[0xd7, ty as u8]));
				try!(self.write(data));
				return Ok(10);
			}
			
			//16 bytes 0xd8
			x if x == 8 => {
				try!(self.write(&[0xd8, ty as u8]));
				try!(self.write(data));
				return Ok(18);
			}
			
			//0xc7 ext 8 stores an integer and a byte array whose length is upto (2^8)-1 bytes:
			x if x <= 2usize.pow(8)-1 => {
				try!(self.write(&[0xc7, x as u8, ty as u8]));
				try!(self.write(data));
				return Ok(3+len);
			}
			
			//0xc8 ext 16 stores an integer and a byte array whose length is upto (2^16)-1 bytes:
			x if x <= 2usize.pow(16)-1 => {
				try!(self.write(&[0xc8]));
				try!(self.write_u16::<BigEndian>(x as u16));
				try!(self.write(&[ty as u8]));
				try!(self.write(data));
				return Ok(4+len);
			}
			
			// 0xc9 ext 32 stores an integer and a byte array whose length is upto (2^32)-1 bytes:
			x if x <= 2usize.pow(32)-1 => {
				try!(self.write(&[0xc9]));
				try!(self.write_u32::<BigEndian>(x as u32));
				try!(self.write(&[ty as u8]));
				try!(self.write(data));
				return Ok(6+len);
			}
			
			_ => { return Err(Error::new(ErrorKind::Other, "too big for ext element")); }
		}
	}
}

impl<T: Write> MsgPackWriter for T {}
