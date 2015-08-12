
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
	
	Array(Vec<Value>), 
	Map(Vec<(Value,Value)>),
	
	Bin(Vec<u8>),
	Str(String),
	
	//fixed size extensions
	Ext1(i8, u8),
	Ext2(i8, [u8; 2]),
	Ext4(i8, [u8; 4]),
	Ext8(i8, [u8; 8]),
	Ext16(i8, [u8; 16]),
	Ext(i8, Vec<u8>),
}



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


