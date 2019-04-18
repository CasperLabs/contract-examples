#![no_std]
#![feature(alloc)]

#[macro_use]
extern crate alloc;
use alloc::vec::Vec;

extern crate common;
use common::bytesrepr::ToBytes;
use common::contract_api::call_contract;
use common::contract_api::pointers::ContractPointer;

#[no_mangle]
pub extern "C" fn call() {
    //This hash comes from blake2b256( [0;32] ++ [0;8] ++ [0;4] )
    let hash = ContractPointer::Hash([
        164, 102, 153, 51, 236, 214, 169, 167, 126, 44, 250, 247, 179, 214, 203, 229, 239, 69, 145,
        25, 5, 153, 113, 55, 255, 188, 176, 201, 7, 4, 42, 100,
    ]);
    let arg = "inc";
    let _result: () = call_contract(hash.clone(), &arg, &Vec::new());
    let value: i32 = {
        let arg = "get";
        call_contract(hash, &arg, &Vec::new())
    };
    assert_eq!(value, 1);
}
