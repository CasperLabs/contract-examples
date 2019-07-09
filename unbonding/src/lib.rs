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
const UNBOND_METHOD_NAME: &str = "unbond";

#[no_mangle]
pub extern "C" fn call() {
    let pos_public: UPointer<Key> = unwrap_or_revert(contract_api::get_uref(POS_CONTRACT_NAME).to_u_ptr(), 66);
    let pos_contract: Key = contract_api::read(pos_public);
    let pos_pointer = unwrap_or_revert(pos_contract.to_c_ptr(), 77);

    // Put the desired unbonding amount here.
    // None means that the complete stake will be unbonded, while
    // Some(x) means an amount x will be unbonded while the rest
    // remains.
    // Note the type of the argument is correct, do not change
    // this when changing the value.
    let unbond_amount: Option<U512> = None;

    contract_api::call_contract(pos_pointer, &(UNBOND_METHOD_NAME, unbond_amount), &vec![])
}


fn unwrap_or_revert<T>(option: Option<T>, code: u32) -> T {
    if let Some(value) = option {
        value
    } else {
        contract_api::revert(code)
    }
}
