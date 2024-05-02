use alloy_rlp::{RlpDecodable, Decoder};


fn main() {

    let rlp = b"c88363617483646f67";
    let mut decoder = Decoder::new(rlp);
    
    //let options = Options::<String>::rlp_decode(&mut decoder).unwrap();
    //println!("decoder {:?}", decoder);



    // decode unsigned integer

    //decode signed integer

    //decode lists

    //decode strings

}