use alloy_rlp::{RlpEncodable, RlpDecodable, Decoder,rlp_encode,Header};
use hex::FromHex;
use hex_literal::hex;



fn main() -> Result<(), Box<dyn std::error::Error>>{
    println!("Hello, world! test1.rs");
    let rlp = hex!("cf846361747383646f6785686f727365");
    println!("fl0");

    let mut decoder = Decoder::new(&rlp[..],true)?;
    println!("DeCODER {:?}", decoder.payload_view);
    decoder.rlp_decode_raw();


    //let decoded = RlpDecodable::rlp_decode(&mut &rlp[..]).unwrap();
    //println!("decoded [] {:?}", decoded);

    //let decoded = &decoder.payload_view[..].rlp_decode();
    //let decoded = &rlp[..].rlp_decode();
    //let decoded = Header::decode_str( &mut &rlp[..])?;

    

    //let decoded = decoder.rlp_decode()?;
    
    //println!("decoded [] {:?}", decoded);

    let val = "cats";
    let out = rlp_encode(&val);
    println!("// cats // encoded: {:?}", out);

    let val = "dog";
    let out = rlp_encode(&val);
    println!("// dog // encoded: {:?}", out);

    let val = "horse";
    let out = rlp_encode(&val);
    println!("// horse // encoded: {:?}", out);

    let val = vec!["cats", "dog", "horse"];
    let out = rlp_encode(&val);
    println!("// [cats, dog, horse] // encoded: {:?}",hex::encode(&out));
    

    let val = "test str";
    let out = rlp_encode(&val);
    println!("// test str - 887465737420737472// encoded: {:?}", hex::encode(&out));



    let DATA: [u8; 9] = hex!("887465737420737472");

    let mut decoder = Decoder::new(&DATA[..],false)?;
    println!("DeCODER {:?}", decoder.payload_view);

    //let decoded = Header::decode_str(&mut decoder.payload_view).map(|x| x);
let decoded = decoder.get_next()?;

let remote_id = decoder.get_next()?.ok_or("error");



    //decoded?.rlp_decode(&mut &decoder.payload_view[..]);;
    println!("decoded_a [] {:?}", remote_id);

    //let data_rlp = DATA.rlp_decode(&mut &DATA[..]);

    println!("DATA: {:?}",DATA);
   // println!("data_rlp: {:?}",data_rlp);
/*    */
Ok(())
    //rlp encode '["cat", "dog"]' -> 0xc88363617483646f67
    //rlp decode 0xc88363617483646f67 -> ["cat","dog"]



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