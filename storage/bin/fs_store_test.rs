
#![feature(convert)]

extern crate storage;
extern crate argparse;

use std::io::{self, Cursor, Write};

use storage::{KeyValueStorage, FilesystemStorage};

use argparse::{ArgumentParser, Store};

fn main()
{
	let mut command = "".to_string();
	let mut repo_path = "".to_string();
	let mut key = "".to_string();
	let mut value = "".to_string();
	          
	{
		let mut ap = ArgumentParser::new();
		ap.set_description("FS Storage Test-Tool");
		ap.refer(&mut repo_path).add_argument("repo", Store, "Repository");
		ap.refer(&mut command).add_argument("command", Store, "Command to run");
		ap.refer(&mut key).add_argument("key", Store, "Key");
		//ap.refer(&mut key).add_option(&["--key", "-k"], Store, "Key");
		ap.refer(&mut value).add_option(&["--value"], Store, "Value");
		//ap.refer(&mut value_file).add_option(&["--value-file", "-i"], Store, "Value-File");
		//ap.refer(&mut out_file).add_option(&["--out", "-o"], Store, "Value-File");
        ap.parse_args_or_exit();
    }
    
    if repo_path.is_empty() {
		println!("No repo path");
		return;
	}
	
	if key.is_empty() {
		println!("No key given");
		return;
	}
	
	println!("Repo-Path: {}", repo_path);
	let mut fsstore = FilesystemStorage::open(repo_path.as_str()).unwrap();
	let mut key_reader = Cursor::new(key.as_bytes());
	
	match command.as_str() 
	{
		"put" => 
		{
			println!("Command: {}", command);
			
			if value.is_empty() {
				println!("No value");
				return;
			}
			
			println!("Value: {}", value);
			let mut value_reader = Cursor::new(value.as_bytes());
			
			let mut key_hnd = storage::ReadHandle::new_with_len(&mut key_reader, key.len());
			let mut value_hnd = storage::ReadHandle::new_with_len(&mut value_reader, value.len());
			
			let result = fsstore.put(&mut key_hnd, &mut value_hnd);
			
			if result.is_err() {
				println!("{}", result.err().unwrap());
			}
		}
		
		"get" => 
		{
			println!("Command: {}", command);
			
			let mut out = io::stdout();
			let mut key_hnd = storage::ReadHandle::new_with_len(&mut key_reader, key.len());
			let mut out_hnd = storage::WriteHandle::new(&mut out);
			let result = fsstore.get(&mut key_hnd, &mut out_hnd);
			
			if result.is_err() {
				println!("{}", result.err().unwrap());
			}
			else { out_hnd.writer.write("\n".as_bytes()); }
		}
		
		_ => {
			println!("Invalid command");
			return;
		}
	}
	
	
}
