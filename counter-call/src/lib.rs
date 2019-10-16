#![no_std]

extern crate alloc;

extern crate contract_ffi;

use alloc::vec::Vec;

use contract_ffi::contract_api::{runtime, ContractRef, Error};
use contract_ffi::key::Key;
use contract_ffi::unwrap_or_revert::UnwrapOrRevert;

#[no_mangle]
pub extern "C" fn call() {
    let contract_key = runtime::get_key("counter").unwrap_or_revert_with(Error::GetKey);
    let pointer = match contract_key {
        Key::Hash(hash) => ContractRef::Hash(hash),
        _ => runtime::revert(Error::UnexpectedKeyVariant),
    };

    let arg = ("inc",);
    runtime::call_contract::<_, ()>(pointer.clone(), &arg, &Vec::new());
    let _value: i32 = {
        let arg = ("get",);
        runtime::call_contract(pointer, &arg, &Vec::new())
    };
}
