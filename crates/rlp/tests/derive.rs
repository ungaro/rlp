#![cfg(feature = "derive")]

use alloy_rlp::{RlpDecodable, *};

#[test]
fn simple_derive() {
    #[derive(RlpEncodable, RlpDecodable, RlpMaxEncodedLen, PartialEq, Debug)]
    struct MyThing(#[rlp] [u8; 12]);

    let thing = MyThing([0; 12]);

    // roundtrip fidelity
    let mut buf = Vec::new();
    thing.rlp_encode(&mut buf);
    let decoded = MyThing::rlp_decode(&mut buf.as_slice()).unwrap();
    assert_eq!(thing, decoded);

    // does not panic on short input
    assert_eq!(Err(Error::InputTooShort), MyThing::rlp_decode(&mut [0x8c; 11].as_ref()))
}

#[test]
fn wrapper() {
    #[derive(RlpEncodableWrapper, RlpDecodableWrapper, RlpMaxEncodedLen, PartialEq, Debug)]
    struct Wrapper([u8; 8]);

    #[derive(RlpEncodableWrapper, RlpDecodableWrapper, PartialEq, Debug)]
    struct ConstWrapper<const N: usize>([u8; N]);
}

#[test]
fn generics() {
    trait LT<'a> {}

    #[derive(RlpEncodable, RlpDecodable, RlpMaxEncodedLen)]
    struct Generic<T, U: for<'a> LT<'a>, V: Default, const N: usize>(T, usize, U, V, [u8; N])
    where
        U: std::fmt::Display;

    #[derive(RlpEncodableWrapper, RlpDecodableWrapper, RlpMaxEncodedLen)]
    struct GenericWrapper<T>(T)
    where
        T: Sized;
}




#[derive(Debug, RlpDecodable,)]
struct SimpleTransfer {
    nonce: u8,
    value: u64,
    to: [u8; 20],
}

#[derive(Debug, RlpDecodable,)]
struct ContractCreation {
    nonce: u8,
    value: u64,
    init: Vec<u8>,
}


trait Transaction {
    fn rlp_decode(decoder: &mut &[u8]) -> Result<Self> where Self: Sized;
}

fn decode_transaction(decoder: &mut Decoder<'_>) -> Result<Box<dyn RlpDecodable>> {
    //let tx_type = decoder.get_next()?;
    let mut payload = Header::decode_bytes(decoder, false)?;
    let tx = u8::rlp_decode(&mut payload)?;
    Ok(tx)

    /*
    match tx_type {
        Some(0x01) => Ok(Box::new(SimpleTransfer::rlp_decode(decoder.get_next())?)),
        Some(0x02) => Ok(Box::new(ContractCreation::rlp_decode(decoder.get_next())?)),
        _ => Err(Error::NoVariant),
    }*/

}


#[test]
fn test1() {

/*
rlp encode '5' -> 0x05
rlp encode '[5]' -> 0xc105
rlp encode '["cat", "dog"]' -> 0xc88363617483646f67
rlp decode 0xc88363617483646f67 -> ["cat","dog"]



*/
let catdog = Decoder::new(["cat", "dog"].as_bytes());


let decoded = catdog::rlp_decode::<Vec<String>>().unwrap();
/*
  println!("Simple rlp decoding check");

  let data = [0x01, 0x05, 0x64, /* address */ 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
  let mut decoder = Decoder::new(&data);
  let tx = decode_transaction(&mut decoder)?;
  */
  println!("Decoded transaction: {:?}", decoded);
}



/*
impl RlpDecodable for SimpleTransfer {
    fn rlp_decode(decoder:&mut &[u8]) -> Result<Self> {
        let mut payload = Header::decode_bytes(decoder, true)?;
        let nonce = u8::rlp_decode(&mut payload)?;
        let value = u64::rlp_decode(&mut payload)?;
        let to = <[u8; 20]>::rlp_decode(&mut payload)?;
        Ok(Self { nonce, value, to })
    }
}

impl RlpDecodable for ContractCreation {
    fn rlp_decode(decoder: &mut &[u8]) -> Result<Self> {
        let mut payload = Header::decode_bytes(decoder, true)?;
        let nonce = u8::rlp_decode(&mut payload)?;
        let value = u64::rlp_decode(&mut payload)?;
        let init = Vec::<u8>::rlp_decode(&mut payload)?;
        Ok(Self { nonce, value, init })
    }
}

*/
/*

fn decode_transaction(decoder: &mut Decoder<'_>) -> Result<Box<dyn RlpDecodable>> {
    let tx_type = decoder.get_next()?;
    match tx_type {
        Some(0x01) => Ok(Box::new(SimpleTransfer::rlp_decode(decoder.get_next())?)),
        Some(0x02) => Ok(Box::new(ContractCreation::rlp_decode(decoder.get_next())?)),
        _ => Err(Error::NoVariant),
    }
}

*/

/*
#[test]
fn eip2718() {



    let data = [0x01, 0x05, 0x64, /* address */ 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    let mut decoder = Decoder::new(&data);
    let tx = decode_transaction(&mut decoder)?;
    println!("Decoded transaction: {:?}", tx);

/*

    check_decode::<Bytes, _>([
        (Ok(hex!("00")[..].to_vec().into()), &hex!("00")[..]),
        (
            Ok(hex!("6f62636465666768696a6b6c6d")[..].to_vec().into()),
            &hex!("8D6F62636465666768696A6B6C6D")[..],
        ),
        (Err(Error::UnexpectedList), &hex!("C0")[..]),
    ])
*/



}

*/




#[test]
fn opt() {
    #[derive(RlpEncodable, RlpDecodable)]
    #[rlp(trailing)]
    struct Options<T>(Option<Vec<T>>);

    #[derive(RlpEncodable, RlpDecodable)]
    #[rlp(trailing)]
    struct Options2<T> {
        a: Option<T>,
        #[rlp(default)]
        b: Option<T>,
    }
}
