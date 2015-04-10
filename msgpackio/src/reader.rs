
use std::io::{Result, Error, ErrorKind, Read};
use byteorder::{ReadBytesExt, BigEndian};

use super::id::{MsgPackId};
use super::value::{Value};


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
	

	
	fn read_value(&mut self) -> Result<Value>
	{
		let mut buf : [u8;1] = [0];
		let r = self.read(&mut buf);
		
		if r.is_err() {
			return Err(r.err().unwrap());
		}
		
		if r.unwrap() == 0 {
			return Err(Error::new(ErrorKind::Other, "No data to read"));
		}
		
		let msgpack_ty = MsgPackId::from_u8(buf[0]);
		
		match buf[0]
		{
			//0xC0 Nil
			0xC0 => { return Ok(Value::Nil); }	
			// 	0xc1 => never used
			//Boolean
			0xC2 => { return Ok(Value::Boolean(false)); }
			0xC3 => { return Ok(Value::Boolean(true)); }
			
			//fixed int positive
			0x00...0x7f => { return Ok(Value::UInt8(buf[0] & 0x7f)); }
			//fixed int negative
			0xe0...0xff => { return Ok(Value::Int8(-((buf[0] & 0x1f) as i8))); } //right?
			
			//0x80 - 0x8f fixmap
			0x80...0x8f => { 
				return Ok(Value::MapStart((buf[0] & 0x0f) as usize));
			}
			
			//0x90 - 0x9f fixarray
			0x90...0x9f => { 
				return Ok(Value::ArrayStart((buf[0] & 0x0f) as usize));
			}
			
			//0xa0 - 0xbf fixstr
			0xa0...0xbf => { 
				return Ok(Value::StrStart((buf[0] & 0x1f) as usize));
			}
			
			//0xc4 bin 8
			0xc4 => {
				let length = try!(self.read_u8());
				return Ok(Value::BinStart(length as usize));
			}
			
			//0xc5 bin 16
			0xc5 => {
				let length = try!(self.read_u16::<BigEndian>());
				return Ok(Value::BinStart(length as usize));
			}
			
			
			//0xc6 bin 32
			0xc6 => {
				let length = try!(self.read_u32::<BigEndian>());
				return Ok(Value::BinStart(length as usize));
			}
			
			//0xc7 ext 8
			0xc7 => {
				let length = try!(self.read_u8());
				let ty = try!(self.read_i8());
				return Ok(Value::ExtStart(length as usize, ty));
			}
			
			//0xc8 ext 16
			0xc8 => {
				let length = try!(self.read_u16::<BigEndian>());
				let ty = try!(self.read_i8());
				return Ok(Value::ExtStart(length as usize, ty));
			}
			
			//0xc9 ext 32
			0xc9 => {
				let length = try!(self.read_u32::<BigEndian>());
				let ty = try!(self.read_i8());
				return Ok(Value::ExtStart(length as usize, ty));
			}
			
			//0xca float 32
			0xca => {
				let value = try!(self.read_f32::<BigEndian>());
				return Ok(Value::Float32(value));
			}
			
			//0xcb float 64
			0xcb => {
				let value = try!(self.read_f64::<BigEndian>());
				return Ok(Value::Float64(value));
			}
			
			//0xcc uint 8
			0xcc => {
				let value = try!(self.read_u8());
				return Ok(Value::UInt8(value));
			}
			
			//0xcd uint 16
			0xcd => {
				let value = try!(self.read_u16::<BigEndian>());
				return Ok(Value::UInt16(value));
			}
			
			//0xce uint 32
			0xce => {
				let value = try!(self.read_u32::<BigEndian>());
				return Ok(Value::UInt32(value));
			}
			
			//0xcf uint 64
			0xcf => {
				let value = try!(self.read_u64::<BigEndian>());
				return Ok(Value::UInt64(value));
			}
			
			//0xd0 int 8
			0xd0 => {
				let value = try!(self.read_i8());
				return Ok(Value::Int8(value));
			}
			
			//0xd1 int 16
			0xd1 => {
				let value = try!(self.read_i16::<BigEndian>());
				return Ok(Value::Int16(value));
			}
			
			//0xd2 int 32
			0xd2 => {
				let value = try!(self.read_i32::<BigEndian>());
				return Ok(Value::Int32(value));
			}
			
			//0xd3 int 64
			0xd3 => {
				let value = try!(self.read_i64::<BigEndian>());
				return Ok(Value::Int64(value));
			}
			
			//0xd4 fixext 1
			0xd4 => {
				let ty = try!(self.read_i8());
				let mut buf : [u8;1] = [0];
				try!(self.read(&mut buf));
				return Ok(Value::Ext1(ty, buf[0]));
			}
			
			//0xd5 fixext 2
			0xd5 => {
				let ty = try!(self.read_i8());
				let mut buf : [u8;2] = [0;2];
				try!(self.read(&mut buf));
				return Ok(Value::Ext2(ty, buf));
			}
			
			//0xd6 fixext 4
			0xd6 => {
				let ty = try!(self.read_i8());
				let mut buf : [u8;4] = [0;4];
				try!(self.read(&mut buf));
				return Ok(Value::Ext4(ty, buf));
			}
			
			//0xd7 fixext 8
			0xd7 => {
				let ty = try!(self.read_i8());
				let mut buf : [u8;8] = [0;8];
				try!(self.read(&mut buf));
				return Ok(Value::Ext8(ty, buf));
			}
			
			//0xd8 fixext 16
			0xd8 => {
				let ty = try!(self.read_i8());
				let mut buf : [u8;16] = [0;16];
				try!(self.read(&mut buf));
				return Ok(Value::Ext16(ty, buf));
			}
			
			//0xd9 str 8
			0xd9 => {
				let length = try!(self.read_u8());
				return Ok(Value::StrStart(length as usize));
			}
			
			//0xda str 16
			0xda => {
				let length = try!(self.read_u16::<BigEndian>());
				return Ok(Value::StrStart(length as usize));
			}
			
			//0xdb str 32
			0xdb => {
				let length = try!(self.read_u32::<BigEndian>());
				return Ok(Value::StrStart(length as usize));
			}
			
			//0xdc array 16
			0xdc => {
				let length = try!(self.read_u16::<BigEndian>());
				return Ok(Value::ArrayStart(length as usize));
			}
			
			//0xdd array 32
			0xdd => {
				let length = try!(self.read_u32::<BigEndian>());
				return Ok(Value::ArrayStart(length as usize));
			}
			
			//0xde -> map 16
			0xde => {
				let length = try!(self.read_u16::<BigEndian>());
				return Ok(Value::MapStart(length as usize));
			}
			
			
			//0xdf -> map 32
			0xdf => {
				let length = try!(self.read_u32::<BigEndian>());
				return Ok(Value::MapStart(length as usize));
			}
			
			
			_ => { return Err(Error::new(ErrorKind::Other, "Invalid data")); }
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
