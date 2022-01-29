//! Simple types for testing macro generation

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(any(feature = "std", test))]
#[allow(unused_imports)]
#[macro_use]
extern crate std;

#[cfg(all(not(feature = "std"), not(test)))]
#[macro_use]
extern crate core as std;

mod array;
mod blob;

/// 8-bit unsigned integer
#[allow(non_camel_case_types)]
pub type uint8 = u8;

/// 8-bit signed integer
#[allow(non_camel_case_types)]
pub type int8 = i8;

/// 16-bit unsigned integer
#[allow(non_camel_case_types)]
pub type uint16 = u16;

/// 16-bit signed integer
#[allow(non_camel_case_types)]
pub type int16 = i16;

/// 32-bit unsigned integer
#[allow(non_camel_case_types)]
pub type uint32 = u32;

/// 32-bit signed integer
#[allow(non_camel_case_types)]
pub type int32 = i32;

pub struct Uint24([u8; 3]);

pub type Fixed = i32;
pub type F2dot14 = i16;
pub type LongDateTime = i64;
pub type Offset16 = u16;
pub type Offset24 = Uint24;
pub type Offset32 = u32;
pub type Tag = [u8; 4];
pub type Version16Dot16 = u32;
pub use array::Array;
pub use blob::Blob;

/// a concrete table. generated by macro.
pub trait FontThing<'a>: FontRead<'a> {
    /// A zero-copy 'view' type into the same table.
    type View: FontRead<'a>;
}

/// Things that can be read from font data.
pub trait FontRead<'a>: Sized {
    fn read(blob: Blob<'a>) -> Option<Self>;
}

/// A marker for types that can be represented with a set number of bytes.
///
/// This is implemented for all the scalars. We use inside macros to generate
/// offsets at compile time.
pub trait ExactSized: Sized {
    const SIZE: usize = std::mem::size_of::<Self>();
}

/// A type that can be constructed directly from bytes.
///
/// This is used for scalars.
pub trait FromBeBytes: Sized {
    fn from_bytes(bytes: &[u8]) -> Option<Self>;
}

// this is weirdly cyclical and I don't even know if it makes sense
impl<'a, T: ExactSized + FromBeBytes> FontRead<'a> for T {
    fn read(blob: Blob<'a>) -> Option<Self> {
        blob.read(0)
    }
}

macro_rules! be_scalar {
    ($name:ident, $size:literal) => {
        impl ExactSized for $name {
            const SIZE: usize = $size;
        }

        impl FromBeBytes for $name {
            #[inline(always)]
            fn from_bytes(raw: &[u8]) -> Option<Self> {
                raw.get(..Self::SIZE)
                    .map(|b| $name::from_be_bytes(b.try_into().unwrap()))
            }
        }
    };
}

be_scalar!(u8, 1);
be_scalar!(i8, 1);
be_scalar!(u16, 2);
be_scalar!(i16, 2);
be_scalar!(u32, 4);
be_scalar!(i32, 4);
be_scalar!(i64, 8);

impl ExactSized for [u8; 4] {}

impl ExactSized for Uint24 {
    const SIZE: usize = 3;
}

impl FromBeBytes for [u8; 4] {
    fn from_bytes(bytes: &[u8]) -> Option<Self> {
        bytes.get(..4).map(|b| Self::try_from(b).unwrap())
    }
}
impl FromBeBytes for Uint24 {
    fn from_bytes(bytes: &[u8]) -> Option<Self> {
        bytes.get(..4).map(|b| Self(b.try_into().unwrap()))
    }
}