#![no_std]
#![feature(alloc)]

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

extern crate common;
use common::contract_api::{add_uref, get_arg, ret, store_function};

fn hello_name(name: &str) -> String {
    let mut result = String::from("Hello, ");
    result.push_str(name);
    result
}

#[no_mangle]
pub extern "C" fn hello_name_ext() {
    let name: String = get_arg(0);
    let y = hello_name(&name);
    ret(&y, &Vec::new());
}

#[no_mangle]
pub extern "C" fn call() {
    let pointer = store_function("hello_name_ext", BTreeMap::new());
    add_uref("hello_name", &pointer.into());
}
