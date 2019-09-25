#![no_std]

#[macro_use]
extern crate alloc;
extern crate contract_ffi;

use contract_ffi::contract_api::pointers::TURef;
use contract_ffi::contract_api::{self, Error, PurseTransferResult};
use contract_ffi::key::Key;
use contract_ffi::value::uint::U512;

const BOND_METHOD_NAME: &str = "bond";
const POS_CONTRACT_NAME: &str = "pos";

// Bonding contract.
//
// Accepts bonding amount (of type `u64`) as first argument.
// Issues bonding request to the PoS contract.
#[no_mangle]
pub extern "C" fn call() {
    let pos_public: TURef<Key> = unwrap_or_revert(
        contract_api::get_uref(POS_CONTRACT_NAME).and_then(Key::to_turef),
        66,
    );
    let pos_contract: Key = match contract_api::read(pos_public) {
        Ok(Some(contract)) => contract,
        Ok(None) => contract_api::revert(Error::ValueNotFound.into()),
        Err(_) => contract_api::revert(Error::GetURef.into())
    };
    let pos_pointer = unwrap_or_revert(pos_contract.to_c_ptr(), 77);

    let source_purse = contract_api::main_purse();
    let bonding_purse = contract_api::create_purse();
    let bond_amount: U512 = match contract_api::get_arg::<u64>(0) {
        Some(Ok(amount)) => amount.into(),
        Some(Err(_)) => contract_api::revert(Error::InvalidArgument.into()),
        None => contract_api::revert(Error::MissingArgument.into()),
    };

    match contract_api::transfer_from_purse_to_purse(source_purse, bonding_purse, bond_amount) {
        PurseTransferResult::TransferSuccessful => contract_api::call_contract(
            pos_pointer,
            &(BOND_METHOD_NAME, bond_amount, bonding_purse),
            &vec![Key::URef(bonding_purse.value())],
        ),

        PurseTransferResult::TransferError => contract_api::revert(1324),
    }
}

fn unwrap_or_revert<T>(option: Option<T>, code: u32) -> T {
    if let Some(value) = option {
        value
    } else {
        contract_api::revert(code)
    }
}
