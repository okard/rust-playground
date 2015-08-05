
use id::{MsgPackId};

/**
* Basic MsgPack Values for stateless reading
* TODO rename to ValueHeader?
* TODO internal only?
*/
#[derive(Debug)]
pub enum Value
{
	Nil,
	Boolean(bool),
	
	UInt8(u8),
	UInt16(u16),
	UInt32(u32),
	UInt64(u64),
	
	Int8(i8),
	Int16(i16),
	Int32(i32),
	Int64(i64),
	
	Float32(f32),
	Float64(f64),
	
	Array(MsgPackId, Vec<Value>), 
	Map(MsgPackId, Vec<(Value,Value)>),
	
	Bin(MsgPackId, Vec<u8>),
	Str(MsgPackId, String),
	
	//fixed size extensions
	Ext1(i8, u8),
	Ext2(i8, [u8; 2]),
	Ext4(i8, [u8; 4]),
	Ext8(i8, [u8; 8]),
	Ext16(i8, [u8; 16]),
	Ext(MsgPackId, i8, Vec<u8>),
}

impl Value
{
	fn get_id(&self) -> MsgPackId
	{
		match(self)
		{
			&Value::Nil => MsgPackId::Nil,
			&Value::Boolean(b) => if b {MsgPackId::True}else{MsgPackId::False},
	
			&Value::UInt8(_) => MsgPackId::UInt8,
			&Value::UInt16(_) => MsgPackId::UInt16,
			&Value::UInt32(_) => MsgPackId::UInt32,
			&Value::UInt64(_) => MsgPackId::UInt64,
			
			&Value::Int8(_) => MsgPackId::Int8,
			&Value::Int16(_) => MsgPackId::Int16,
			&Value::Int32(_) => MsgPackId::Int32,
			&Value::Int64(_) => MsgPackId::Int64,
			
			&Value::Float32(_) => MsgPackId::Float32,
			&Value::Float64(_) => MsgPackId::Float64,
			
			&Value::Array(id, _) => id, 
			&Value::Map(id, _) => id,
			
			&Value::Bin(id, _) => id,
			&Value::Str(id, _) => id,
			
			//fixed size extensions
			&Value::Ext1(_, _) => MsgPackId::FixExt1,
			&Value::Ext2(_, _) => MsgPackId::FixExt2,
			&Value::Ext4(_, _) => MsgPackId::FixExt4,
			&Value::Ext8(_, _) => MsgPackId::FixExt8,
			&Value::Ext16(_, _) => MsgPackId::FixExt16,
			&Value::Ext(id, _, _) => id,
			
		}
	}
}

//TODO implement fn get_id() -> MsgPackId for Value



/*
* Represents access to raw data?
* better a struct? track readed data?
* methods with to vec? not required?
*/
/*
pub enum DataHandle<'a> {
	
	None,
	//Raw Data
	Buf(Vec<u8>),
	Slice(&'a [u8]),
	Reader(&'a mut Read),
}

enum RefValue<'a>
{
	//deliver &'a [u8]?
	//deliver &'a Read?
}*/


