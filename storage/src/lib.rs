
#![allow(dead_code)]

#![feature(path_ext)]
#![feature(convert)]
#![feature(vec_push_all)]
//#![feature(io)]

#[macro_use]
extern crate bitflags;

extern crate crypto;

#[macro_use]
extern crate log;

//main traits
mod core; 
mod util;

//public use exports:
pub use core::ReadHandle;
pub use core::WriteHandle;
pub use core::KeyValueStorage;

////////////////////////////////////////////////////////////////////////
//feature flags which backend to include

//filesystem storage
#[cfg(feature = "storage_filesystem")]
mod storage_filesystem;
pub use storage_filesystem::FilesystemStorage;

//in memory storage
mod storage_memory;
pub use storage_memory::MemoryStorage;

//TODO hybrid systems? mem + tmp file

//TODO storage_rocksdb
//TODO possible? LMDB http://symas.com/mdb/




