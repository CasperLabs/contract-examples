#![no_std]
#![feature(alloc)]

extern crate common;
use common::contract_api::call_contract;
use common::contract_api::pointers::ContractPointer;

extern crate alloc;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn call() {
    // Assumes that `define` contract was deployed with
    // address == 303030...
    // nonce == 0 (this is a bug since deploy should be using NEW nonce instead of OLD)
    // https://casperlabs.atlassian.net/browse/EE-384
    // fn_index == 0
    let hash = ContractPointer::Hash([
        164, 102, 153, 51, 236, 214, 169, 167, 126, 44, 250, 247, 179, 214, 203, 229, 239, 69, 145,
        25, 5, 153, 113, 55, 255, 188, 176, 201, 7, 4, 42, 100,
    ]);
    // Call `define` part of the contract.
    call_contract(hash, &(), &Vec::new())
}
