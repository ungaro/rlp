use alloy_rlp::{RlpEncodable, Encoder};


fn main() {

    let rlp = b"c88363617483646f67";
    let mut decoder = Decoder::new(rlp);
    
    //let options = Options::<String>::rlp_decode(&mut decoder).unwrap();
    //println!("{:?}", options);



    // encode unsigned integer

    //encode signed integer

    //encode lists

    //encode strings

}