
//use id::{MsgPackId};
use read::value::{Value};
use read::reader::{MsgPackReader};

use std::io::{Read};
//use std::iter::{IntoIterator};


/*
enum MsgPackIteratorState
{
	Free,
	Map(usize),
	Array(usize)
}
* */
/*
enum MsgPackValueHandle
{
	Int...
}
*/

//bytes readed

pub struct MsgPackIterator<'a, T: 'a>
	where T: Read
{
	pub reader: &'a mut T,
	
	//state stack for map and array
		//elements left?
	
	//kindof -> key or value

	//depth -> stack count 
	
	//provide a &mut 'a Handle Function?
	//internal Handle stack state
	//so values can be skipped and reader doesnt get stuck on an invalid point
		//is error then
	
	//different error strategy?
		//error state -> data is corrupted doesnt make sense to find anything
		//search for next header (maybe unsafe)
		//report position in datastream?
}

/*
impl<'a, T> MsgPackIterator<'a, T>
	where T: Read
{
	pub fn new<'b, S>(reader: &'b mut S) ->  MsgPackIterator<'b, S>
		where S : Read
	{
		MsgPackIterator {  reader: reader }
	}
}*/

//read trait


impl<'a, T> Iterator for MsgPackIterator<'a, T>
	where T: Read
{
	type Item = Value;
	
	fn next(&mut self) -> Option<<Self as Iterator>::Item>
	{
		if let Ok((v,_)) = self.reader.read_msgpack_value() {
			Some(v)
		}
		else {
			None
		}
	}
}




