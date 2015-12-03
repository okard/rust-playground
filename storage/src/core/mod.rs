



mod iohandle; //io handle for key and data exchange
mod base;	//base implementations
mod meta;	//meta data interface
mod seek;	//seek interface

pub use self::iohandle::ReadHandle;
pub use self::iohandle::WriteHandle;
pub use self::base::KeyValueStorage;
pub use self::base::ContentStorage;
//meta
//seek


//TODO maintenance trait with verify,repair functions?



