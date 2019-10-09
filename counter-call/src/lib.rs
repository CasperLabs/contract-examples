#![no_std]

extern crate alloc;

extern crate contract_ffi;

use alloc::vec::Vec;

use contract_ffi::contract_api::pointers::ContractPointer;
use contract_ffi::contract_api::{self, Error};
use contract_ffi::key::Key;
use contract_ffi::unwrap_or_revert::UnwrapOrRevert;

#[no_mangle]
pub extern "C" fn call() {
    let contract_key = contract_api::get_key("counter").unwrap_or_revert_with(Error::GetURef);
    let pointer = match contract_key {
        Key::Hash(hash) => ContractPointer::Hash(hash),
        _ => contract_api::revert(Error::UnexpectedKeyVariant),
    };

    let arg = ("inc",);
    contract_api::call_contract::<_, ()>(pointer.clone(), &arg, &Vec::new());
    let _value: i32 = {
        let arg = ("get",);
        contract_api::call_contract(pointer, &arg, &Vec::new())
    };
}
