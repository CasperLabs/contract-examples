#![no_std]
#![feature(alloc)]

extern crate alloc;
use alloc::vec::Vec;

extern crate common;
use common::contract_api::pointers::ContractPointer;
use common::contract_api::{call_contract, get_uref, revert};
use common::key::Key;

#[no_mangle]
pub extern "C" fn call() {
    let pointer = if let Some(Key::Hash(hash)) = get_uref("counter") {
        ContractPointer::Hash(hash)
    } else {
        revert(66)
    };

    let arg = "inc";
    let _result: () = call_contract(pointer.clone(), &arg, &Vec::new());
    let _value: i32 = {
        let arg = "get";
        call_contract(pointer, &arg, &Vec::new())
    };
}
