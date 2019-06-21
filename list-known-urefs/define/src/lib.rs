#![no_std]
#![feature(alloc)]

extern crate alloc;
extern crate common;

use alloc::borrow::ToOwned;
use alloc::collections::btree_map::BTreeMap;
use alloc::string::String;
use common::contract_api::{add_uref, list_known_urefs, store_function, new_uref, get_uref};
use common::key::Key;
use common::value::Value;
use core::iter;

#[no_mangle]
pub extern "C" fn list_known_urefs_ext() {
    let passed_in_uref = get_uref("Foo");
    let uref = new_uref(Value::String("Test".to_owned()));
    add_uref("Bar", &uref.clone().into());
    let contracts_known_urefs = list_known_urefs();
    let expected_urefs: BTreeMap<String, Key> = {
        let mut tmp = BTreeMap::new();
        tmp.insert("Bar".to_owned(), uref.into());
        tmp.insert("Foo".to_owned(), passed_in_uref);
        tmp
    };
    // Test that `list_known_urefs` returns correct value when in the subcall (contract).
    assert_eq!(expected_urefs, contracts_known_urefs);
}

#[no_mangle]
pub extern "C" fn call() {
    let uref = new_uref(Value::Int32(1));
    add_uref("Foo", &uref.clone().into());
    let accounts_known_urefs = list_known_urefs();
    let expected_urefs: BTreeMap<String, Key> =
        iter::once(("Foo".to_owned(), uref.into())).collect();
    // Test that `list_known_urefs` returns correct value when called in the context of an account.
    // Store `list_known_urefs_ext` to be called in the `call` part of this contract.
    // We don't have to  pass `expected_urefs` to exercise this function but
    // it adds initial known urefs to the state of the contract.
    store_function("list_known_urefs_ext", expected_urefs);
}
