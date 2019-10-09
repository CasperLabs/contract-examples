#![no_std]

#[macro_use]
extern crate alloc;

extern crate contract_ffi;

use alloc::string::String;
use alloc::vec::Vec;

use contract_ffi::contract_api::pointers::{ContractPointer, TURef};
use contract_ffi::contract_api::{self, Error};
use contract_ffi::key::Key;
use contract_ffi::unwrap_or_revert::UnwrapOrRevert;

#[no_mangle]
pub extern "C" fn call() {
    let contract_key = contract_api::get_key("mailing").unwrap_or_revert_with(Error::GetURef);
    let pointer = match contract_key {
        Key::Hash(hash) => ContractPointer::Hash(hash),
        _ => contract_api::revert(Error::UnexpectedKeyVariant),
    };

    let method = "sub";
    let name = "CasperLabs";
    let args = (method, name);
    let maybe_sub_key: Option<Key> =
        contract_api::call_contract(pointer.clone(), &args, &Vec::new());
    let sub_key = maybe_sub_key.unwrap_or_revert();

    let key_name = "mail_feed";
    contract_api::put_key(key_name, &sub_key);
    assert_eq!(Some(sub_key), contract_api::get_key(key_name));

    let method = "pub";
    let message = "Hello, World!";
    let args = (method, message);
    contract_api::call_contract::<_, ()>(pointer, &args, &Vec::new());

    let list_key: TURef<Vec<String>> = sub_key
        .to_turef()
        .unwrap_or_revert_with(Error::UnexpectedKeyVariant);
    let messages = contract_api::read(list_key)
        .unwrap_or_revert_with(Error::Read)
        .unwrap_or_revert_with(Error::ValueNotFound);

    assert_eq!(
        vec![String::from("Welcome!"), String::from(message)],
        messages
    );
}
