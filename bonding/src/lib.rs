#![no_std]
#![feature(alloc)]

#[macro_use]
extern crate alloc;
extern crate common;

use common::contract_api::{self, PurseTransferResult};
use common::contract_api::pointers::UPointer;
use common::key::Key;
use common::value::uint::U512;

// Put your desired bonding amount here.
const BONDING_AMOUNT: u64 = 1000;

#[no_mangle]
pub extern "C" fn call() {
    let pos_public: UPointer<Key> = contract_api::get_uref("pos").to_u_ptr().unwrap();
    let pos_contract: Key = contract_api::read(pos_public);
    let pos_pointer = pos_contract.to_c_ptr().unwrap();

    let source_purse = contract_api::main_purse();
    let bonding_purse = contract_api::create_purse();
    let bond_amount: U512 = BONDING_AMOUNT.into();

    match contract_api::transfer_from_purse_to_purse(source_purse, bonding_purse, bond_amount) {
        PurseTransferResult::TransferSuccessful => {
            let _result: () = contract_api::call_contract(
                pos_pointer,
                &("bond", bond_amount, bonding_purse),
                &vec![Key::URef(bonding_purse.value())],
            );
        }

        PurseTransferResult::TransferError => contract_api::revert(1324),
    }
}
