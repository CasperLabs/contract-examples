#![no_std]

#[macro_use]
extern crate alloc;
extern crate contract_ffi;

use contract_ffi::contract_api::pointers::TURef;
use contract_ffi::contract_api::{self, Error};
use contract_ffi::key::Key;
use contract_ffi::value::uint::U512;

const POS_CONTRACT_NAME: &str = "pos";
const UNBOND_METHOD_NAME: &str = "unbond";

// Unbonding contract.
//
// Accepts unbonding amount (of type `Option<u64>`) as first argument.
// Unbonding with `None` unbonds all stakes in the PoS contract.
// Otherwise (`Some<u64>`) unbonds with part of the bonded stakes.
#[no_mangle]
pub extern "C" fn call() {
    let pos_public: TURef<Key> = unwrap_or_revert(
        contract_api::get_uref(POS_CONTRACT_NAME).and_then(Key::to_turef),
        66,
    );
    let pos_contract: Key = contract_api::read(pos_public)
        .unwrap_or_else(|_| contract_api::revert(Error::GetURef.into()))
        .unwrap_or_else(|| contract_api::revert(Error::ValueNotFound.into()));
    let pos_pointer = unwrap_or_revert(pos_contract.to_c_ptr(), 77);

    let unbond_amount: Option<U512> = contract_api::get_arg::<Option<u64>>(0).map(U512::from);

    contract_api::call_contract(pos_pointer, &(UNBOND_METHOD_NAME, unbond_amount), &vec![])
}

fn unwrap_or_revert<T>(option: Option<T>, code: u32) -> T {
    if let Some(value) = option {
        value
    } else {
        contract_api::revert(code)
    }
}
