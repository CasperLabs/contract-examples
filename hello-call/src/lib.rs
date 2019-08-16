#![no_std]
#![feature(alloc)]

extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

extern crate common;
use common::contract_api::pointers::ContractPointer;
use common::contract_api::{call_contract, get_uref, new_uref, revert};
use common::key::Key;
use common::value::Value;

#[no_mangle]
pub extern "C" fn call() {
    let pointer = if let Some(Key::Hash(hash)) = get_uref("hello_name") {
        ContractPointer::Hash(hash)
    } else {
        revert(66); // exit code is currently arbitrary
    };
    let arg = "World";
    let result: String = call_contract(pointer, &arg, &Vec::new());
    assert_eq!("Hello, World", result);

    //store the result at a uref so it can be seen as an effect on the global state
    let _uref = new_uref(Value::String(result));
}
