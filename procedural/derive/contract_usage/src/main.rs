#![allow(unused)]
extern crate derive_contract;
use derive_contract::Contract;

#[derive(Contract)]
pub struct MyContract {
    
}

fn main() {
    let contract = MyContract{};
}
