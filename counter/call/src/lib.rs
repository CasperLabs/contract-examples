#![no_std]
#![feature(alloc)]

#[macro_use]
extern crate alloc;
use alloc::vec::Vec;

extern crate common;
use common::bytesrepr::ToBytes;
use common::ext::call_contract;
use common::key::Key;

#[no_mangle]
pub extern "C" fn call() {
    //This hash comes from blake2b256( [0;32] ++ [0;8] ++ [0;4] )
    let hash = Key::Hash([
        164, 102, 153, 51, 236, 214, 169, 167, 126, 44, 250, 247, 179, 214, 203, 229, 239, 69, 145, 25, 5, 153, 113, 55, 255, 188, 176, 201, 7, 4, 42, 100
    ]);
    let arg = "inc";
    let args = vec![arg.to_bytes()];
    let _result: () = call_contract(&hash, &args, &Vec::new());
    let value: i32 = {
        let arg = "get";
        let args = vec![arg.to_bytes()];
        call_contract(&hash, &args, &Vec::new())
    };
    assert_eq!(value, 1);
}
