

extern crate msgpackio;


use std::fs::{File};


fn main()
{
	//write testfile?
	
	let mut f = File::open("test.msgpack").unwrap();
	
	let mut mpi = msgpackio::MsgPackIterator { reader: &mut f};
	for v in mpi
	{
		println!("{:?}", v);
	}
}
