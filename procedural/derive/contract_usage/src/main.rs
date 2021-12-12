#![allow(unused)]
extern crate derive_contract;
use derive_contract::Contract;

#[derive(Contract)]//, Debug
pub struct MyContract {
    
}

fn main() {
    let contract = MyContract{};
    //println!("{:?}", contract);
}
