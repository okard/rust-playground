
extern crate msgpackio;

use std::io::{Cursor};

use msgpackio::writer::{MsgPackWriter};
use msgpackio::reader::{MsgPackReader};
use msgpackio::ext;
use msgpackio::value::{Value};


macro_rules! test_read {
    ($a:pat, $b:expr, $c:expr) => 
    {
        {
			
			$b.read_msgpack_value(|_, v| {
				if let &$a = v {
					$c;
				}
				else {
					assert!(false, "The returned value has not the correct type");
				}
			})
			
        }
    };
}

#[test]
fn msgpack_nil_wr() 
{
	let mut buf : Vec<u8> = Vec::new();
	assert_eq!(buf.write_msgpack_nil().unwrap(), 1);
	
	let mut buf_reader = Cursor::new(buf);
	test_read!(Value::Nil, buf_reader, ());
}

#[test]
fn msgpack_bool_wr() 
{
	let mut buf : Vec<u8> = Vec::new();
	assert_eq!(buf.write_msgpack_bool(true).unwrap(), 1);
	assert_eq!(buf.write_msgpack_bool(false).unwrap(), 1);
	
	let mut buf_reader = Cursor::new(buf);
	
	test_read!(Value::Boolean(x), buf_reader, assert_eq!(x, true));
	test_read!(Value::Boolean(x), buf_reader, assert_eq!(x, false));
}

#[test]
fn msgpack_fixint_wr() 
{
	let mut buf : Vec<u8> = Vec::new();
	
	assert_eq!(buf.write_msgpack_pos_fixint(120).unwrap(), 1);
	assert_eq!(buf.write_msgpack_neg_fixint(-12).unwrap(), 1);
	
	let mut buf_reader = Cursor::new(buf);
	
	test_read!(Value::UInt8(x), buf_reader, assert_eq!(x, 120));
	test_read!(Value::Int8(x), buf_reader, assert_eq!(x, -12));
}


/*
#[test]
fn msgpack_map_wr() 
{
	let mut buf : Vec<u8> = Vec::new();
	
	//fixmap
	//map16
	//map32
	assert_eq!(buf.write_msgpack_map_start(10).unwrap(), 1);
	assert_eq!(buf.write_msgpack_map_start(30000).unwrap(), 3);
	assert_eq!(buf.write_msgpack_map_start(4294967000).unwrap(), 5);
	
	let mut buf_reader = Cursor::new(buf);
	
	test_read!(Value::MapStart(x), buf_reader, assert_eq!(x, 10));
	test_read!(Value::MapStart(x), buf_reader, assert_eq!(x, 30000));
	test_read!(Value::MapStart(x), buf_reader, assert_eq!(x, 4294967000));
}
*/

/*
#[test]
fn msgpack_bin_wr() 
{
	let mut buf : Vec<u8> = Vec::new();
	
	//test bin8 with data
	let test_string = "teststring";
	assert_eq!(buf.write_msgpack_bin(test_string.as_bytes()).unwrap(), 2+test_string.len());
	
	let mut buf_reader = Cursor::new(buf);
	let mut length : usize = 0;
	test_read!(Value::BinStart(x), buf_reader, {length = x; assert_eq!(x, test_string.len())});
	
	let mut buf : Vec<u8> = Vec::new();
	assert_eq!(length, ext::read_to_vec(&mut buf_reader, length, &mut buf).unwrap());
	assert_eq!(buf, test_string.as_bytes());
	
	//test only header?
	//bin16
	//bin32
}
*/

/*
#[test]
fn msgpack_str_wr() 
{
	let mut buf : Vec<u8> = Vec::new();
	
	//test str with data
	let test_string = "teststring"; //fixstring size
	assert_eq!(buf.write_msgpack_str(test_string).unwrap(), 1+test_string.len());
	
	let mut buf_reader = Cursor::new(buf);
	let mut length : usize = 0;
	test_read!(Value::StrStart(x), buf_reader, {length = x; assert_eq!(x, test_string.len())});
	
	let mut buf : Vec<u8> = Vec::new();
	assert_eq!(length, ext::read_to_vec(&mut buf_reader, length, &mut buf).unwrap());
	assert_eq!(buf, test_string.as_bytes());
	
	//test only header?
	//bin16
	//bin32
}
*/




