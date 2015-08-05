
//Memory mapping types (into enum?)

//zero copy api?

use id::{MsgPackId};

//put this into enum?

struct Int8Map<'a>(& 'a [u8]);

impl<'a> Int8Map<'a>
{
	pub fn new<'b>(slice: & 'b [u8]) -> Option<Int8Map<'b>>
	{
		if slice.len() != 2 
		|| Some(MsgPackId::Int8) != MsgPackId::from_u8(slice[0])
		{
			return None;
		}
		
		//check slice
		Some(Int8Map(slice))
	}
	
	#[inline]
	pub fn get_type(&self) -> MsgPackId
	{
		let &Int8Map(slice) = self;
		let id = MsgPackId::from_u8(slice[0]).unwrap();
		//assert_eq!(id.as_u8(), MsgPackId::Int8.as_u8());
		id
	}
	
	#[inline]
	pub fn get_u8(&self) -> u8
	{
		let &Int8Map(slice) = self;
		assert!(slice.len() >= 2);
		//assert_eq!(slice[0], MsgPackId::Int8.as_u8());
		slice[1]
	}
}


//mapper 
	//take a &[u8]
	//add a position
		//can actual type
		//can deliver type
			//get_uint8() -> Int8Map<'a> 
	//can move
	
	//works with buffers? 
