
use crate::{Error, Header, Result};
use bytes::{Bytes, BytesMut};
use bytes::{Buf, BufMut};
use  std::default::Default;
use core::{borrow::BorrowMut, marker::{PhantomData, PhantomPinned}};
use crate::{ EMPTY_LIST_CODE, EMPTY_STRING_CODE};
use core::hint::unreachable_unchecked;

/// A type that can be decoded from an Decoder blob.
pub trait RlpDecodable: Sized {
    /// Decodes the blob into the appropriate type. `buf` must be advanced past
    /// the decoded object.
    fn rlp_decode(buf: &mut &[u8]) -> Result<Self>;
    fn rlp_decode_raw(buf: &mut &[u8]) -> Result<Self> {
        Self::rlp_decode(buf)
    }
}

/// An active Decoder decoder, with a specific slice of a payload.
#[derive(Default)]
pub struct Decoder<'a> {
    payload_view: &'a [u8],
    /// True if list, false otherwise.
    header: Option<Header>

}

impl<'a> Decoder<'a> {
    /// Instantiate an RLP decoder with a payload slice.
    pub fn new(mut payload: &'a [u8]) -> Result<Self> {
        let payload_view = Header::decode_bytes(&mut payload, true)?;
        Ok(Self { payload_view, header: None })
    }

    /// Decode the next item from the buffer.
    #[inline]
    pub fn get_next<T: RlpDecodable>(&mut self) -> Result<Option<T>> {
        if self.payload_view.is_empty() {
            Ok(None)
        } else {
            T::rlp_decode(&mut self.payload_view).map(Some)
        }
    }

  



    /// Decodes an RLP header from the given buffer.
    ///
    /// # Errors
    ///
    /// Returns an error if the buffer is too short or the header is invalid.
    #[inline]
    pub fn decode(buf: &mut &[u8]) -> Result<Self> {
        let payload_length;
        let mut list = false;
        match get_next_byte(buf)? {
            0..=0x7F => payload_length = 1,

            b @ EMPTY_STRING_CODE..=0xB7 => {
                buf.advance(1);
                payload_length = (b - EMPTY_STRING_CODE) as usize;
                if payload_length == 1 && get_next_byte(buf)? < EMPTY_STRING_CODE {
                    return Err(Error::NonCanonicalSingleByte);
                }
            }

            b @ (0xB8..=0xBF | 0xF8..=0xFF) => {
                buf.advance(1);

                list = b >= 0xF8; // second range
                let code = if list { 0xF7 } else { 0xB7 };

                // SAFETY: `b - code` is always in the range `1..=8` in the current match arm.
                // The compiler/LLVM apparently cannot prove this because of the `|` pattern +
                // the above `if`, since it can do it in the other arms with only 1 range.
                let len_of_len = unsafe { b.checked_sub(code).unwrap_unchecked() } as usize;
                if len_of_len == 0 || len_of_len > 8 {
                    unsafe { unreachable_unchecked() }
                }

                if buf.len() < len_of_len {
                    return Err(Error::InputTooShort);
                }
                // SAFETY: length checked above
                let len = unsafe { buf.get_unchecked(..len_of_len) };
                buf.advance(len_of_len);

                let len = u64::from_be_bytes(static_left_pad(len)?);
                payload_length =
                    usize::try_from(len).map_err(|_| Error::Custom("Input too big"))?;
                if payload_length < 56 {
                    return Err(Error::NonCanonicalSize);
                }
            }

            b @ EMPTY_LIST_CODE..=0xF7 => {
                buf.advance(1);
                list = true;
                payload_length = (b - EMPTY_LIST_CODE) as usize;
            }
        }

        if buf.remaining() < payload_length {
            return Err(Error::InputTooShort);
        }

        Ok(Self { list, payload_length })
    }




    /// Decodes the next payload from the given buffer, advancing it.
    ///
    /// # Errors
    ///
    /// Returns an error if the buffer is too short or the header is invalid.
    #[inline]
    pub fn decode_bytes<'b>(buf: &mut &'b [u8], is_list: bool) -> Result<&'b [u8]> {
        //let Self { header: Option<Header<list, payload_length>> } = Self::decode(buf)?;
        let Self { header: Option<header: Header{list, payload_length}> } = Self::decode(buf)?;
        
        if list != is_list {
            return Err(if is_list { Error::UnexpectedString } else { Error::UnexpectedList });
        }

        // SAFETY: this is already checked in `decode`
        if buf.remaining() < payload_length {
            unsafe { unreachable_unchecked() }
        }
        let bytes = unsafe { buf.get_unchecked(..payload_length) };
        buf.advance(payload_length);
        Ok(bytes)
    }


    /// Decodes the next payload from the given buffer, advancing it.
    ///
    /// # Errors
    ///
    /// Returns an error if the buffer is too short or the header is invalid.
    #[inline]
    pub fn decode_raw<'a>(buf: &mut &'a [u8]) -> Result<&'a [u8]> {
        //let Self { list, payload_length } = Self::decode(buf)?;

       
        // SAFETY: this is already checked in `decode`
        if buf.remaining() < payload_length {
            unsafe { unreachable_unchecked() }
        }
        let bytes = unsafe { buf.get_unchecked(..payload_length) };
        buf.advance(payload_length);
        Ok(bytes)
    }



    /// Decodes a string slice from the given buffer, advancing it.
    ///
    /// # Errors
    ///
    /// Returns an error if the buffer is too short or the header is invalid.
    #[inline]
    pub fn decode_str<'a>(buf: &mut &'a [u8]) -> Result<&'a str> {
        let bytes = Self::decode_bytes(buf, false)?;
        core::str::from_utf8(bytes).map_err(|_| Error::Custom("invalid string"))
    }

    /// Encodes the header into the `out` buffer.
    #[inline]
    pub fn encode(&self, out: &mut dyn BufMut) {
        if self.payload_length < 56 {
            let code = if self.list { EMPTY_LIST_CODE } else { EMPTY_STRING_CODE };
            out.put_u8(code + self.payload_length as u8);
        } else {
            let len_be;
            let len_be = crate::encode::to_be_bytes_trimmed!(len_be, self.payload_length);
            let code = if self.list { 0xF7 } else { 0xB7 };
            out.put_u8(code + len_be.len() as u8);
            out.put_slice(len_be);
        }
    }

    /// Returns the length of the encoded header.
    #[inline]
    pub const fn length(&self) -> usize {
        crate::length_of_length(self.payload_length)
    }




}



impl<T: ?Sized> RlpDecodable for PhantomData<T> {
    fn rlp_decode(_buf: &mut &[u8]) -> Result<Self> {
        Ok(Self)
    }
}

impl RlpDecodable for PhantomPinned {
    fn rlp_decode(_buf: &mut &[u8]) -> Result<Self> {
        Ok(Self)
    }
}

impl RlpDecodable for bool {
    #[inline]
    fn rlp_decode(buf: &mut &[u8]) -> Result<Self> {
        Ok(match u8::rlp_decode(buf)? {
            0 => false,
            1 => true,
            _ => return Err(Error::Custom("invalid bool value, must be 0 or 1")),
        })
    }
}

impl<const N: usize> RlpDecodable for [u8; N] {
    #[inline]
    fn rlp_decode(from: &mut &[u8]) -> Result<Self> {
        let bytes = Header::decode_bytes(from, false)?;
        Self::try_from(bytes).map_err(|_| Error::UnexpectedLength)
    }
}

macro_rules! decode_integer {
    ($($t:ty),+ $(,)?) => {$(
        impl RlpDecodable for $t {
            #[inline]
            fn rlp_decode(buf: &mut &[u8]) -> Result<Self> {
                let bytes = Header::decode_bytes(buf, false)?;
                static_left_pad(bytes).map(<$t>::from_be_bytes)
            }
        }
    )+};
}


// add int support
decode_integer!(u8, u16, u32, u64, usize, u128, i8, i16, i32, i64, isize, i128);

impl RlpDecodable for Bytes {
    #[inline]
    fn rlp_decode(buf: &mut &[u8]) -> Result<Self> {
        Header::decode_bytes(buf, false).map(|x| Self::from(x.to_vec()))
    }
}

impl RlpDecodable for BytesMut {
    #[inline]
    fn rlp_decode(buf: &mut &[u8]) -> Result<Self> {
        Header::decode_bytes(buf, false).map(Self::from)
    }
}

impl RlpDecodable for alloc::string::String {
    #[inline]
    fn rlp_decode(buf: &mut &[u8]) -> Result<Self> {
        Header::decode_str(buf).map(Into::into)
    }
}

impl<T: RlpDecodable> RlpDecodable for alloc::vec::Vec<T> {
    #[inline]
    fn rlp_decode(buf: &mut &[u8]) -> Result<Self> {
        let mut bytes = Header::decode_bytes(buf, true)?;
        let mut vec = Self::new();
        let payload_view = &mut bytes;
        while !payload_view.is_empty() {
            vec.push(T::rlp_decode(payload_view)?);
        }
        Ok(vec)
    }
}

macro_rules! wrap_impl {
    ($($(#[$attr:meta])* [$($gen:tt)*] <$t:ty>::$new:ident($t2:ty)),+ $(,)?) => {$(
        $(#[$attr])*
        impl<$($gen)*> RlpDecodable for $t {
            #[inline]
            fn rlp_decode(buf: &mut &[u8]) -> Result<Self> {
                <$t2 as RlpDecodable>::rlp_decode(buf).map(<$t>::$new)
            }
        }
    )+};
}

wrap_impl! {
    #[cfg(feature = "arrayvec")]
    [const N: usize] <arrayvec::ArrayVec<u8, N>>::from([u8; N]),
    [T: ?Sized + RlpDecodable] <alloc::boxed::Box<T>>::new(T),
    [T: ?Sized + RlpDecodable] <alloc::rc::Rc<T>>::new(T),
    [T: ?Sized + RlpDecodable] <alloc::sync::Arc<T>>::new(T),
}

impl<T: ?Sized + alloc::borrow::ToOwned> RlpDecodable for alloc::borrow::Cow<'_, T>
where
    T::Owned: RlpDecodable,
{
    #[inline]
    fn rlp_decode(buf: &mut &[u8]) -> Result<Self> {
        T::Owned::rlp_decode(buf).map(Self::Owned)
    }
}

#[cfg(feature = "std")]
mod std_impl {
    use super::*;
    use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

    impl RlpDecodable for IpAddr {
        fn rlp_decode(buf: &mut &[u8]) -> Result<Self> {
            let bytes = Header::decode_bytes(buf, false)?;
            match bytes.len() {
                4 => Ok(Self::V4(Ipv4Addr::from(slice_to_array::<4>(bytes).expect("infallible")))),
                16 => {
                    Ok(Self::V6(Ipv6Addr::from(slice_to_array::<16>(bytes).expect("infallible"))))
                }
                _ => Err(Error::UnexpectedLength),
            }
        }
    }

    impl RlpDecodable for Ipv4Addr {
        #[inline]
        fn rlp_decode(buf: &mut &[u8]) -> Result<Self> {
            let bytes = Header::decode_bytes(buf, false)?;
            slice_to_array::<4>(bytes).map(Self::from)
        }
    }

    impl RlpDecodable for Ipv6Addr {
        #[inline]
        fn rlp_decode(buf: &mut &[u8]) -> Result<Self> {
            let bytes = Header::decode_bytes(buf, false)?;
            slice_to_array::<16>(bytes).map(Self::from)
        }
    }
}

/// Left-pads a slice to a statically known size array.
///
/// # Errors
///
/// Returns an error if the slice is too long or if the first byte is 0.
#[inline]
pub(crate) fn static_left_pad<const N: usize>(data: &[u8]) -> Result<[u8; N]> {
    if data.len() > N {
        return Err(Error::Overflow);
    }

    let mut v = [0; N];

    if data.is_empty() {
        return Ok(v);
    }

    if data[0] == 0 {
        return Err(Error::LeadingZero);
    }

    // SAFETY: length checked above
    unsafe { v.get_unchecked_mut(N - data.len()..) }.copy_from_slice(data);
    Ok(v)
}

#[cfg(feature = "std")]
#[inline]
fn slice_to_array<const N: usize>(slice: &[u8]) -> Result<[u8; N]> {
    slice.try_into().map_err(|_| Error::UnexpectedLength)
}


#[inline(always)]
fn get_next_byte(buf: &[u8]) -> Result<u8> {
    if buf.is_empty() {
        return Err(Error::InputTooShort);
    }
    // SAFETY: length checked above
    Ok(*unsafe { buf.get_unchecked(0) })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::RlpEncodable;
    use core::fmt::Debug;
    use hex_literal::hex;
    use crate::RlpDecodable;



    #[allow(unused_imports)]
    use alloc::{string::String, vec::Vec};

    fn check_decode<'a, T, IT>(fixtures: IT)
    where
        T: RlpEncodable + RlpDecodable + PartialEq + Debug,
        IT: IntoIterator<Item = (Result<T>, &'a [u8])>,
    {
        for (expected, mut input) in fixtures {
            if let Ok(expected) = &expected {
                assert_eq!(crate::rlp_encode(expected), input, "{expected:?}");
            }

            let orig = input;
            assert_eq!(
                T::rlp_decode(&mut input),
                expected,
                "input: {}{}",
                hex::encode(orig),
                if let Ok(expected) = &expected {
                    format!("; expected: {}", hex::encode(crate::rlp_encode(expected)))
                } else {
                    String::new()
                }
            );

            if expected.is_ok() {
                assert_eq!(input, &[]);
            }
        }
    }

    #[test]
    fn rlp_strings() {
        check_decode::<Bytes, _>([
            (Ok(hex!("00")[..].to_vec().into()), &hex!("00")[..]),
            (
                Ok(hex!("6f62636465666768696a6b6c6d")[..].to_vec().into()),
                &hex!("8D6F62636465666768696A6B6C6D")[..],
            ),
            (Err(Error::UnexpectedList), &hex!("C0")[..]),
        ])
    }


    #[test]
    fn rlp_fixed_length() {
        check_decode([
            (Ok(hex!("6f62636465666768696a6b6c6d")), &hex!("8D6F62636465666768696A6B6C6D")[..]),
            (Err(Error::UnexpectedLength), &hex!("8C6F62636465666768696A6B6C")[..]),
            (Err(Error::UnexpectedLength), &hex!("8E6F62636465666768696A6B6C6D6E")[..]),
        ])
    }

    #[test]
    fn rlp_u64() {
        check_decode([
            (Ok(9_u64), &hex!("09")[..]),
            (Ok(0_u64), &hex!("80")[..]),
            (Ok(0x0505_u64), &hex!("820505")[..]),
            (Ok(0xCE05050505_u64), &hex!("85CE05050505")[..]),
            (Err(Error::Overflow), &hex!("8AFFFFFFFFFFFFFFFFFF7C")[..]),
            (Err(Error::InputTooShort), &hex!("8BFFFFFFFFFFFFFFFFFF7C")[..]),
            (Err(Error::UnexpectedList), &hex!("C0")[..]),
            (Err(Error::LeadingZero), &hex!("00")[..]),
            (Err(Error::NonCanonicalSingleByte), &hex!("8105")[..]),
            (Err(Error::LeadingZero), &hex!("8200F4")[..]),
            (Err(Error::NonCanonicalSize), &hex!("B8020004")[..]),
            (
                Err(Error::Overflow),
                &hex!("A101000000000000000000000000000000000000008B000000000000000000000000")[..],
            ),
        ])
    }

    #[test]
    fn rlp_vectors() {
        check_decode::<Vec<u64>, _>([
            (Ok(vec![]), &hex!("C0")[..]),
            (Ok(vec![0xBBCCB5_u64, 0xFFC0B5_u64]), &hex!("C883BBCCB583FFC0B5")[..]),
        ])
    }

    #[cfg(feature = "std")]
    #[test]
    fn rlp_ip() {
        use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

        let localhost4 = Ipv4Addr::new(127, 0, 0, 1);
        let localhost6 = localhost4.to_ipv6_mapped();
        let expected4 = &hex!("847F000001")[..];
        let expected6 = &hex!("9000000000000000000000ffff7f000001")[..];
        check_decode::<Ipv4Addr, _>([(Ok(localhost4), expected4)]);
        check_decode::<Ipv6Addr, _>([(Ok(localhost6), expected6)]);
        check_decode::<IpAddr, _>([
            (Ok(IpAddr::V4(localhost4)), expected4),
            (Ok(IpAddr::V6(localhost6)), expected6),
        ]);
    }

    #[test]
    fn malformed_rlp() {
        check_decode::<Bytes, _>([
            (Err(Error::InputTooShort), &hex!("C1")[..]),
            (Err(Error::InputTooShort), &hex!("D7")[..]),
        ]);
        check_decode::<[u8; 5], _>([
            (Err(Error::InputTooShort), &hex!("C1")[..]),
            (Err(Error::InputTooShort), &hex!("D7")[..]),
        ]);
        #[cfg(feature = "std")]
        check_decode::<std::net::IpAddr, _>([
            (Err(Error::InputTooShort), &hex!("C1")[..]),
            (Err(Error::InputTooShort), &hex!("D7")[..]),
        ]);
        check_decode::<Vec<u8>, _>([
            (Err(Error::InputTooShort), &hex!("C1")[..]),
            (Err(Error::InputTooShort), &hex!("D7")[..]),
        ]);
        check_decode::<String, _>([
            (Err(Error::InputTooShort), &hex!("C1")[..]),
            (Err(Error::InputTooShort), &hex!("D7")[..]),
        ]);
        check_decode::<String, _>([
            (Err(Error::InputTooShort), &hex!("C1")[..]),
            (Err(Error::InputTooShort), &hex!("D7")[..]),
        ]);
        check_decode::<u8, _>([(Err(Error::InputTooShort), &hex!("82")[..])]);
        check_decode::<u64, _>([(Err(Error::InputTooShort), &hex!("82")[..])]);
    }
}
