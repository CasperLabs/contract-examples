#![no_std]
#![feature(alloc)]

#[macro_use]
extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

extern crate common;
use common::contract_api::*;
use common::contract_api::pointers::UPointer;
use common::key::Key;

fn get_list_key(name: &str) -> UPointer<Vec<String>> {
    get_uref(name).to_u_ptr().unwrap()
}

fn update_list(name: String) {
    let list_key = get_list_key("list");
    let mut list = read(list_key.clone());
    list.push(name);
    write(list_key, list);
}

fn sub(name: String) -> Option<UPointer<Vec<String>>> {
    if has_uref(&name) {
        None //already subscribed
    } else {
        let init_message = vec![String::from("Welcome!")];
        let new_key = new_uref(init_message);
        add_uref(&name, &new_key.clone().into());
        update_list(name);
        Some(new_key)
    }
}

fn publish(msg: String) {
    let curr_list = read(get_list_key("list"));
    for name in curr_list.iter() {
        let uref = get_list_key(name);
        let mut messages = read(uref.clone());
        messages.push(msg.clone());
        write(uref, messages);
    }
}

#[no_mangle]
pub extern "C" fn mailing_list_ext() {
    let method_name: String = get_arg(0);
    match method_name.as_str() {
        "sub" => match sub(get_arg(1)).map(Key::from) {
            Some(key) => {
                let extra_urefs = vec![key];
                ret(&Some(key), &extra_urefs);
            }
            none => ret(&none, &Vec::new()),
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
    let list_key = new_uref(init_list);

    //create map of references for stored contract
    let mut mailing_list_urefs: BTreeMap<String, Key> = BTreeMap::new();
    let key_name = String::from("list");
    mailing_list_urefs.insert(key_name, list_key.into());

    let _hash = store_function("mailing_list_ext", mailing_list_urefs);
}
