#![no_std]

#[macro_use]
extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

extern crate contract_ffi;
use contract_ffi::contract_api::pointers::TURef;
use contract_ffi::contract_api::*;
use contract_ffi::key::Key;
use contract_ffi::uref::URef;

fn get_list_key(name: &str) -> TURef<Vec<String>> {
    get_uref(name).and_then(Key::to_turef).unwrap()
}

fn update_list(name: String) {
    let list_key = get_list_key("list");
    let mut list = read(list_key.clone())
        .unwrap_or_else(|_| revert(Error::GetURef.into()))
        .unwrap_or_else(|| revert(Error::ValueNotFound.into()));
    list.push(name);
    write(list_key, list);
}

fn sub(name: String) -> Option<TURef<Vec<String>>> {
    if has_uref(&name) {
        None //already subscribed
    } else {
        let init_message = vec![String::from("Welcome!")];
        let new_key = new_turef(init_message);
        add_uref(&name, &new_key.clone().into());
        update_list(name);
        Some(new_key)
    }
}

fn publish(msg: String) {
    let curr_list = read(get_list_key("list"))
        .unwrap_or_else(|_| revert(Error::GetURef.into()))
        .unwrap_or_else(|| revert(Error::ValueNotFound.into()));
    for name in curr_list.iter() {
        let uref = get_list_key(name);
        let mut messages = read(uref.clone())
            .unwrap_or_else(|_| revert(Error::GetURef.into()))
            .unwrap_or_else(|| revert(Error::ValueNotFound.into()));
        messages.push(msg.clone());
        write(uref, messages);
    }
}

#[no_mangle]
pub extern "C" fn mailing_list_ext() {
    let method_name: String = get_arg(0);
    match method_name.as_str() {
        "sub" => match sub(get_arg(1)) {
            Some(turef) => {
                let extra_uref = URef::new(turef.addr(), turef.access_rights());
                let extra_urefs = vec![extra_uref];
                ret(&Some(Key::from(turef)), &extra_urefs);
            }
            _ => ret(&Option::<Key>::None, &Vec::new()),
        },
        //Note that this is totally insecure. In reality
        //the pub method would be only available under an
        //unforgable reference because otherwise anyone could
        //spam the mailing list.
        "pub" => {
            publish(get_arg(1));
        }
        _ => panic!("Unknown method name!"),
    }
}

#[no_mangle]
pub extern "C" fn call() {
    let init_list: Vec<String> = Vec::new();
    let list_key = new_turef(init_list);

    //create map of references for stored contract
    let mut mailing_list_urefs: BTreeMap<String, Key> = BTreeMap::new();
    let key_name = String::from("list");
    mailing_list_urefs.insert(key_name, list_key.into());

    let pointer = store_function("mailing_list_ext", mailing_list_urefs);
    add_uref("mailing", &pointer.into())
}
