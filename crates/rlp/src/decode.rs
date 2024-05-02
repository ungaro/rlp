pub use crate::encode::RlpLength;
use crate::{Error, Header, Result, RlpEncodable, EMPTY_LIST_CODE, EMPTY_STRING_CODE};
use bytes::{Buf, BufMut, Bytes, BytesMut};
use core::{
    borrow::BorrowMut,
    hint::unreachable_unchecked,
    marker::{PhantomData, PhantomPinned},
};
use std::default::Default;
use std::fmt;

/// A type that can be decoded from an Decoder blob.
pub trait RlpDecodable: Sized {
    /// Decodes the blob into the appropriate type. `buf` must be advanced past
    /// the decoded object.
    fn rlp_decode(buf: &mut &[u8]) -> Result<Self>;
    fn rlp_decode_raw(buf: &mut &[u8]) -> Result<Self>
    where
        Self: Sized,
    {
        Self::rlp_decode(buf)
    }
}

/// An active Decoder decoder, with a specific slice of a payload.
#[derive(Default)]
pub struct Decoder<'de> {
    pub payload_view: &'de [u8],
   // header: Option<Header>,

}

/*
impl<'a> RlpDecodable for Decoder<'de> {
    #[inline]
    fn rlp_decode(buf: &'de mut &'de[u8]) -> Result<Self> {
        Ok(Self::new(*buf, false)?)
    }
}
*/
impl<'de> Decoder<'de> {
    /// Instantiate an RLP decoder with a payload slice.
    pub fn new(mut payload: &'de [u8],is_list:bool) -> Result<Self> {
        
        let payload_view = Header::decode_bytes(&mut payload,is_list)?;
        /*Ok(Self { payload_view: payload, header: Some(header) })
*/
    //    let payload_view = Header::decode(&mut payload)?;
      //  Ok(Self { payload_view })

Ok(Self{payload_view})
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





    #[inline]
    
        pub fn rlp_decode_raw<'a>(&mut self) -> Result<&'de [u8]> {
            println!("rlp_decode_raw0: {:?}",self.payload_view);
            let data = Header::decode_bytes(&mut self.payload_view, false)?;


            let data = Header::decode_str(&mut self.payload_view)?;
            //println!("rlp_decode_raw2: {:?}", hex!(&data()));
            println!("rlp_decode_raw2: {:?}", &data);

            let data = Header::decode_bytes(&mut self.payload_view, false)?;
            println!("rlp_decode_raw2: {:?}", &data);

            let data = Header::decode_bytes(&mut self.payload_view, false)?;
            println!("rlp_decode_raw2: {:?}", &data);
            Ok(data)
    }

    /// Decodes a string slice from the given buffer, advancing it.
    ///
    /// # Errors
    ///
    /// Returns an error if the buffer is too short or the header is invalid.
    #[inline]
    pub fn decode_str<'a>(buf: &mut &'a [u8]) -> Result<&'a str> {
        let bytes = Header::decode_bytes(buf, false)?;
        core::str::from_utf8(bytes).map_err(|_| Error::Custom("invalid string"))
    }

    /*
        /// Encodes the header into the `out` buffer.
        #[inline]
        pub fn encode(&self, out: &mut dyn BufMut) {
            if self.header.as_ref()?.payload_length < 56 {
                let code = if self.header.unwrap().list { EMPTY_LIST_CODE } else { EMPTY_STRING_CODE };
                out.put_u8(code + self.header.unwrap().payload_length as u8);
            } else {
                let len_be;
                let len_be =
                    crate::encode::to_be_bytes_trimmed!(len_be, self.header.unwrap().payload_length);
                let code = if self.header.unwrap().list { 0xF7 } else { 0xB7 };
                out.put_u8(code + len_be.len() as u8);
                out.put_slice(len_be);
            }
        }
    */

}


impl<'de> fmt::Debug for Decoder<'de> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Decoder")
         .field("payload_view", &&self.payload_view)  // Shows the slice as an array of bytes
         //.field("header", &self.header)
         .finish()
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

/*
impl RlpLength for &[u8] {
    fn rlp_len_raw(&self) -> usize {
        self.len()
    }
}
*/
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
/*
impl RlpLength for alloc::string::String {
    #[inline]
    fn rlp_len_raw(&self) -> usize {
        self.len()
    }
}
*/

impl<T: RlpDecodable + RlpEncodable> RlpLength for alloc::vec::Vec<T> {
    #[inline]
    fn rlp_len_raw(&self) -> usize {
        0
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

/*
        impl<$($gen)*> RlpLength for $t {
            #[inline]
            fn rlp_len_raw(&self) -> usize {
                <$t1 as RlpLength>::rlp_len_raw()
            }
        }
       */
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

/*
// Ensure all RlpEncodable and RlpDecodable types also implement RlpLength
impl<T: RlpEncodable + RlpDecodable> RlpLength for T {
    fn rlp_len_raw(&self) -> usize {
        return 0
    }
}
*/

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
    use crate::{RlpDecodable, RlpEncodable};
    use core::fmt::Debug;
    use hex_literal::hex;

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
