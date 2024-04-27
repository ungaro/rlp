use alloy_rlp::{encode, encode_list, RlpDecodable, RlpEncodable, Error, Header};
use bytes::BufMut;

#[derive(Debug, PartialEq)]
enum FooBar {
    Foo(u64),
    Bar(u16, u64),
}

impl RlpEncodable for FooBar {
    fn rlp_encode(&self, out: &mut dyn BufMut) {
        match self {
            Self::Foo(x) => {
                let enc: [&dyn RlpEncodable; 2] = [&0u8, x];
                encode_list::<_, dyn RlpEncodable>(&enc, out);
            }
            Self::Bar(x, y) => {
                let enc: [&dyn RlpEncodable; 3] = [&1u8, x, y];
                encode_list::<_, dyn RlpEncodable>(&enc, out);
            }
        }
    }
}



impl RlpDecodable for FooBar {
    fn rlp_decode(data: &mut &[u8]) -> Result<Self, Error> {
        let mut payload = Header::decode_bytes(data, true)?;
        match u8::rlp_decode(&mut payload)? {
            0 => Ok(Self::Foo(u64::rlp_decode(&mut payload)?)),
            1 => Ok(Self::Bar(u16::rlp_decode(&mut payload)?, u64::rlp_decode(&mut payload)?)),
            _ => Err(Error::Custom("unknown type")),
        }
    }
}

fn main() {
    let val = FooBar::Foo(42);
    let out = encode(&val);
    assert_eq!(FooBar::rlp_decode(&mut out.as_ref()), Ok(val));

    let val = FooBar::Bar(7, 42);
    let out = encode(&val);
    assert_eq!(FooBar::rlp_decode(&mut out.as_ref()), Ok(val));

    println!("success!")
}
