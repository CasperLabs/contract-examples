#![no_std]

extern crate alloc;

extern crate contract_ffi;

use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

use contract_ffi::contract_api::{runtime, storage, Error};
use contract_ffi::unwrap_or_revert::UnwrapOrRevert;

fn hello_name(name: &str) -> String {
    let mut result = String::from("Hello, ");
    result.push_str(name);
    result
}

#[no_mangle]
pub extern "C" fn hello_name_ext() {
    let name: String = runtime::get_arg(0)
        .unwrap_or_revert_with(Error::MissingArgument)
        .unwrap_or_revert_with(Error::InvalidArgument);
    let y = hello_name(&name);
    runtime::ret(y, Vec::new());
}

#[no_mangle]
pub extern "C" fn call() {
    let pointer = storage::store_function("hello_name_ext", BTreeMap::new());
    runtime::put_key("hello_name", &pointer.into());
}
