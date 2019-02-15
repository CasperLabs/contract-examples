#![no_std]
#![feature(alloc)]

#[macro_use]
extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

extern crate common;
use common::ext::*;
use common::key::Key;
use common::value::Value;

fn curr_list() -> Vec<String> {
    let names_list_key = get_uref("list");

    if let Value::ListString(list) = read(&names_list_key) {
        list
    } else {
        panic!("A list of strings is not found at the list key!")
    }
}

fn update_list(name: String) {
    let names_list_key = get_uref("list");
    let mut list = curr_list();
    list.push(name);
    write(&names_list_key, &Value::ListString(list));
}

fn sub(name: String) -> Option<Key> {
    if has_uref(&name) {
        None //already subscribed
    } else {
        let new_key = new_uref();
        let init_message = Value::ListString(vec![String::from("Welcome!")]);
        add_uref(&name, &new_key);
        write(&new_key, &init_message);
        update_list(name);
        Some(new_key)
    }
}

fn publish(msg: String) {
    for name in curr_list().iter() {
        let uref = get_uref(name);
        if let Value::ListString(mut messages) = read(&uref) {
            messages.push(msg.clone());
            write(&uref, &Value::ListString(messages));
        }
    }
}

#[no_mangle]
pub extern "C" fn mailing_list_ext() {
    let method_name: String = get_arg(0);
    match method_name.as_str() {
        "sub" => match sub(get_arg(1)) {
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
    let list_key = new_uref();
    write(&list_key, &Value::ListString(Vec::new()));

    //create map of references for stored contract
    let mut mailing_list_urefs: BTreeMap<String, Key> = BTreeMap::new();
    let key_name = String::from("list");
    mailing_list_urefs.insert(key_name, list_key);

    let _hash = store_function("mailing_list_ext", mailing_list_urefs);
}
