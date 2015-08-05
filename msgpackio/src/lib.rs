
//msgpackio lib

//#![feature(io)]
//#![feature(core)]
#![feature(convert)]

#![allow(dead_code)]

extern crate byteorder;

pub mod ext;
pub mod id;

pub mod read {
	pub mod value;
	pub mod reader; //low level reader
	pub mod iter;
	pub mod map;
	
	pub use self::value::{Value};
	pub use self::reader::{MsgPackReader};
	pub use self::iter::{MsgPackIterator};
}

pub mod write {
	pub mod writer; //low level writer
	pub use self::writer::{MsgPackWriter};
}








