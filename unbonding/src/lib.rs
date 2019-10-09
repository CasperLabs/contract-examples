#![no_std]

#[macro_use]
extern crate alloc;

extern crate contract_ffi;

use contract_ffi::contract_api::pointers::TURef;
use contract_ffi::contract_api::{self, Error};
use contract_ffi::key::Key;
use contract_ffi::unwrap_or_revert::UnwrapOrRevert;
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
    let pos_key = contract_api::get_key(POS_CONTRACT_NAME).unwrap_or_revert_with(Error::GetURef);
    let pos_turef: TURef<Key> = pos_key
        .to_turef()
        .unwrap_or_revert_with(Error::UnexpectedKeyVariant);

    let pos_contract = contract_api::read(pos_turef)
        .unwrap_or_revert_with(Error::Read)
        .unwrap_or_revert_with(Error::ValueNotFound);
    let pos_pointer = pos_contract
        .to_c_ptr()
        .unwrap_or_revert_with(Error::UnexpectedKeyVariant);

    let unbond_amount: Option<U512> = contract_api::get_arg::<Option<u64>>(0)
        .unwrap_or_revert_with(Error::MissingArgument)
        .unwrap_or_revert_with(Error::InvalidArgument)
        .map(Into::into);

    contract_api::call_contract(pos_pointer, &(UNBOND_METHOD_NAME, unbond_amount), &vec![])
}
