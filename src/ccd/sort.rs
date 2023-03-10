//---------------------------------------------------------------------------------------------------- Use
//use anyhow::{anyhow,bail,ensure};
//use log::{info,error,warn,trace,debug};
//use serde::{Serialize,Deserialize};
//use crate::macros::*;
//use disk::prelude::*;
//use disk::{};
//use std::{};
//use std::sync::{Arc,Mutex,RwLock};
use crate::collection::{
	Artist,
	Album,
	Song,
	ArtistKey,
	AlbumKey,
	SongKey,
	Collection,
};

//---------------------------------------------------------------------------------------------------- __NAME__
// These functions create new sorted `Vec<_Key>`'s.
// Each function matches a field within the `Collection`.
//
// INVARIANT:
// These functions assume the input data is correct.
// AKA, the `Collection` should already be filled out with (un-sorted) data.
//
// They also depend on each other as it goes down, e.g:
// Songs depend on -> Sorted Albums depends on -> Sorted Artists
//
// So, these functions should be called from top to bottom as defined here, such
// that the output of the previous function can be used as the input to the next.
impl super::Ccd {
	#[inline]
	// Returns a `Vec` filled with a specified amount of `usize`.
	fn filled_vec_usize(len: usize) -> Vec<usize> {
		(0..len).map(|k| k).collect()
	}

	//--------------------------------------------------------------- `ArtistKey` sorts.
	#[inline]
	pub(super) fn sort_artist_lexi(artists: &[Artist]) -> Vec<ArtistKey> {
		let mut vec_artist = Self::filled_vec_usize(artists.len());
		vec_artist.sort_by(|a, b| artists[*a].name.to_lowercase().cmp(&artists[*b].name.to_lowercase()));
		vec_artist.into_iter().map(|key| ArtistKey::from(key)).collect()
	}

	#[inline]
	pub(super) fn sort_artist_album_count(artists: &[Artist]) -> Vec<ArtistKey> {
		let mut vec_artist = Self::filled_vec_usize(artists.len());
		vec_artist.sort_by(|a, b| artists[*a].albums.len().cmp(&artists[*b].albums.len()));
		vec_artist.into_iter().map(|key| ArtistKey::from(key)).collect()
	}

	#[inline]
	pub(super) fn sort_artist_song_count(artists: &[Artist], albums: &[Album]) -> Vec<ArtistKey> {
		let mut vec_artist = Self::filled_vec_usize(artists.len());
		vec_artist.sort_by(|a, b| {
			let first:  usize = artists[*a].albums.iter().map(|a| albums[a.inner()].songs.len()).sum();
			let second: usize = artists[*b].albums.iter().map(|a| albums[a.inner()].songs.len()).sum();
			a.cmp(&b)
		});
		vec_artist.into_iter().map(|key| ArtistKey::from(key)).collect()
	}

	//--------------------------------------------------------------- `AlbumKey` sorts.
	#[inline]
	// INVARIANT:
	// These album functions require an already lexi-sorted `Vec<ArtistKey>`
	// since this iterates over the artists, and gets their albums along the way.
	pub(super) fn sort_album_release_artist_lexi(sorted_artists: &[ArtistKey], artists: &[Artist], albums: &[Album]) -> Vec<AlbumKey> {
		let mut vec_album: Vec<Vec<AlbumKey>> = Vec::with_capacity(albums.len());

		for artist in sorted_artists {
			let mut tmp: Vec<AlbumKey> = artists[artist.inner()].albums.clone();
			tmp.sort_by(|a, b|
				Self::cmp_tuple_dates(
					albums[a.inner()].release,
					albums[b.inner()].release
				)
			);
			vec_album.push(tmp);
		}

		vec_album.into_iter().flatten().collect()
	}

	#[inline]
	pub(super) fn sort_album_lexi_artist_lexi(sorted_artists: &[ArtistKey], artists: &[Artist], albums: &[Album]) -> Vec<AlbumKey> {
		let mut vec_album: Vec<Vec<AlbumKey>> = Vec::with_capacity(albums.len());

		for artist in sorted_artists {
			let mut tmp: Vec<AlbumKey> = artists[artist.inner()].albums.clone();
			tmp.sort_by(|a, b|
				albums[a.inner()].title.to_lowercase().cmp(
					&albums[b.inner()].title.to_lowercase()
				)
			);
			vec_album.push(tmp);
		}

		vec_album.into_iter().flatten().collect()
	}

	#[inline]
	// Doesn't require `Vec<Artist>`.
	pub(super) fn sort_album_lexi(albums: &[Album]) -> Vec<AlbumKey> {
		let mut vec_album = Self::filled_vec_usize(albums.len());

		vec_album.sort_by(|a, b|
			albums[*a].title.to_lowercase().cmp(
				&albums[*b].title.to_lowercase(),
			)
		);

		vec_album.into_iter().map(|key| AlbumKey::from(key)).collect()
	}

	#[inline]
	pub(super) fn sort_album_release(albums: &[Album]) -> Vec<AlbumKey> {
		let mut vec_album = Self::filled_vec_usize(albums.len());

		vec_album.sort_by(|a, b|
			Self::cmp_tuple_dates(
				albums[*a].release,
				albums[*b].release,
			)
		);

		vec_album.into_iter().map(|key| AlbumKey::from(key)).collect()
	}

	#[inline]
	// INVARIANT:
	// `runtime` is a `f64` which could be `NaN`.
	// Except I (CCD) control this and it's always at least
	// initialized as `0.0` so using `cmp_f64` is fine (it ignores `NaN`s).
	pub(super) fn sort_album_runtime(albums: &[Album]) -> Vec<AlbumKey> {
		let mut vec_album = Self::filled_vec_usize(albums.len());

		vec_album.sort_by(|a, b|
			crate::search::Search::cmp_f64(&albums[*a].runtime, &albums[*b].runtime)
		);

		vec_album.into_iter().map(|key| AlbumKey::from(key)).collect()
	}

	//--------------------------------------------------------------- `SongKey` sorts.
	#[inline]
	// INVARIANT:
	// Needs a already sorted `Vec<Album>`
	// in the variant of: `sort_song_artist_lexi_album_release`.
	pub(super) fn sort_song_artist_lexi_album_release(sorted_albums: &[AlbumKey], albums: &[Album], songs: &[Song]) -> Vec<SongKey> {
		let mut vec_song = Self::filled_vec_usize(songs.len());

		vec_song.sort_by(|a, b|
			albums[songs[*a].album.inner()].title.to_lowercase().cmp(
				&albums[songs[*b].album.inner()].title.to_lowercase()
			)
		);

		vec_song.into_iter().map(|key| SongKey::from(key)).collect()
	}

	#[inline]
	// INVARIANT:
	// Needs an already sorted `Vec<AlbumKey>`.
	//
	// The ordering of the `Song`'s are just based off iterating
	// on the given `AlbumKey`'s. So whatever order the `AlbumKey`'s
	// are in, the `Song`'s will be as well.
	pub(super) fn sort_song_iterating_over_albums(sorted_albums: &[AlbumKey], artists: &[Artist], albums: &[Album]) -> Vec<SongKey> {
		let vec_song: Vec<Vec<SongKey>> = sorted_albums.iter().map(|a| albums[a.inner()].songs.clone()).collect();
		vec_song.into_iter().flatten().collect()
	}

	#[inline]
	pub(super) fn sort_song_lexi(songs: &[Song]) -> Vec<SongKey> {
		let mut vec_song = Self::filled_vec_usize(songs.len());

		vec_song.sort_by(|a, b| {
			songs[*a].title.to_lowercase().cmp(
				&songs[*b].title.to_lowercase(),
			)
		});

		vec_song.into_iter().map(|key| SongKey::from(key)).collect()
	}

	#[inline]
	// INVARIANT:
	// `f64` must not be a `NaN`.
	// (It won't be, I control it).
	pub(super) fn sort_song_runtime(songs: &[Song]) -> Vec<SongKey> {
		let mut vec_song = Self::filled_vec_usize(songs.len());

		vec_song.sort_by(|a, b|
			crate::search::Search::cmp_f64(&songs[*a].runtime, &songs[*b].runtime)
		);

		vec_song.into_iter().map(|key| SongKey::from(key)).collect()
	}
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn __TEST__() {
//  }
//}