#![no_std]

extern crate alloc;

extern crate contract_ffi;

use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

use contract_ffi::contract_api::pointers::TURef;
use contract_ffi::contract_api::{self, Error};
use contract_ffi::key::Key;
use contract_ffi::unwrap_or_revert::UnwrapOrRevert;

#[no_mangle]
pub extern "C" fn counter_ext() {
    let i_key = contract_api::get_key("count").unwrap_or_revert_with(Error::GetURef);
    let i_turef: TURef<i32> = i_key
        .to_turef()
        .unwrap_or_revert_with(Error::UnexpectedKeyVariant);

    let method_name: String = contract_api::get_arg(0)
        .unwrap_or_revert_with(Error::MissingArgument)
        .unwrap_or_revert_with(Error::InvalidArgument);
    match method_name.as_str() {
        "inc" => contract_api::add(i_turef, 1),
        "get" => {
            let result = contract_api::read(i_turef)
                .unwrap_or_revert_with(Error::Read)
                .unwrap_or_revert_with(Error::ValueNotFound);
            contract_api::ret(&result, &Vec::new());
        }
        _ => panic!("Unknown method name!"),
    }
}

#[no_mangle]
pub extern "C" fn call() {
    let counter_local_key = contract_api::new_turef(0); //initialize counter

    //create map of references for stored contract
    let mut counter_urefs: BTreeMap<String, Key> = BTreeMap::new();
    let key_name = String::from("count");
    counter_urefs.insert(key_name, counter_local_key.into());

    let pointer = contract_api::store_function("counter_ext", counter_urefs);
    contract_api::put_key("counter", &pointer.into());
}
