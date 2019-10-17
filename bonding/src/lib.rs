#![no_std]

#[macro_use]
extern crate alloc;

extern crate contract_ffi;

use contract_ffi::contract_api::{account, runtime, storage, system, Error, TURef};
use contract_ffi::key::Key;
use contract_ffi::unwrap_or_revert::UnwrapOrRevert;
use contract_ffi::value::uint::U512;

const BOND_METHOD_NAME: &str = "bond";
const POS_CONTRACT_NAME: &str = "pos";

// Bonding contract.
//
// Accepts bonding amount (of type `u64`) as first argument.
// Issues bonding request to the PoS contract.
#[no_mangle]
pub extern "C" fn call() {
    let pos_key = runtime::get_key(POS_CONTRACT_NAME).unwrap_or_revert_with(Error::GetKey);
    let pos_turef: TURef<Key> = pos_key
        .to_turef()
        .unwrap_or_revert_with(Error::UnexpectedKeyVariant);

    let pos_contract = storage::read(pos_turef)
        .unwrap_or_revert_with(Error::Read)
        .unwrap_or_revert_with(Error::ValueNotFound);
    let pos_pointer = pos_contract
        .to_c_ptr()
        .unwrap_or_revert_with(Error::UnexpectedKeyVariant);

    let source_purse = account::get_main_purse();
    let bonding_purse = system::create_purse();
    let bond_amount: U512 = runtime::get_arg::<u64>(0)
        .unwrap_or_revert_with(Error::MissingArgument)
        .unwrap_or_revert_with(Error::InvalidArgument)
        .into();

    system::transfer_from_purse_to_purse(source_purse, bonding_purse, bond_amount)
        .unwrap_or_revert();
    runtime::call_contract::<_, ()>(
        pos_pointer,
        &(BOND_METHOD_NAME, bond_amount, bonding_purse),
        &vec![Key::URef(bonding_purse.value())],
    );
}
