/*

open storage
	memory
	filesystem
	rocksdb
starts console

Options for <key>/<value>:
	asdads / str:asdasd
	hex:asdasd
	base64:asdasd
	base64url:asdasd 
	file:asdasd
	
Commands:
	get <key> --out --info_only
	put <key> <value> --flags
	delete
	meta
	set_flags
	do_file -> do file with command list
*/
#![feature(convert)] 
#![feature(box_syntax)]

extern crate log;
extern crate linenoise;
extern crate argparse;
extern crate rustc_serialize;
extern crate storage;

use std::io::{Write, Read, Cursor, Result, Error, ErrorKind}; //ErrorKind, Error, Result
use std::fs::File;

use argparse::{ArgumentParser, Store};
use log::{LogRecord, LogLevel, LogMetadata, LogLevelFilter};
use rustc_serialize::base64::{FromBase64};
use rustc_serialize::hex::{FromHex};

use storage::{KeyValueStorage, FilesystemStorage, MemoryStorage};

///
/// Storage console readhandle
///
struct ReadHandle<'a>
{
	reader: Box<Read + 'a>,
	size: usize
}

///
/// Write handle
///
struct WriteHandle<'a>
{
	writer: &'a mut Write,
	size: Option<usize>
}

///
/// create a readhandle from a special string
/// 	can read hex, file, base64 values
///
fn create_readhandle<'a>(input: &'a str) -> ReadHandle<'a> //Result<ReadHandle<'a>>
{
	if input.starts_with("hex:") {
		let data = Box::new(Cursor::new(input[4..].from_hex().unwrap()));
		ReadHandle {
			reader: data,
			size: input[4..].len()/2
		}
	}
	else if input.starts_with("file:") {
		let file = Box::new(File::open(&input[5..]).unwrap());
		let len = file.metadata().unwrap().len();
		ReadHandle {
			reader: file as Box<Read>,
			size: len as usize
		}
	}
	else if input.starts_with("base64:") {
		let data = Box::new(Cursor::new(input[7..].from_base64().unwrap()));
		let len = data.get_ref().len();
		ReadHandle {
			reader: data,
			size: len
		}
	}
	//TODO base64url
	else {
		ReadHandle {
			reader: Box::new(Cursor::new(input.as_bytes())) as Box<Read>,
			size: input.len()
		}
	}
}

///
/// handle line
/// 
fn handle_line(storage: &mut KeyValueStorage, line: &str) -> Result<()>
{
	let mut stdout = std::io::stdout(); //put into context struct
	let mut stderr = std::io::stderr();
	
	//get arguments
	let mut args : Vec<String> = Vec::new();
	args.push(String::from("")); //program arg dummy
	for arg in line.split(' ') {
		args.push(String::from(arg));
	}
	
	//parse arguments
	let mut command = "".to_string();
	let mut key = "".to_string();
	let mut value = "".to_string();
	{
		let mut ap = ArgumentParser::new();
		ap.refer(&mut command).add_argument("command", Store, "Command (get, put, delete)");
		ap.refer(&mut key).add_argument("key", Store, "Key of Value");
		ap.refer(&mut value).add_argument("value", Store, "Value Data");
		let r = ap.parse(args, &mut stdout, &mut stderr);
		if r.is_err() {
			return Err(Error::new(ErrorKind::Other, "argument parsing failed"));
		}
	}
	
	//handle commands
	match command.as_str() 
	{
		"get" => 
		{
			//macro? create read + size from value or key looking at prefixed values
			//check for value 
			
			let mut key_handle = create_readhandle(&key);
			let mut out_handle = WriteHandle { writer: &mut stdout, size: None };
			{
				let mut skey_handle = storage::ReadHandle::Reader(&mut key_handle.reader, Some(key_handle.size));
				let mut sout_handle = storage::WriteHandle::Writer(&mut out_handle.writer, out_handle.size);
				try!(storage.get(&mut skey_handle, &mut sout_handle));
			}
			try!(out_handle.writer.write("\n".as_bytes()));
			try!(out_handle.writer.flush());
		}
		"put" => 
		{
			let mut key_handle = create_readhandle(&key);
			let mut value_handle =  create_readhandle(&value);
			let mut skey_handle = storage::ReadHandle::Reader(&mut key_handle.reader, Some(key_handle.size));
			let mut svalue_handle = storage::ReadHandle::Reader(&mut value_handle.reader, Some(value_handle.size));
				
			try!(storage.put(&mut skey_handle, &mut svalue_handle));
		}
		"delete" => 
		{
			let mut key_handle = create_readhandle(&key);
			let mut skey_handle = storage::ReadHandle::Reader(&mut key_handle.reader, Some(key_handle.size));
			try!(storage.delete(&mut skey_handle));
		}
		
		//TODO delete command
		//TODO meta command
		//TODO set_flags command
		//TODO do_file command
		_ => { println!("Wrong command: {}", command.as_str()); }
	}
	
	Ok(())
}

///
/// The main function
///
fn main() 
{
	//setup logger
	let r = log::set_logger(|max_log_level| {
		max_log_level.set(LogLevelFilter::Debug);
		Box::new(StdoutLogger)
	});
	
	if r.is_err() {
		println!("failed to initialize the logger");
		return;
	}
	
	let mut repo_type = "".to_string();
	let mut repo_path = "".to_string();
	{
		let mut ap = ArgumentParser::new();
		ap.set_description("Storage Console");
		ap.refer(&mut repo_type).add_argument("repo_type", Store, "Repository Type");
		ap.refer(&mut repo_path).add_argument("repo_path", Store, "Repository Path");
		//TODO support --dofile here?
		ap.parse_args_or_exit();
	}
	
	let mut storage : Option<Box<KeyValueStorage>> = None;
	
	match repo_type.as_str()
	{
		"fs" => {
			
			if repo_path.is_empty() {
				println!("No repo path is given");
				return;
			}
			storage = Some(Box::new(FilesystemStorage::open(repo_path.as_str()).unwrap()));
			
			
			//create file system
		}
		
		"mem" => {
			storage = Some(Box::new(MemoryStorage::new()));
		}
		
		_ => {}
	}
	if storage.is_none() {
		println!("Invalid storage type"); 
		return;
	}
	
	
	let mut storage = storage.unwrap();
	
	//create storage
	
	loop 
	{
		let val = linenoise::input("> ");
		match val 
		{
			None => { break }
			Some(input) => 
			{
				if input == "exit" 
				|| input == "q" 
				|| input == "quit" {
					break;
				}
				
				linenoise::history_add(input.as_str());
				let r = handle_line(&mut *storage, input.as_str());
				if r.is_err() {
					println!("error: {}", r.err().unwrap());
				}
			}
		}
	}
}

/// phantom struct for implementing a stdout logger
struct StdoutLogger;

impl log::Log for StdoutLogger 
{
	fn enabled(&self, metadata: &LogMetadata) -> bool 
	{
		metadata.level() <= LogLevel::Debug
	}

	fn log(&self, record: &LogRecord) 
	{
		if self.enabled(record.metadata()) {
			println!("{}", record.args());
		}
	}
}
