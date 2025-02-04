//! Traits for hashing to field elements.
//!
//! <https://datatracker.ietf.org/doc/draft-irtf-cfrg-hash-to-curve>

mod expand_msg;

pub use expand_msg::{xmd::*, xof::*, *};

use crate::Result;
use generic_array::{typenum::Unsigned, ArrayLength, GenericArray};

/// The trait for helping to convert to a field element.
pub trait FromOkm {
    /// The number of bytes needed to convert to a field element.
    type Length: ArrayLength<u8>;

    /// Convert a byte sequence into a field element.
    fn from_okm(data: &GenericArray<u8, Self::Length>) -> Self;
}

/// Convert an arbitrary byte sequence into a field element.
///
/// <https://tools.ietf.org/html/draft-irtf-cfrg-hash-to-curve-11#section-5.3>
pub fn hash_to_field<E, T>(data: &[u8], domain: &'static [u8], out: &mut [T]) -> Result<()>
where
    E: ExpandMsg,
    T: FromOkm + Default,
{
    let len_in_bytes = T::Length::to_usize() * out.len();
    let mut tmp = GenericArray::<u8, <T as FromOkm>::Length>::default();
    let mut expander = E::expand_message(data, domain, len_in_bytes)?;
    for o in out.iter_mut() {
        expander.fill_bytes(&mut tmp);
        *o = T::from_okm(&tmp);
    }
    Ok(())
}
