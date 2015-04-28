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
extern crate linenoise;
extern crate argparse;
use argparse::{ArgumentParser, Store};

extern crate storage;
use storage::{KeyValueStorage, FilesystemStorage};

use std::io::{Read, Cursor}; //ErrorKind, Error, Result
use std::fs::File;

struct ReadHandle<'a>
{
	reader: Box<Read + 'a>,
	size: usize
}

fn create_readhandle<'a>(input: &'a str) -> ReadHandle<'a> //Result<ReadHandle<'a>>
{
	if input.starts_with("hex:") {
		panic!();
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
		panic!();
	}
	else {
		ReadHandle {
			reader: Box::new(Cursor::new(input.as_bytes())) as Box<Read>,
			size: input.len()
		}
	}
}



//handle line
fn handle_line(storage: &mut KeyValueStorage, line: &str)
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
				println!("{}", r.err().unwrap());
		}
	}
	
	//handle commands
	match command.as_str() {
		"get" => 
		{
			//macro? create read + size from value or key looking at prefixed values
			
			let mut key_handle = create_readhandle(&key);
			let r = storage.get(&mut key_handle.reader, key_handle.size, &mut stdout);
			if r.is_err() {
				println!("{}", r.err().unwrap());
			}
		}
		"put" => 
		{
			let mut key_handle = create_readhandle(&key);
			let mut value_handle =  create_readhandle(&value);
				
			let r = storage.put(&mut key_handle.reader, key_handle.size, &mut value_handle.reader, value_handle.size);
			if r.is_err() {
				println!("{}", r.err().unwrap());
			}
		}
		_ => { println!("Wrong command: {}", command.as_str()); }
	}
}

fn main() 
{
	let mut repo_type = "".to_string();
	let mut repo_path = "".to_string();
	{
		let mut ap = ArgumentParser::new();
		ap.set_description("Storage Console");
		ap.refer(&mut repo_type).add_argument("repo_type", Store, "Repository Type");
		ap.refer(&mut repo_path).add_argument("repo_path", Store, "Repository Path");
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
		_ => { println!("Invalid storage type"); return; }
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
				handle_line(&mut *storage, input.as_str());
			}
		}
	}
}
