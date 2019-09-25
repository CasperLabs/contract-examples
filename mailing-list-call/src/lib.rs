#![no_std]

#[macro_use]
extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

extern crate contract_ffi;
use contract_ffi::contract_api::pointers::*;
use contract_ffi::contract_api::*;
use contract_ffi::key::Key;

#[no_mangle]
pub extern "C" fn call() {
    let pointer = if let Some(Key::Hash(hash)) = get_uref("mailing") {
        ContractPointer::Hash(hash)
    } else {
        revert(66); // exit code is currently arbitrary
    };

    let method = "sub";
    let name = "CasperLabs";
    let args = (method, name);
    let maybe_sub_key: Option<Key> = call_contract(pointer.clone(), &args, &Vec::new());
    let sub_key = maybe_sub_key.unwrap();

    let key_name = "mail_feed";
    add_uref(key_name, &sub_key);
    assert_eq!(Some(sub_key), get_uref(key_name));

    let method = "pub";
    let message = "Hello, World!";
    let args = (method, message);
    let _result: () = call_contract(pointer, &args, &Vec::new());

    let list_key: TURef<Vec<String>> = sub_key.to_turef().unwrap();
    let messages = match read(list_key) {
        Ok(Some(messages)) => messages,
        Ok(None) => revert(Error::ValueNotFound.into()),
        Err(_) => revert(Error::Read.into()),
    };

    assert_eq!(
        vec![String::from("Welcome!"), String::from(message)],
        messages
    );
}
