//---------------------------------------------------------------------------------------------------- Use
//use anyhow::{bail,ensure,Error};
use log::{info,error,warn,trace,debug};
use serde::{Serialize,Deserialize};
use super::{
	album::Album,
	artist::Artist,
	song::Song,
	plural::{Artists,Albums,Songs},
};
use crate::key::{
	Key,
	ArtistKey,
	AlbumKey,
	SongKey,
};
use crate::sort::{
	ArtistSort,
	AlbumSort,
	SongSort,
};
use std::collections::HashMap;
use disk::prelude::*;
use disk::{Bincode,bincode_file};
use crate::constants::{
	FESTIVAL,
	FESTIVAL_HEADER,
	COLLECTION_VERSION,
};

//---------------------------------------------------------------------------------------------------- The Collection™
bincode_file!(Collection, Dir::Data, FESTIVAL, "", "collection", FESTIVAL_HEADER, COLLECTION_VERSION);
#[derive(Debug,Serialize,Deserialize)]
/// The main music `Collection`
///
/// This is the `struct` that holds all the (meta)data about the user's music.
///
/// This holds:
/// - The "3 Vecs", holding _all_ [`Artist`]'s, [`Album`]'s, and [`Song`]'s.
/// - Pre-computed, sorted keys
/// - Metadata about the [`Collection`] itself
///
/// ### Sort
/// The "3 Vecs" are (basically) in random order due to how `Collection` is created.
///
/// Iterating directly on them is not very useful, so use the pre-calculated sorted keys.
///
/// The sorted key fields all start with `sort_`.
///
/// `lexi` is shorthand for `lexicographically`, as defined [here.](https://doc.rust-lang.org/stable/std/primitive.str.html#impl-Ord-for-str)
///
/// ### Index
/// To properly index the [`Collection`], for example, an [`Album`], you CAN use the `[]` operators, however,
/// they must be type-safe. Meaning: it CANNOT be a random [`usize`], it must be the proper type of [`Key`].
///
/// Example:
/// ```rust
/// let my_usize = 0;
/// let key = AlbumKey::from(my_usize);
///
/// // NOT type-safe, compile error!.
/// collection.albums[my_usize];
///
/// // Type-safe, compiles.
/// collection.albums[key];
/// ```
pub struct Collection {
	/// All the [`Artist`]'s in mostly random order.
	pub artists: Artists,
	/// All the [`Album`]'s in mostly random order.
	pub albums: Albums,
	/// All the [`Song`]'s in mostly random order.
	pub songs: Songs,

	// Sorted `Artist` keys.
	/// [`Artist`] `lexi`.
	pub sort_artist_lexi: Vec<ArtistKey>,
	/// [`Artist`] with most [`Album`]'s to least.
	pub sort_artist_album_count: Vec<ArtistKey>,
	/// [`Artist`] with most [`Song`]'s to least.
	pub sort_artist_song_count: Vec<ArtistKey>,

	// Sorted `Album` keys.
	/// [`Artist`] `lexi`, [`Album`]'s oldest release to latest.
	pub sort_album_release_artist_lexi: Vec<AlbumKey>,
	/// [`Artist`] `lexi`, [`Album`]'s `lexi`.
	pub sort_album_lexi_artist_lexi: Vec<AlbumKey>,
	/// [`Album`] lexi.
	pub sort_album_lexi: Vec<AlbumKey>,
	/// [`Album`] oldest to latest.
	pub sort_album_release: Vec<AlbumKey>,
	/// [`Album`] shortest to longest.
	pub sort_album_runtime: Vec<AlbumKey>,

	// Sorted `Song` keys.
	/// [`Artist`] lexi, [`Album`] release, [`Song`] track_number
	pub sort_song_album_release_artist_lexi: Vec<SongKey>,
	/// [`Artist`] lexi, [`Album`] lexi, [`Song`] track_number.
	pub sort_song_album_lexi_artist_lexi: Vec<SongKey>,
	/// [`Song`] lexi.
	pub sort_song_lexi: Vec<SongKey>,
	/// [`Song`] oldest to latest.
	pub sort_song_release: Vec<SongKey>,
	/// [`Song`] shortest to longest.
	pub sort_song_runtime: Vec<SongKey>,

	// Metadata about the `Collection` itself.
	/// Is this [`Collection`] empty?
	///
	/// Meaning, are there absolutely no [`Artist`]'s, [`Album`]'s and [`Song`]'s?
	pub empty: bool,
	/// UNIX timestamp of the [`Collection`]'s creation date.
	pub timestamp: u64,
	/// How many [`Artist`]'s in this [`Collection`]?
	pub count_artist: usize,
	/// How many [`Album`]'s in this [`Collection`]?
	pub count_album: usize,
	/// How many [`Song`]'s in this [`Collection`]?
	pub count_song: usize,
}

impl Collection {
	//-------------------------------------------------- New.
	#[inline(always)]
	/// Creates an empty [`Collection`].
	///
	/// All [`Vec`]'s are empty.
	///
	/// The `timestamp` and `count_*` fields are set to `0`.
	///
	/// `empty` is set to `true`.
	pub const fn new() -> Self {
		Self {
			artists: Artists::new(),
			albums: Albums::new(),
			songs: Songs::new(),

			sort_artist_lexi: vec![],
			sort_artist_album_count: vec![],
			sort_artist_song_count: vec![],

			sort_album_release_artist_lexi: vec![],
			sort_album_lexi_artist_lexi: vec![],
			sort_album_lexi: vec![],
			sort_album_release: vec![],
			sort_album_runtime: vec![],

			sort_song_album_release_artist_lexi: vec![],
			sort_song_album_lexi_artist_lexi: vec![],
			sort_song_lexi: vec![],
			sort_song_release: vec![],
			sort_song_runtime: vec![],

			empty: true,
			timestamp: 0,
			count_artist: 0,
			count_album: 0,
			count_song: 0,
		}
	}

	//-------------------------------------------------- Misc functions.
	// Get current timestamp as UNIX time.
	pub(crate) fn timestamp_now() -> u64 {
		let now = std::time::SystemTime::now();
		match now.duration_since(std::time::SystemTime::UNIX_EPOCH) {
			Ok(ts) => ts.as_secs(),
			Err(e) => {
				warn!("Failed to get timestamp, returning UNIX_EPOCH (0)");
				0
			}
		}
	}

	//-------------------------------------------------- Indexing.
	/// Directly index the [`Collection`] with a [`Key`].
	///
	/// # Panics:
	/// The [`ArtistKey`], [`AlbumKey`] and [`SongKey`] within
	/// the [`Key`] must be valid indicies into the [`Collection`].
	#[inline(always)]
	pub fn index(&self, key: &Key) -> (&Artist, &Album, &Song) {
		let (artist, album, song) = key.inner_usize();
		(&self.artists.0[artist], &self.albums.0[album], &self.songs.0[song])
	}

	#[inline(always)]
	/// [`slice::get`] the [`Collection`] with a [`Key`].
	///
	/// # Errors:
	/// The [`ArtistKey`], [`AlbumKey`] and [`SongKey`] within
	/// the [`Key`] must be valid indicies into the [`Collection`].
	pub fn get(&self, key: &Key) -> Option<(&Artist, &Album, &Song)> {
		let (artist, album, song) = key.inner_usize();

		let artists = match self.artists.0.get(artist) {
			Some(a) => a,
			None    => return None,
		};

		let album = match self.albums.0.get(album) {
			Some(a) => a,
			None    => return None,
		};

		let song = match self.songs.0.get(song) {
			Some(a) => a,
			None    => return None,
		};

		Some((artists, album, song))
	}

	//-------------------------------------------------- Key traversal (index).
	#[inline(always)]
	/// Obtain an [`Artist`], but from a [`AlbumKey`].
	///
	/// # Panics:
	/// The [`AlbumKey`] must be a valid index.
	pub fn artist_from_album(&self, key: AlbumKey) -> &Artist {
		&self.artists[self.albums[key].artist]
	}

	#[inline(always)]
	/// Obtain an [`Album`], but from a [`SongKey`].
	///
	/// # Panics:
	/// The [`SongKey`] must be a valid index.
	pub fn album_from_song(&self, key: SongKey) -> &Album {
		&self.albums[self.songs[key].album]
	}

	#[inline(always)]
	/// Obtain an [`Artist`], but from a [`SongKey`].
	///
	/// # Panics:
	/// The [`SongKey`] must be a valid index.
	pub fn artist_from_song(&self, key: SongKey) -> &Artist {
		&self.artist_from_album(self.songs[key].album)
	}

	//-------------------------------------------------- Key traversal (`.get()`).
	#[inline(always)]
	/// Obtain an [`Artist`], but from a [`AlbumKey`].
	///
	/// # Errors:
	/// The [`AlbumKey`] must be a valid index.
	pub fn get_artist_from_album(&self, key: AlbumKey) -> Option<&Artist> {
		let artist = match self.albums.get(key) {
			Some(a) => a.artist,
			None    => return None,
		};

		self.artists.get(artist)
	}

	#[inline(always)]
	/// Obtain an [`Album`], but from a [`SongKey`].
	///
	/// # Errors:
	/// The [`SongKey`] must be a valid index.
	pub fn get_album_from_song(&self, key: SongKey) -> Option<&Album> {
		let album = match self.songs.get(key) {
			Some(a) => a.album,
			None    => return None,
		};

		self.albums.get(album)
	}

	#[inline(always)]
	/// Obtain an [`Artist`], but from a [`SongKey`].
	///
	/// # Errors:
	/// The [`SongKey`] must be a valid index.
	pub fn get_artist_from_song(&self, key: SongKey) -> Option<&Artist> {
		let album = match self.songs.get(key) {
			Some(a) => a.album,
			None    => return None,
		};

		self.get_artist_from_album(album)
	}

	//-------------------------------------------------- Sorting
	#[inline]
	/// Access a particular `sort_artist_` field in the [`Collection`] via a [`ArtistSort`].
	pub fn artist_sort(&self, sort: &ArtistSort) -> &Vec<ArtistKey> {
		use ArtistSort::*;
		match sort {
			Lexi       => &self.sort_artist_lexi,
			AlbumCount => &self.sort_artist_album_count,
			SongCount  => &self.sort_artist_song_count,
		}
	}

	#[inline]
	/// Access a particular `sort_album_` field in the [`Collection`] via a [`AlbumSort`].
	pub fn album_sort(&self, sort: &AlbumSort) -> &Vec<AlbumKey> {
		use AlbumSort::*;
		match sort {
			ReleaseArtistLexi => &self.sort_album_release_artist_lexi,
			LexiArtistLexi    => &self.sort_album_lexi_artist_lexi,
			Lexi              => &self.sort_album_lexi,
			Release           => &self.sort_album_release,
			Runtime           => &self.sort_album_runtime,
		}
	}

	#[inline]
	/// Access a particular `sort_song_` field in the [`Collection`] via a [`SongSort`].
	pub fn song_sort(&self, sort: &SongSort) -> &Vec<SongKey> {
		use SongSort::*;
		match sort {
			AlbumReleaseArtistLexi => &self.sort_song_album_release_artist_lexi,
			AlbumLexiArtistLexi    => &self.sort_song_album_lexi_artist_lexi,
			Lexi                   => &self.sort_song_lexi,
			Release                => &self.sort_song_release,
			Runtime                => &self.sort_song_runtime,
		}
	}
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn _() {
//  }
//}
