use alloy_rlp::{RlpEncodable, RlpDecodable, Decoder,rlp_encode};


fn main() {
    println!("Hello, world! 02-encode-tx.rs");
    let rlp = b"c88363617483646f67";
    let mut decoder = Decoder::new(rlp);
    

    let val = "5";
    let out = rlp_encode(&val);
    println!("encoded: {:?}", out);

    let val = "[5]";
    let out = rlp_encode(&val);
    println!("encoded: {:?}", out);

    let val = vec!["cat", "dog"];
    let out = rlp_encode(&val);
    println!("encoded: {:?}", out);
    
    //rlp encode '["cat", "dog"]' -> 0xc88363617483646f67


dbg!("encoded: {:?}", out);

/*

rlp encode '5' -> 0x05
rlp encode '[5]' -> 0xc105
rlp encode '["cat", "dog"]' -> 0xc88363617483646f67
rlp decode 0xc88363617483646f67 -> ["cat","dog"]

RLP.encode("0x12345678");
// '0x8412345678'

RLP.encode([ "0x12345678" ]);
// '0xc58412345678'

RLP.encode([ new Uint8Array([ 0x12, 0x34, 0x56, 0x78 ]) ]);
// '0xc58412345678'

RLP.encode([ [ "0x42", [ "0x43" ] ], "0x12345678", [ ] ]);
// '0xcac342c1438412345678c0'

RLP.encode([ ]);
// '0xc0'
*/



    //let options = Options::<String>::rlp_decode(&mut decoder).unwrap();
    //println!("{:?}", options);



    // encode unsigned integer

    //encode signed integer

    //encode lists

    //encode strings

}