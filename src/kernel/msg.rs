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
	Song,
	Collection,
};
use crate::key::{
	Keychain,
	QueueKey,
};
use super::KernelState;
use rolock::RoLock;
use std::path::PathBuf;
use crate::kernel::Volume;
use super::Kernel;

//---------------------------------------------------------------------------------------------------- Kernel Messages.
/// Messages `Frontend` can send to [`Kernel`]
///
/// This is the "API" that all frontends must implement
/// in order to communicate with `Festival`'s internals.
///
/// You can treat these as "commands" sent to [`Kernel`].
pub enum FrontendToKernel {
	// Audio playback.
	/// Play current song.
	Play,
	/// Stop playback.
	Stop,
	/// Play next song in queue (stop if none).
	Next,
	/// Play last song in queue.
	Last,
	/// Seek to point in current song.
	Seek(f64),

	// Audio settings.
	/// Toggle shuffling songs.
	Shuffle,
	/// Toggle repeating songs.
	Repeat,
	/// Change the audio volume.
	Volume(Volume),

	// Queue/playlist.
	/// Play the `n`'th index [`Song`] in the queue.
	PlayQueueKey(QueueKey),

	// Collection.
	/// I'd like a new [`Collection`], scanning these [`PathBuf`]'s for audio files.
	NewCollection(Vec<PathBuf>),
	/// I'd like to search the [`Collection`] with this [`String`].
	Search(String),

	// Exiting.
	/// I'm exiting, save everything.
	///
	/// # Notes
	/// After you send this message, [`Kernel`] will save everything, and respond with a
	/// [`KernelToFrontend::ExitResponse`] that contains either a [`Result::Ok`] meaning
	/// everything went okay, or [`Result::Err`] with a [`String`] payload containing an error message.
	///
	/// After the reponse (regardless of the [`Result`]), [`Kernel`] will
	/// - [`std::thread::park`] forever
	/// - Ignore all channel messages
	///
	/// After you receive the response, you should [`std::process::exit`] to kill all threads.
	Exit,
}

/// Messages [`Kernel`] can send to `Frontend`
///
/// This is the "API" that all frontends must implement
/// in order to communicate with `Festival`'s internals.
///
/// You can treat these as "commands" sent _from_ [`Kernel`] that you _**must**_ follow correctly.
///
/// [`Kernel`] assumes that all of these messages are implemented correctly.
///
/// # For example:
/// If your frontend does _not_ actually drop the `Arc<Collection>`
/// after receiving the message [`KernelToFrontend::DropCollection`],
/// then `Festival`'s internals will not be able to destruct the old
/// [`Collection`] correctly.
///
/// This will leave the deconstruction of the old [`Collection`] up to
/// your frontend thread, which is most likely not desired, as it will
/// probably skip a few frames or cause latency.
pub enum KernelToFrontend {
	// Collection.
	/// Drop your [`Arc`] pointer to the [`Collection`].
	DropCollection,
	/// Here's the new [`Collection`] pointer.
	NewCollection(Arc<Collection>),
	/// Here's an update on the new [`Collection`].
	Update(String),
	/// Creating the new [`Collection`] failed, here's the old pointer and error message.
	Failed((Arc<Collection>, String)),

	// Audio error.
	/// The audio file at this [`PathBuf`] has errored (probably doesn't exist).
	PathError(String),

	// Misc.
	/// Here's a new [`KernelState`] pointer inside a [`rolock::RoLock`].
	NewState(RoLock<KernelState>),
	/// Here's a search result.
	SearchResult(Keychain),

	// Exit.
	/// You sent a [`FrontendToKernel::Exit`], here is the [`Result`]
	/// of saving the data. I'm going to [`std::thread::park`] forever
	/// after this response and ignore channel messages.
	ExitResponse(Result<(), String>),
}

//---------------------------------------------------------------------------------------------------- TESTS
//#[cfg(test)]
//mod tests {
//  #[test]
//  fn __TEST__() {
//  }
//}
