#![no_std]
#![feature(alloc)]

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

extern crate common;
use common::ext::*;
use common::key::Key;
use common::value::Value;

fn inc(uref: &Key) {
    let one = Value::Int32(1);
    add(uref, &one);
}

fn get(uref: &Key) -> i32 {
    if let Value::Int32(i) = read(uref) {
        i
    } else {
        0
    }
}

#[no_mangle]
pub extern "C" fn counter_ext() {
    let i_key: Key = get_uref("count");
    let method_name: String = get_arg(0);
    match method_name.as_str() {
        "inc" => inc(&i_key),
        "get" => {
            let result = get(&i_key);
            ret(&result, &Vec::new());
        }
        _ => panic!("Unknown method name!"),
    }
}

#[no_mangle]
pub extern "C" fn call() {
    let counter_local_key = new_uref();
    write(&counter_local_key, &Value::Int32(0)); //initialize counter

    //create map of references for stored contract
    let mut counter_urefs: BTreeMap<String, Key> = BTreeMap::new();
    let key_name = String::from("count");
    counter_urefs.insert(key_name, counter_local_key);

    let _hash = store_function("counter_ext", counter_urefs);
}
