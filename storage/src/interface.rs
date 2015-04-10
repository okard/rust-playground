
use std::io::{Result, Read, Write};


//use sha256 hashs of value as key
pub type HASH_SHA256 = [u8; 32];

//Blake2B hash

//use uuid as key
pub type UUID16 = [u8; 16];
pub type UUID32 = [u8; 32];



// type of repository (key/value storage)
	// - hash based (hash of value) 
		// objects can't change
	// - uuid based (generate uuid for each object)
	// - key/value free (choose key and value)
	// Mixed variants?
	
	// stacking storage for repository?
		//use different styles at the same time for one repository
		//use different repositories at the same time of node sharing?
		
	//repository -> different storages
		//e.g. uuid + hash 

//trait for key


	//put freely in with a generated key received through result
	//fn put_free(&mut self, value_reader:&mut Read, value_size: usize) -> Result<Vec<u8>>;

//search trait
	//byte/offset based search? (with some regex?)
	//search parameter enum/struct

//hash trait -> deliver hash of content for compare (same key different value)

//type RepositorySHA256 = Repository<u8>;




