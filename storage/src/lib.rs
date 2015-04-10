
#![allow(dead_code)]

#![feature(path_ext)]
#![feature(convert)]
//#![feature(io)]

extern crate crypto;

//interface traits
mod core; 
mod extended;

//feature flags which backend to include

//filesystem storage
mod storage_filesystem;

//in memory storage
mod storage_memory;

//TODO hybrid systems? mem + tmp file

//TODO storage_rocksdb
//TODO possible? LMDB http://symas.com/mdb/

pub use core::KeyValueStorage;
pub use storage_filesystem::FilesystemStorage;
