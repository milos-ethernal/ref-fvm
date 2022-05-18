#![no_main]

use std::io::Cursor;

use common_fuzz::cbor::Payload;
use fvm_ipld_encoding as encoding;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let p = encoding::from_reader::<Payload, _>(Cursor::new(data));
    if p.is_err() {
        return;
    }
    let p = p.unwrap();
    //if p.bytes.len() > 128 << 20 {
    //    panic!("too large array {}", p.bytes.len())
    //}

    let out = encoding::to_vec(&p).expect("decoded payload must be possible to encode");

    let p2 = encoding::from_reader::<Payload, _>(Cursor::new(&out))
        .expect("everything that encodes must decode");
    let out2 = encoding::to_vec(&p2).expect("decoded payload must be possible to encode2");
    if !out.eq(&out2) {
        panic!("repeated encodings must be stable");
    }
});