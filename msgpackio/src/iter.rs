
use super::value::{Value};
use super::reader::{MsgPackReader};

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

//bytes readed

pub struct MsgPackIterator<'a, T: 'a>
	where T: Read
{
	pub reader: &'a mut T,
	
	//state stack for map and array
		//elements left?
	
	//kindof -> key or value

	//depth -> stack count 
	
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
	type Item = Value; //better wrapper?
						//array, map, start value?, end value?
						//sax style api?
	
	fn next(&mut self) -> Option<<Self as Iterator>::Item>
	{
		//let reader : &mut MsgPackReader = reader;
		let r = self.reader.read_value();
		
		if r.is_ok() {
			let value = r.ok().unwrap();
			
			/*match(value)
			{
				
			}*/
			
			
			Some(value)
		}
		else {
			None
		}
	}
}



