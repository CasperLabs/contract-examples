#![no_std]
#![feature(alloc)]

extern crate alloc;
extern crate common;

use alloc::collections::btree_map::BTreeMap;
use common::contract_api::{get_caller, store_function};
use common::value::account::PublicKey;

#[no_mangle]
pub extern "C" fn get_caller_ext() {
    // Assumes that will be called using account with
    // public key == 303030...[48u8; 32] in binary representation.
    // Will fail if we ever change that.
    let caller = get_caller();
    assert!(caller.is_some());
    let expected_caller = PublicKey::new([48u8; 32]);
    assert_eq!(caller.unwrap(), expected_caller);
}

#[no_mangle]
pub extern "C" fn call() {
    //  When in the base context there is no caller.
    assert_eq!(get_caller(), None);
    store_function("get_caller_ext", BTreeMap::new());
}
