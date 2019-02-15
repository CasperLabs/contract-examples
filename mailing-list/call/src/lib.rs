#![no_std]
#![feature(alloc)]

#[macro_use]
extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

extern crate common;
use common::bytesrepr::ToBytes;
use common::ext::*;
use common::key::Key;
use common::value::Value;

#[no_mangle]
pub extern "C" fn call() {
    //This hash comes from blake2b256( [0;32] ++ [0;8] ++ [0;4] )
    let hash = Key::Hash([
        164, 102, 153, 51, 236, 214, 169, 167, 126, 44, 250, 247, 179, 214, 203, 229, 239, 69, 145, 25, 5, 153, 113, 55, 255, 188, 176, 201, 7, 4, 42, 100
    ]);
    let method = "sub";
    let name = "CasperLabs";
    let args = vec![method.to_bytes(), name.to_bytes()];
    let maybe_sub_key: Option<Key> = call_contract(&hash, &args, &Vec::new());
    let sub_key = maybe_sub_key.unwrap();

    let key_name = "mail_feed";
    add_uref(key_name, &sub_key);
    assert_eq!(sub_key, get_uref(key_name));

    let method = "pub";
    let message = "Hello, World!";
    let args = vec![method.to_bytes(), message.to_bytes()];
    let _result: () = call_contract(&hash, &args, &Vec::new());

    let messages = read(&sub_key);

    assert_eq!(
        Value::ListString(vec![String::from("Welcome!"), String::from(message)]),
        messages
    );
}
