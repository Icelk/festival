//---------------------------------------------------------------------------------------------------- Use
use bincode::{Encode,Decode};
use serde::{Serialize,Deserialize};
use strum::{
	AsRefStr,
	Display,
	EnumCount,
	EnumIter,
	EnumString,
	EnumVariantNames,
	IntoStaticStr,
};

//---------------------------------------------------------------------------------------------------- Sort Constants
/// [`SearchKind::All`]
pub const ALL:    &str = "View all the results, sorted from most similar to least";
/// [`SearchKind::Sim70`]
pub const SIM_70: &str = "View only the results that are at least 70% similar";
/// [`SearchKind::Top25`]
pub const TOP_25: &str = "View only the top 25 similar results";
/// [`SearchKind::Top1`]
pub const TOP_1: &str = "View only the top 1 similar results";

//---------------------------------------------------------------------------------------------------- SearchKind
#[derive(Copy,Clone,Debug,Default,Hash,PartialEq,Eq,PartialOrd,Ord,Serialize,Deserialize,Encode,Decode)]
#[derive(AsRefStr,Display,EnumCount,EnumIter,EnumString,EnumVariantNames,IntoStaticStr)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
/// The different kinds of searches you can request from `Kernel`
pub enum SearchKind {
	/// String similarity, returns all calculated keys
	/// in order from most similar to least.
	All,
	#[default]
	/// [`Self::All`], but only returns the results that are at least 70% similar
	Sim70,
	/// [`Self::All`], but only returns the top 25 results
	Top25,
	/// [`Self::All`], but only returns the top 1 results
	Top1,
}

impl SearchKind {
	#[inline]
	/// Returns formatted, human readable versions.
	pub const fn human(&self) -> &'static str {
		match self {
			Self::Sim70 => SIM_70,
			Self::Top25 => TOP_25,
			Self::Top1  => TOP_1,
			Self::All   => ALL,
		}
	}

	/// Returns the next sequential [`Self`] variant.
	///
	/// This returns the _first_ if at the _last_.
	pub fn next(&self) -> Self {
		match self {
			Self::All   => Self::Sim70,
			Self::Sim70 => Self::Top25,
			Self::Top25 => Self::Top1,
			Self::Top1  => Self::All,
		}
	}

	/// Returns the previous sequential [`Self`] variant.
	///
	/// This returns the _last_ if at the _first_.
	pub fn previous(&self) -> Self {
		match self {
			Self::All   => Self::Top1,
			Self::Sim70 => Self::All,
			Self::Top25 => Self::Sim70,
			Self::Top1  => Self::Top25,
		}
	}
}

//---------------------------------------------------------------------------------------------------- TESTS
#[cfg(test)]
mod tests {
	use super::*;
	use strum::*;

	#[test]
	// Asserts each variant:
	// 1. Gives a different string
	// 2. `.next()` gives a different variant
	// 3. `.prev()` gives a different variant
	fn diff() {
		let mut set1 = std::collections::HashSet::new();
		let mut set2 = std::collections::HashSet::new();
		let mut set3 = std::collections::HashSet::new();

		for i in SearchKind::iter() {
            assert!(set1.insert(i.human()));
            assert!(set2.insert(i.next()));
            assert!(set3.insert(i.previous()));
		}
	}
}
