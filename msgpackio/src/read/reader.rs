
use std::io::{Result, Error, ErrorKind, Read};
use byteorder::{ReadBytesExt, BigEndian};

use id::{MsgPackId};
use read::value::{Value};


//TODO high level reader

//internal trait and external trait

//read_trait with slice?

//complex callback interface?

pub trait MsgPackReader : Read
{
	//read value with a reader callback which uses a inner read wrapper to read value
		//read_bin_cb(&mut self, cb: Fn(usize, &Read) -> Result<()>);
	
	//read with stream?
	//direct reads (with streams)?
	
	//buffer style, stream style?
	
	//fn read_header() -> 
	
	//TODO unsafe reading?
	
	
	fn read_msgpack_value(&mut self) -> Result<(Value,MsgPackId)>
	{
		let mut buf : [u8;1] = [0];
		let bytes_read = try!(self.read(&mut buf));
		
		if bytes_read == 0 {
			return Err(Error::new(ErrorKind::Other, "No data to read"));
		}
		
		let msgpack_id = MsgPackId::from_u8(buf[0]);
		if msgpack_id.is_none() {
			return Err(Error::new(ErrorKind::Other, "Not a valid start value for msgpack data"));
		}
		let msgpack_id =  msgpack_id.unwrap();
		
		match msgpack_id
		{
			MsgPackId::Nil => { return Ok((Value::Nil, msgpack_id)); }	
			
			//Boolean
			MsgPackId::False => { return Ok((Value::Boolean(false), msgpack_id)); }
			MsgPackId::True => { return Ok((Value::Boolean(true), msgpack_id)); }
			
			//fixed integer
			MsgPackId::FixPosInt => { return Ok((Value::UInt8(buf[0] & 0x7f), msgpack_id)); }
			MsgPackId::FixNegInt => { return Ok((Value::Int8(-((buf[0] & 0x1f) as i8)), msgpack_id)); } //right?
			
			/*
			MsgPackId::FixMap => { 
				return Ok(Value::MapStart((buf[0] & 0x0f) as usize));
			}
			
			MsgPackId::FixArray => { 
				return Ok(Value::ArrayStart((buf[0] & 0x0f) as usize));
			}
			*/
			
			MsgPackId::FixStr => { 
				let len = (buf[0] & 0x1f) as usize;
				let mut  buf = Vec::with_capacity(len);
				try!(self.read(&mut buf));
				
				if let Ok(str) = String::from_utf8(buf) {
					return Ok((Value::Str(str), msgpack_id));
				}
				else {
					return Err(Error::new(ErrorKind::Other, "Not a valid utf8 string"));
				}
			}
			
			MsgPackId::Bin8 => {
				let len = try!(self.read_u8()) as usize;
				let mut buf = Vec::with_capacity(len);
				try!(self.read(&mut buf));
				return Ok((Value::Bin(buf), msgpack_id));
			}
			
			MsgPackId::Bin16 => {
				let len = try!(self.read_u16::<BigEndian>()) as usize;
				let mut buf = Vec::with_capacity(len);
				try!(self.read(&mut buf));
				return Ok((Value::Bin(buf), msgpack_id));
			}
			
			MsgPackId::Bin32 => {
				let len = try!(self.read_u32::<BigEndian>()) as usize;
				let mut buf = Vec::with_capacity(len);
				try!(self.read(&mut buf));
				return Ok((Value::Bin(buf), msgpack_id));
			}
			
			MsgPackId::Ext8 => {
				let len = try!(self.read_u8()) as usize;
				let ty = try!(self.read_i8());
				let mut buf = Vec::with_capacity(len);
				try!(self.read(&mut buf));
				return Ok((Value::Ext(ty, buf), msgpack_id));
			}
			
			MsgPackId::Ext16 => {
				let len = try!(self.read_u16::<BigEndian>()) as usize;
				let ty = try!(self.read_i8());
				let mut buf = Vec::with_capacity(len);
				try!(self.read(&mut buf));
				return Ok((Value::Ext(ty, buf), msgpack_id));
			}
			
			MsgPackId::Ext32 => {
				let len = try!(self.read_u32::<BigEndian>()) as usize;
				let ty = try!(self.read_i8());
				let mut  buf = Vec::with_capacity(len);
				try!(self.read(&mut buf));
				return Ok((Value::Ext(ty, buf), msgpack_id));
			}
			
			MsgPackId::Float32 => {
				let value = try!(self.read_f32::<BigEndian>());
				return Ok((Value::Float32(value), msgpack_id));
			}
			
			MsgPackId::Float64 => {
				let value = try!(self.read_f64::<BigEndian>());
				return Ok((Value::Float64(value), msgpack_id));
			}
			
			MsgPackId::UInt8 => {
				let value = try!(self.read_u8());
				return Ok((Value::UInt8(value), msgpack_id));
			}
			
			MsgPackId::UInt16 => {
				let value = try!(self.read_u16::<BigEndian>());
				return Ok((Value::UInt16(value), msgpack_id));
			}
			
			MsgPackId::UInt32 => {
				let value = try!(self.read_u32::<BigEndian>());
				return Ok((Value::UInt32(value), msgpack_id));
			}
			
			MsgPackId::UInt64 => {
				let value = try!(self.read_u64::<BigEndian>());
				return Ok((Value::UInt64(value), msgpack_id));
			}
			
			MsgPackId::Int8 => {
				let value = try!(self.read_i8());
				return Ok((Value::Int8(value), msgpack_id));
			}
			
			MsgPackId::Int16 => {
				let value = try!(self.read_i16::<BigEndian>());
				return Ok((Value::Int16(value), msgpack_id));
			}
			
			MsgPackId::Int32 => {
				let value = try!(self.read_i32::<BigEndian>());
				return Ok((Value::Int32(value), msgpack_id));
			}
			
			MsgPackId::Int64 => {
				let value = try!(self.read_i64::<BigEndian>());
				return Ok((Value::Int64(value), msgpack_id));
			}
			
			//0xd4 fixext 1
			MsgPackId::FixExt1 => {
				let ty = try!(self.read_i8());
				let mut buf : [u8;1] = [0];
				try!(self.read(&mut buf));
				return Ok((Value::Ext1(ty, buf[0]), msgpack_id));
			}
			
			//0xd5 fixext 2
			MsgPackId::FixExt2 => {
				let ty = try!(self.read_i8());
				let mut buf : [u8;2] = [0;2];
				try!(self.read(&mut buf));
				return Ok((Value::Ext2(ty, buf), msgpack_id));
			}
			
			//0xd6 fixext 4
			MsgPackId::FixExt4 => {
				let ty = try!(self.read_i8());
				let mut buf : [u8;4] = [0;4];
				try!(self.read(&mut buf));
				return Ok((Value::Ext4(ty, buf), msgpack_id));
			}
			
			//0xd7 fixext 8
			MsgPackId::FixExt8 => {
				let ty = try!(self.read_i8());
				let mut buf : [u8;8] = [0;8];
				try!(self.read(&mut buf));
				return Ok((Value::Ext8(ty, buf), msgpack_id));
			}
			
			//0xd8 fixext 16
			MsgPackId::FixExt16 => {
				let ty = try!(self.read_i8());
				let mut buf : [u8;16] = [0;16];
				try!(self.read(&mut buf));
				return Ok((Value::Ext16(ty, buf), msgpack_id));
			}
			
			//0xd9 str 8
			MsgPackId::Str8 => {
				let len = try!(self.read_u8()) as usize;
				let mut buf = Vec::with_capacity(len);
				try!(self.read(&mut buf));
				
				if let Ok(str) = String::from_utf8(buf) {
					return Ok((Value::Str(str), msgpack_id));
				}
				else {
					return Err(Error::new(ErrorKind::Other, "Not a valid utf8 string"));
				}
			}
			
			//0xda str 16
			MsgPackId::Str16 => {
				let len = try!(self.read_u16::<BigEndian>()) as usize;
				let mut buf = Vec::with_capacity(len);
				try!(self.read(&mut buf));
				
				if let Ok(str) = String::from_utf8(buf) {
					return Ok((Value::Str(str), msgpack_id));
				}
				else {
					return Err(Error::new(ErrorKind::Other, "Not a valid utf8 string"));
				}
			}
			
			//0xdb str 32
			MsgPackId::Str32 => {
				let len = try!(self.read_u32::<BigEndian>()) as usize;
				let mut buf = Vec::with_capacity(len);
				try!(self.read(&mut buf));
				
				if let Ok(str) = String::from_utf8(buf) {
					return Ok((Value::Str(str), msgpack_id));
				}
				else {
					return Err(Error::new(ErrorKind::Other, "Not a valid utf8 string"));
				}
			}
			
			/*
			//0xdc array 16
			MsgPackId::Array16 => {
				let length = try!(self.read_u16::<BigEndian>());
				return Ok(Value::ArrayStart(length as usize));
			}
			
			//0xdd array 32
			MsgPackId::Array32 => {
				let length = try!(self.read_u32::<BigEndian>());
				return Ok(Value::ArrayStart(length as usize));
			}
			
			//0xde -> map 16
			MsgPackId::Map16 => {
				let length = try!(self.read_u16::<BigEndian>());
				return Ok(Value::MapStart(length as usize));
			}
			
			//0xdf -> map 32
			MsgPackId::Map32 => {
				let length = try!(self.read_u32::<BigEndian>());
				return Ok(Value::MapStart(length as usize));
			}
			*/
			
			_ => { return Err(Error::new(ErrorKind::Other, "Not implemented")); }
		}
	}
	
}

impl<T: Read> MsgPackReader for T {}

/*
trait MsgPackReaderExt : MsgPackReader
{
	fn read_u8(&mut self) -> Result<u8>
	{
		let v = self.read();
		if v.is_err() {
			return Err(v.err().unwrap());
		}
		
		if let Value::UInt8(x) = v.unwrap() {
			return Ok(x);
		}
		
		return Err(Error::new(ErrorKind::Other, "Wrong datatype", None));
	}
}
*/
