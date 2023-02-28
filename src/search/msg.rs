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
	key::CollectionKeychain,
};

//---------------------------------------------------------------------------------------------------- Kernel Messages.
pub enum SearchToKernel {
	SearchResult(CollectionKeychain), // Here's the search result.
}

pub enum KernelToSearch {
	Search(String),                 // Start a search on string input.
	DropCollection,                 // Drop your pointer.
	CollectionArc(Arc<Collection>), // Here's a new `Collection` pointer.
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn __TEST__() {
//  }
//}