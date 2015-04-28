
use super::core::{KeyValueStorage};

use std::io::{Result, Read};

pub trait ExtendedStorage : KeyValueStorage
{
	//put only value and get generated key
	//support Content-addressable storage(CAS)/associative storage
	//-> readonly flag
	
	//verify data (check)
	fn verify(&self, k: &mut Read) -> Result<bool>;
	
	//deliver meta informationen trait
	fn get_meta_info(&self, key_reader : &mut Read) -> Result<MetaInformation>;
	
	//toggle readonly flag
}


// MetaInformation for a key/value storage
	// not all storage systems have to implement this
trait MetaInformation
{
	fn get_data_hash(&self) -> Vec<u8>;
	//get data hash
	//get key hash
	//get complete hash (meta+key+value)
	//get last update timestamp
	//get key size
	//get data size
	//readonly
}


//seek/iterate interface


