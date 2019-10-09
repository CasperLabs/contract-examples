#![no_std]

extern crate alloc;

extern crate contract_ffi;

use alloc::string::String;
use alloc::vec::Vec;

use contract_ffi::contract_api::pointers::ContractPointer;
use contract_ffi::contract_api::{self, Error};
use contract_ffi::key::Key;
use contract_ffi::unwrap_or_revert::UnwrapOrRevert;
use contract_ffi::value::Value;

#[no_mangle]
pub extern "C" fn call() {
    let contract_key = contract_api::get_key("hello_name").unwrap_or_revert_with(Error::GetURef);
    let pointer = match contract_key {
        Key::Hash(hash) => ContractPointer::Hash(hash),
        _ => contract_api::revert(Error::UnexpectedKeyVariant),
    };
    let arg = ("World",);
    let result: String = contract_api::call_contract(pointer, &arg, &Vec::new());
    assert_eq!("Hello, World", result);

    //store the result at a uref so it can be seen as an effect on the global state
    let _uref = contract_api::new_turef(Value::String(result));
}
