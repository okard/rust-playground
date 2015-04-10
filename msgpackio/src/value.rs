
/**
* Basic MsgPack Values for stateless reading
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
	
	ArrayStart(usize), 	// count of elements
	MapStart(usize),	// count of element pairs
	
	BinStart(usize),	//the size of the following data
	ExtStart(usize, i8),	//the size of the following data
	
	StrStart(usize),	//the size of following string data
	
	//fixed size extensions
	Ext1(i8, u8),
	Ext2(i8, [u8; 2]),
	Ext4(i8, [u8; 4]),
	Ext8(i8, [u8; 8]),
	Ext16(i8, [u8; 16]),
}
