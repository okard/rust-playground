

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MsgPackId
{
	FixPosInt = 0, 
	FixMap, 
	FixArray,
	FixStr, 

	Nil,
	//0xc1 never used
	False,
	True,
	Bin8,
	Bin16,
	Bin32,
	Ext8,
	Ext16,
	Ext32,
	Float32,
	Float64,
	UInt8,
	UInt16,
	UInt32,
	UInt64,
	Int8,
	Int16,
	Int32,
	Int64,
	FixExt1,
	FixExt2,
	FixExt4,
	FixExt8,
	FixExt16,
	Str8,
	Str16,
	Str32,
	Array16,
	Array32,
	Map16,
	Map32,
	FixNegInt
}

impl MsgPackId 
{
	#[inline]
	pub fn from_u8(b : u8) -> Option<MsgPackId>
	{
		match b
		{
			0x00...0x7f => Some(MsgPackId::FixPosInt),
			0x80...0x8f => Some(MsgPackId::FixMap),
			0x90...0x9f => Some(MsgPackId::FixArray),
			0xa0...0xbf => Some(MsgPackId::FixStr),
			0xc0 => Some(MsgPackId::Nil),
			//0xc1 never used
			0xc2 => Some(MsgPackId::False),
			0xc3 => Some(MsgPackId::True),
			0xc4 => Some(MsgPackId::Bin8),
			0xc5 => Some(MsgPackId::Bin16),
			0xc6 => Some(MsgPackId::Bin32),
			0xc7 => Some(MsgPackId::Ext8),
			0xc8 => Some(MsgPackId::Ext16),
			0xc9 => Some(MsgPackId::Ext32),
			0xca => Some(MsgPackId::Float32),
			0xcb => Some(MsgPackId::Float64),
			0xcc => Some(MsgPackId::UInt8),
			0xcd => Some(MsgPackId::UInt16),
			0xce => Some(MsgPackId::UInt32),
			0xcf => Some(MsgPackId::UInt64),
			0xd0 => Some(MsgPackId::Int8),
			0xd1 => Some(MsgPackId::Int16),
			0xd2 => Some(MsgPackId::Int32) ,
			0xd3 => Some(MsgPackId::Int64),
			0xd4 => Some(MsgPackId::FixExt1),
			0xd5 => Some(MsgPackId::FixExt2),
			0xd6 => Some(MsgPackId::FixExt4),
			0xd7 => Some(MsgPackId::FixExt8),
			0xd8 => Some(MsgPackId::FixExt16),
			0xd9 => Some(MsgPackId::Str8),
			0xda => Some(MsgPackId::Str16),
			0xdb => Some(MsgPackId::Str32),
			0xdc => Some(MsgPackId::Array16),
			0xdd => Some(MsgPackId::Array32),
			0xde => Some(MsgPackId::Map16),
			0xdf => Some(MsgPackId::Map32),
			0xe0...0xff => Some(MsgPackId::FixNegInt),
			_ => None
		}
	}
}
