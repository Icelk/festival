//---------------------------------------------------------------------------------------------------- Use
//use anyhow::{anyhow,bail,ensure};
//use log::{info,error,warn,trace,debug};
//use serde::{Serialize,Deserialize};
//use crate::macros::*;
//use disk::prelude::*;
//use disk::{};
//use std::{};
//use std::sync::{Arc,Mutex,RwLock};
use egui::{
	TextureId,
	ColorImage,
};
use egui_extras::image::RetainedImage;
use super::Album;
use serde::{Serialize,Deserialize,Serializer,Deserializer};
use bincode::{Encode,Decode};
use once_cell::sync::Lazy;

//---------------------------------------------------------------------------------------------------- Constant
/// The [`Album`] art size in pixels
///
/// 600x600 pixels.
///
/// Album art will _always_ be resized internally to this size.
pub const ALBUM_ART_SIZE: usize = 600;

//---------------------------------------------------------------------------------------------------- Unknown Art (lazy) Constant
pub(crate) const UNKNOWN_ALBUM_BYTES: &[u8] = include_bytes!("../../../assets/images/art/unknown.png");
pub(crate) static UNKNOWN_ALBUM: Lazy<RetainedImage> = Lazy::new(|| RetainedImage::from_image_bytes("Unknown", UNKNOWN_ALBUM_BYTES).unwrap());

//---------------------------------------------------------------------------------------------------- Art
#[derive(Default)]
/// An `enum` that is _always_ an image.
///
/// Some [`Album`]'s may not have art. In this case, we'd like to show a "unknown" image anyway.
///
/// This `enum` and the associated function [`Album::art_or()`] will always return
/// a valid [`egui_extras::RetainedImage`], the real art if it exists, or an "unknown" image.
///
/// The returned "unknown" image is actually just a pointer to a single image.
///
/// The "unknown" image is from `assets/images/art/unknown.png`.
pub enum Art {
	/// This is a known-good, already resized [`RetainedImage`] that
	/// can be used in `egui`.
	///
	/// This image's width/height is guaranteed to be [`ALBUM_ART_SIZE`].
	Known(RetainedImage),
	/// This is raw image bytes that have not yet been transformed into [`Art::Known`].
	///
	/// This variant is never exposed to a `Frontend`, as `Kernel` turns all [`Art`]
	/// into either [`Art::Known`] or [`Art::Unknown`].
	Bytes(Box<[u8]>),
	#[default]
	/// A gray background, white question-mark image representing an unknown image.
	///
	/// This image's width/height is guaranteed to be [`ALBUM_ART_SIZE`].
	Unknown,
}

//---------------------------------------------------------------------------------------------------- Art `Ord`
impl Ord for Art {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		match (self, other) {
			(Self::Unknown, Self::Unknown) => std::cmp::Ordering::Equal,
			(Self::Bytes(_), Self::Bytes(_)) => std::cmp::Ordering::Equal,
			(Self::Known(_), Self::Known(_)) => std::cmp::Ordering::Equal,
			(Self::Known(_), _) => std::cmp::Ordering::Greater,
			(_, Self::Known(_)) => std::cmp::Ordering::Less,
			(Self::Bytes(_), _) => std::cmp::Ordering::Greater,
			(_, Self::Bytes(_)) => std::cmp::Ordering::Less,
		}
	}
}
impl PartialOrd for Art {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		Some(self.cmp(other))
	}
}

//---------------------------------------------------------------------------------------------------- Art `Eq`
impl Eq for Art {}
impl PartialEq for Art {
	fn eq(&self, other: &Self) -> bool {
		match (self, other) {
			(Self::Unknown, Self::Unknown) => true,
			(Self::Bytes(b1), Self::Bytes(b2)) => b1 == b2,
			(Self::Known(_), Self::Known(_)) => true,
			_ => false,
		}
	}
}

//---------------------------------------------------------------------------------------------------- Art Impl
impl Art {
	#[inline(always)]
	/// Returns [`Self::Unknown`].
	pub(crate) const fn new() -> Self {
		Self::Unknown
	}
}

impl Art {
	#[inline]
	// Return the associated art or the default `[?]` image if [`Art::Unknown`]
	pub(crate) fn art_or(&self) -> &RetainedImage {
		match self {
			Self::Known(art) => art,
			_ => &UNKNOWN_ALBUM,
		}
	}

	#[inline]
	// Same as [`Art::art_or`] but with no backup image.
	pub(crate) fn get(&self) -> Option<&RetainedImage> {
		match self {
			Self::Known(art) => Some(art),
			_ => None,
		}
	}

	#[inline]
	// Calls [`egui::extras::texture_id`].
	pub(crate) fn texture_id(&self, ctx: &egui::Context) -> egui::TextureId {
		match self {
			Self::Known(a) => a.texture_id(ctx),
			// TODO: `lazy` this id, no need to lock
			_ => UNKNOWN_ALBUM.texture_id(ctx),
		}
	}
}

impl std::fmt::Debug for Art {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Known(_) => write!(f, "Art::Known(RetainedImage)"),
			Self::Bytes(b) => write!(f, "Art::Bytes({})", b.len()),
			Self::Unknown  => write!(f, "Art::Unknown"),
		}
	}
}

//---------------------------------------------------------------------------------------------------- Art Clone
impl Clone for Art {
	fn clone(&self) -> Self {
		match self {
			Self::Bytes(vec) => Self::Bytes(vec.clone()),
			_ => Self::Unknown,
		}
	}
}

//---------------------------------------------------------------------------------------------------- Art Serde
// HACK:
// Read step 6-7 of `ccd.rs`.
//
// Basically, `RetainedImage` does not implement `serde`
// and extracting the raw image bytes AFTER we've already
// transformed `Vec<u8>` to a `RetainedImage` is extremely painful.
//
// So, we save the bytes to disk before hand, and deserialize the
// bytes into `Art::Bytes`, and convert to `Art::Known` at Festival's startup.
//
// Q1: Why the ugly machine-generated impl below?
// Q2: Why not `#[serde(skip)]` the `Art::Known` variant?
//
// A: Because `bincode` does not support it.
//
// We must manually (kinda) implement `Serialize/Deserialize` so that `Art`
// variants are _only_ (de)serialized as either `Bytes` or `Unknown`.
//
// The way to reproduce the below code:
//     1. Write `Art` enum that matches the one here,
//        replacing `RetainedImage` with a serde compatible type
//     2. Derive `Serialize,Deserialize`
//     3. Run `cargo-expand`
//
// The output will be something similar to below.
// Some changes need to be made so `Art::Known` is handled.
use serde::ser::SerializeSeq;
use serde::de::Visitor;

#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for Art {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            match *self {
                Art::Bytes(ref __field0) => {
                    _serde::Serializer::serialize_newtype_variant(
                        __serializer,
                        "Art",
                        1u32,
                        "Bytes",
                        __field0,
                    )
                }
                _ => {
                    _serde::Serializer::serialize_unit_variant(
                        __serializer,
                        "Art",
                        2u32,
                        "Unknown",
                    )
                }
            }
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for Art {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            #[doc(hidden)]
            enum __Field {
                __field0,
                __field1,
                __field2,
            }
            #[doc(hidden)]
            struct __FieldVisitor;
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "variant identifier",
                    )
                }
                fn visit_u64<__E>(
                    self,
                    __value: u64,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        1u64 => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__field2),
                    }
                }
                fn visit_str<__E>(
                    self,
                    __value: &str,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "Bytes" => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__field2),
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"Bytes" => _serde::__private::Ok(__Field::__field1),
                        _ => {
                            let __value = &_serde::__private::from_utf8_lossy(__value);
                            _serde::__private::Err(
                                _serde::de::Error::unknown_variant(__value, VARIANTS),
                            )
                        }
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(
                        __deserializer,
                        __FieldVisitor,
                    )
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<Art>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = Art;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "enum Art")
                }
                fn visit_enum<__A>(
                    self,
                    __data: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::EnumAccess<'de>,
                {
                    match match _serde::de::EnumAccess::variant(__data) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    } {
                        (__Field::__field1, __variant) => {
                            _serde::__private::Result::map(
                                _serde::de::VariantAccess::newtype_variant::<
                                    Box<[u8]>,
                                >(__variant),
                                Art::Bytes,
                            )
                        }
                        (_, __variant) => {
                            match _serde::de::VariantAccess::unit_variant(__variant) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            };
                            _serde::__private::Ok(Art::Unknown)
                        }
                    }
                }
            }
            #[doc(hidden)]
            const VARIANTS: &[&str] = &["Known", "Bytes", "Unknown"];
            _serde::Deserializer::deserialize_enum(
                __deserializer,
                "Art",
                VARIANTS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<Art>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};

//---------------------------------------------------------------------------------------------------- Art Bincode
// Same thing as above, but for `bincode`'s `Encode` & `Decode`
impl bincode::Encode for Art {
	fn encode<E: bincode::enc::Encoder>(&self, encoder: &mut E) -> std::result::Result<(), bincode::error::EncodeError> {
		match self {
			Self::Bytes(field_0) => {
				<u32 as bincode::Encode>::encode(&(1u32), encoder)?;
				bincode::Encode::encode(field_0, encoder)?;
				Ok(())
			},
			_ => {
				<u32 as bincode::Encode>::encode(&(2u32), encoder)?;
				Ok(())
			},
		}
	}
}
impl bincode::Decode for Art {
	fn decode<D: bincode::de::Decoder>(decoder: &mut D) -> std::result::Result<Self, bincode::error::DecodeError> {
		let variant_index = <u32 as bincode::Decode>::decode(decoder)?;
		match variant_index {
			1u32 => {
				Ok(Self::Bytes(bincode::Decode::decode(decoder)?))
			},
			_ => Ok(Self::Unknown),
		}
	}
}
impl<'de> bincode::BorrowDecode<'de> for Art {
	fn borrow_decode<D: bincode::de::BorrowDecoder<'de>>(decoder: &mut D) -> std::result::Result<Self, bincode::error::DecodeError> {
		let variant_index = <u32 as bincode::Decode>::decode(decoder)?;
		match variant_index {
			1u32 => {
				Ok(Self::Bytes(bincode::BorrowDecode::borrow_decode(decoder)?))
			},
			_ => Ok(Self::Unknown),
		}
	}
}

//---------------------------------------------------------------------------------------------------- TESTS
#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn unknown_art() {
		// Make sure the `.unwrap()` doesn't panic.
		assert!(UNKNOWN_ALBUM.size() == [ALBUM_ART_SIZE; 2]);
	}
}