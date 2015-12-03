
use std::io;


pub enum StorageError
{
	KeyNotFound, //Key is not available
	CorruptStorage, //database file broken or something like that
	Custom(String), //Custom error string
	KeyIoError(io::Error), //io error when accessing key handle
	ValueIoError(io::Error), //io error when accessing value handle
}
