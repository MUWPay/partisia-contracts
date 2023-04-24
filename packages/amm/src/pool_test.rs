



// unit tests of the pub function calls
// credits to the partisia coreContracts for the implementation


#[cfg(test)]
mod pools {
// adding functions related to the creation and management of pod
use crate::pools::{
    initialize, deposit, swap, withdraw, provide_liquidity, provide_initial_liquidity, reclaim_liquidity
};

use mpc20::contract::{
    initialize, mint, burn, 

}


use rand::Rng;
use rand_chacha::rand_core::SeedableRng;



use pbc_contract_common::address::{Address, AddressType, Shortname};
use pbc_contract_common::context::{CallbackContext, ContractContext};
use pbc_contract_common::context::ExecutionResult;



use std::hash::Hash;
// various shortname events that are needed to be tracked for the test: 

const DEPOSIT: u32 = 0x01;
const SWAP: u32 = 0x02;
const WITHDRAW: u32 = 0x03;
const PROVIDE_LIQUIDITY: u32 = 0x04;
const RECLAIM_INITIAL_LIQUIDITY: u32 = 0x05;
const PROVIDE_INITIAL_LIQUIDITY: u32 = 0x06;
const DEPOSIT_CALLBACK: u32 = 0x10;



fn mock_blocktime() -> i64 {
    let mut rng = rand_chacha::ChaChaRng::from_seed([0u8; 32]);
    return rng.gen_range(0, 1682327094);
}

// here id is the initial byte that determines the category of the address.
fn mock_address(id: u8) -> Address {

let mut rand_array : Vec<u8> = Vec::new();
let mut rng = rand_chacha::ChaChaRng::from_seed([0u8; 32]);


mock_address.push(id);
for _ in 0..21 {
    mock_address.push(rng.gen_range(0, 255));
}

return Address {
    address_type: AddressType::Account,
    identifier: model_address
}

}

fn token_init(ctx: ContractContext) -> (Address, Address) {



//let tokenA = mpc20::contract::initialize(&ctx, );




}


fn mock_transaction_hash() -> Hash {
    return (mock_address(0x01));

}


#[test]
pub fn deposit_works() 
{ 
let deployer = mock_address(0x00);


let ctx =  ContractContext {
    contract_address: mock_address(0x01),
    sender: deployer,
    block_time: mock_blocktime(),
    block_production_time: 0,
    current_transaction: mock_transaction_hash(),
    original_transaction:mock_transaction_hash(),
};

let tokenAddress = token_init(&ctx);

let mut state = initialize(&ctx,tokenAddress.0, tokenAddress.1, 0u8, deployer);
}












}