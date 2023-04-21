//! factory contract allows to instantiate the pod with specific token  pool, 
//! this  will allow the the person who wants to swap to search for the  best path swapping from token A -> C via multiple paths.
//! this is based on the Uniswap V2 factory contract
//! 
use std::iter::Map;
use crate::pools::{PoolFactoryContractState, Token};
use create::pools::PoolFactoryContractState::{
   deduce_tokens 
};
use create_type_spec_derive::CreateTypeSpec;
use pbc_contract_common::address::{Address, AddressType, Shortname};
use pbc_contract_common::context::{CallbackContext, ContractContext};
use pbc_contract_common::events::EventGroup;
use read_write_rpc_derive::ReadWriteRPC;
use read_write_state_derive::ReadWriteState;


#[derive(PartialEq, Eq, ReadWriteRPC, CreateTypeSpec)]
#[cfg_attr(test, derive(Debug))]



#[state]
pub struct PoolFactoryContractState {
// storing the various pool contract with corresponding pair of tokens 
poolInfo: Map<Address, PoolFactoryContractState>,
// these are the tokens that are registered with the pool with their liquidity.
registeredPairs: Vec<Token>,
// this is minimum amount of tokens that are to be supplied (for each token in the token pair) to the pool contract
minimum_liquidity: u128,

}

impl PoolFactoryContractState {


///

pub fn createPair(tokenA: Token, tokenB: Token) -> Address {

assertne!(tokenA, tokenB, "factory: IDENTICAL_ADDRESSES");

let (inputToken, outputToken) = deduce_tokens(tokenA, tokenB);

}







}