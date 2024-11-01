//---------------------------------------------------------------------------------------------------- Use
use crate::collection::{AlbumKey, SongKey};
use bincode::{Decode, Encode};
use readable::Runtime;
use serde::Serialize;
use std::path::PathBuf;
use std::sync::Arc;

//----------------------------------------------------------------------------------------------------
#[derive(Clone, Debug, Hash, PartialEq, PartialOrd, Serialize, Encode, Decode)]
/// Struct holding [`Song`] metadata, with a pointer to the [`Album`] it belongs to
///
/// This struct holds all the metadata about a particular [`Song`].
///
/// It contains a [`SongKey`] that is the index of the owning [`Album`], in the [`Collection`].
pub struct Song {
    /// Title of the [`Song`].
    pub title: Arc<str>,
    #[serde(skip)]
    /// Title of the [`Song`] in "Unicode Derived Core Property" lowercase.
    pub title_lowercase: Arc<str>,

    /// This [`Song`]'s [`SongKey`].
    pub key: SongKey,
    /// Key to the [`Album`].
    pub album: AlbumKey,

    #[serde(serialize_with = "crate::serde::runtime")]
    /// Total runtime of this [`Song`].
    pub runtime: Runtime,
    /// Sample rate of this [`Song`].
    pub sample_rate: u32,
    /// The track number of this [`Song`].
    pub track: Option<u32>,
    /// The disc number of this [`Song`].
    pub disc: Option<u32>,

    /// The `MIME` type of this [`Song`].
    pub mime: Arc<str>,
    /// The file extension of this [`Song`].
    pub extension: Arc<str>,

    #[serde(skip)]
    /// The [`PathBuf`] this [`Song`] is located at.
    pub path: PathBuf,
}

impl Default for Song {
    fn default() -> Self {
        Self {
            key: SongKey::zero(),
            title: "".into(),
            title_lowercase: "".into(),
            album: Default::default(),
            runtime: Default::default(),
            sample_rate: Default::default(),
            track: Default::default(),
            disc: Default::default(),
            mime: "".into(),
            extension: "".into(),
            path: Default::default(),
        }
    }
}

//---------------------------------------------------------------------------------------------------- TESTS
#[cfg(test)]
mod tests {
    use super::*;
}
