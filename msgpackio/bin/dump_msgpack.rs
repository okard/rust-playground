

extern crate msgpackio;

extern crate argparse;
use argparse::{ArgumentParser, StoreTrue, Store};


use std::fs::{File};


fn main()
{
	let mut file_path = String::new();
    {  // this block limits scope of borrows by ap.refer() method
        let mut ap = ArgumentParser::new();
        ap.set_description("MsgPack Dump Utility");
        ap.refer(&mut file_path).add_argument("file_path", Store, "MsgPack File");
        ap.parse_args_or_exit();
    }
    
    if file_path.is_empty() {
		println!("No filename given");
		return;
	}
	
	let mut f = File::open(file_path).unwrap();
	let mut mpi = msgpackio::read::MsgPackIterator { reader: &mut f};
	for v in mpi
	{
		println!("{:?}", v);
	}
}
