use alloy_rlp::{RlpEncodable, RlpDecodable, Decoder};

fn main() {

}


/*
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



fn generics() {
    trait LT<'a> {}

    #[derive(RlpEncodable, RlpDecodable, RlpMaxEncodedLen)]
    struct Generic<T, U: for<'a> LT<'a>, V: Default, const N: usize>(T, usize, U, V, [u8; N])
    where
        U: std::fmt::Display;

}




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

#[test]
fn flatten() {


    #[derive(RlpEncodable, RlpDecodable)]
    #[rlp(trailing)]
    struct Options2<T> {
        a: Option<T>,
        #[rlp(flatten)]
        b: Option<T>,
    }
}




#[test]
fn tag() {
    

/// This represents only the reserved `p2p` subprotocol messages.
#[derive(Debug, Clone, PartialEq, Eq, RlpEncodable, RlpDecodable)]
#[rlp(tagged)]
pub enum P2PMessage {
    /// The first packet sent over the connection, and sent once by both sides.
    #[rlp(tag="0x00")]
    Hello(HelloMessage),

    /// Inform the peer that a disconnection is imminent; if received, a peer should disconnect
    /// immediately.
    #[rlp(tag="0x01")]
    Disconnect(DisconnectReason),

    /// Requests an immediate reply of [`P2PMessage::Pong`] from the peer.
    #[rlp(tag="0x02")]
    Ping,

    /// Reply to the peer's [`P2PMessage::Ping`] packet.
    #[rlp(tag="0x03")]
    Pong,
}
}



#[test]
fn skip() {
    
    //should be enum

    #[derive(RlpEncodable, RlpDecodable)]
    #[rlp(tagged)]
    struct Options2<T> {
        a: Option<T>,
        #[rlp(skip)]
        b: Option<T>,
    }
}



*/