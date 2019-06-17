#![no_std]
#![feature(alloc)]

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

extern crate common;
use common::contract_api::*;
use common::contract_api::pointers::UPointer;
use common::key::Key;

#[no_mangle]
pub extern "C" fn counter_ext() {
    let i_key: UPointer<i32> = get_uref("count").to_u_ptr().unwrap();
    let method_name: String = get_arg(0);
    match method_name.as_str() {
        "inc" => add(i_key, 1),
        "get" => {
            let result = read(i_key);
            ret(&result, &Vec::new());
        }
        _ => panic!("Unknown method name!"),
    }
}

#[no_mangle]
pub extern "C" fn call() {
    let counter_local_key = new_uref(0); //initialize counter

    //create map of references for stored contract
    let mut counter_urefs: BTreeMap<String, Key> = BTreeMap::new();
    let key_name = String::from("count");
    counter_urefs.insert(key_name, counter_local_key.into());

    let hash = store_function("counter_ext", counter_urefs);
    add_uref("counter", &hash.into());
}
