
#![allow(dead_code)]

#![feature(path_ext)]
#![feature(convert)]
//#![feature(io)]

#[macro_use]
extern crate bitflags;

extern crate crypto;

#[macro_use]
extern crate log;

//interface traits
mod core; 
mod extended;
mod util;

//feature flags which backend to include

//filesystem storage
#[cfg(feature = "storage_filesystem")]
mod storage_filesystem;

//in memory storage
mod storage_memory;

//TODO hybrid systems? mem + tmp file

//TODO storage_rocksdb
//TODO possible? LMDB http://symas.com/mdb/

//public use exports:
pub use core::ReadHandle;
pub use core::WriteHandle;
pub use core::KeyValueStorage;
pub use storage_filesystem::FilesystemStorage;
pub use storage_memory::MemoryStorage;
