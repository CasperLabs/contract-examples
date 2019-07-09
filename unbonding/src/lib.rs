#![no_std]
#![feature(alloc)]

#[macro_use]
extern crate alloc;
extern crate common;

use common::contract_api;
use common::contract_api::pointers::UPointer;
use common::key::Key;
use common::value::uint::U512;

const POS_CONTRACT_NAME: &str = "pos";

#[no_mangle]
pub extern "C" fn call() {
    let pos_public: UPointer<Key> = contract_api::get_uref(POS_CONTRACT_NAME).to_u_ptr().unwrap();
    let pos_contract: Key = contract_api::read(pos_public);
    let pos_pointer = pos_contract.to_c_ptr().unwrap();

    // Put the desired unbonding amount here.
    // None means that the complete stake will be unbonded, while
    // Some(x) means an amount x will be unbonded while the rest
    // remains.
    // Note the type of the argument is correct, do not change
    // this when changing the value.
    let unbond_amount: Option<U512> = None;

    let _result: () = contract_api::call_contract(pos_pointer, &("unbond", unbond_amount), &vec![]);
}
