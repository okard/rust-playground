


extern crate storage;
use storage::{KeyValueStorage, MemoryStorage, ReadHandle, WriteHandle};


#[test]
fn simple_mem_storage()
{
	let mut storage = MemoryStorage::new();
	let mut storage : &mut KeyValueStorage = &mut storage;

	//insert via slice
	let mut key = &mut ReadHandle::Slice("test".as_bytes());
	let r = storage.put(&mut key, &mut ReadHandle::Slice("hello world".as_bytes()));
	assert!(r.is_ok());

	//read via Write trait
	let mut value : Vec<u8> = Vec::new();
	let r =  storage.get(&mut key, &mut WriteHandle::Writer(&mut value, None));
	assert!(r.is_ok());
	assert_eq!(&value[..], "hello world".as_bytes());

	//read via slice
	let mut value = [0u8; 11];
	let r =  storage.get(&mut key, &mut WriteHandle::Slice(&mut value));
	println!("{:?}", r);
	assert!(r.is_ok());
	assert_eq!(&value, "hello world".as_bytes());
}

#[test]
fn insert_via_reader()
{
	use std::io::{Cursor, Seek, SeekFrom};
	let mut storage = MemoryStorage::new();
	let mut storage : &mut KeyValueStorage = &mut storage;

	//insert via reader without size limit
	let mut key_source = &mut Cursor::new("test".as_bytes());
	let mut value_source = &mut Cursor::new("hello world".as_bytes());
	{
		let mut key = &mut ReadHandle::Reader(&mut key_source, None);
		let mut value = &mut ReadHandle::Reader(&mut value_source, None);
		let r = storage.put(&mut key, &mut value);
		assert!(r.is_ok());
	}

	//check written value
	let r = key_source.seek(SeekFrom::Start(0)); //reset cursor
	assert!(r.is_ok() && r.unwrap() == 0);
	{
		let mut value : Vec<u8> = Vec::new();
		let mut key = &mut ReadHandle::Reader(&mut key_source, None);
		let r =  storage.get(&mut key, &mut WriteHandle::Writer(&mut value, None));
		assert!(r.is_ok());
		assert_eq!(&value[..], "hello world".as_bytes());
	}
}
