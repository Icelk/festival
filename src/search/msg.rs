//---------------------------------------------------------------------------------------------------- Use
//use anyhow::{anyhow,bail,ensure};
//use log::{info,error,warn,trace,debug};
//use serde::{Serialize,Deserialize};
//use crate::macros::*;
//use disk::prelude::*;
//use disk::{};
//use std::{};
use std::sync::Arc;
use crate::collection::{
	Collection,
	Keychain,
};

//---------------------------------------------------------------------------------------------------- Kernel Messages.
pub(crate) enum SearchToKernel {
	// Here's the search (similarity) result.
	SearchSim(Keychain),
}

pub(crate) enum KernelToSearch {
	SearchSim(String),              // Start a (similarity) search on string input.
//	NewCache(String),               // Here's a new `String` key from a recently created `Collection`, add it to your cache.
//	NewCacheVec(Vec<String>),       // Here's a `Vec` of `String` keys, add it to cache
	DropCollection,                 // Drop your pointer.
	NewCollection(Arc<Collection>), // Here's a new `Collection` pointer.
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn __TEST__() {
//  }
//}
