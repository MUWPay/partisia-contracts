//! factory contract allows to instantiate the pod with specific token  pool, 
//! this  will allow the the person who wants to swap to search for the  best path swapping from token A -> C via multiple paths.
//! this is based on the Uniswap V2 factory contract
//! 
use std::collections::BTreeMap;
use crate::pools::{PoolContractState, Token};
 use crate::pools::{
 initialize
 };


use create_type_spec_derive::CreateTypeSpec;
use pbc_contract_common::address::{Address, AddressType, Shortname};
use pbc_contract_common::context::{CallbackContext, ContractContext};
use pbc_contract_common::events::EventGroup;
use read_write_rpc_derive::ReadWriteRPC;
use read_write_state_derive::ReadWriteState;


#[derive(PartialEq, Eq, CreateTypeSpec)]
#[cfg_attr(test, derive(Debug))]
#[state]
pub struct PoolFactoryContractState {
// storing the various pool contract with corresponding pair of tokens 
poolInfo:  BTreeMap<Address, BTreeMap<Address, Address>>,
// these are the tokens that are registered with the pool with their liquidity.
registeredPairs: Vec<Address>,
// this is minimum amount of tokens that are to be supplied (for each token in the token pair) to the pool contract
minimum_liquidity: u128,
// this is the initial fees required to be paid for each swap done by the contract.
swap_fees: u128,

Owner : Address,

}



/**
 * data structure defining the LP pair along with owner 
 * currently i consider the Lp owner to be unique in the given protocol, although in reality it is not.
 * 
 */
struct TokenPair {
tokenA: Token,
token_a_address : Address,
tokenB: Token,
token_b_address : Address,
owner: Address
}

const ZERO_ADDRESS : Address;

impl PoolFactoryContractState {


/**
 * this function sets the pairs for the Pool contract.
 * ctx: reference of the pool contract that is created.
 * pairRegisteredé
 * 
 */
fn set_poolInfo(&self, pairRegistered: &TokenPair,  ctx: ContractContext)   {
   
let mut pairMapping = self.poolInfo.insert(self.Owner ,  {
      let mut inner_map =  BTreeMap::new();
      inner_map.insert(pairRegistered.token_a_address, pairRegistered.token_b_address);
      inner_map
      });

return pairMapping;
}
}


fn set_registered_pairs(&self, pairRegistered: Address) -> Vec<Address> {

self.registeredPairs.push(pairRegistered);
return registeredPairs[registeredPairs.len() - 1];

}




pub fn createPair(
   ctx:  ContractContext,
   pair: TokenPair, 
   _swap_fees: u128,
   initial_liquidity: u128,
   mut state: PoolContractState
) -> (PoolFactoryContractState, Vec<EventGroup>) {

assertne!(pair.tokenA, pair.tokenB, "factory: IDENTICAL TOKENS");
let (inputToken, outputToken) = deduce_tokens(pair.token_a_address, pair.token_b_address);
// now initializing the pool contract with the given pair along with some initial liquidity.
initialize(ctx, pair.token_a_address, pair.token_b_address, swap_fees, ctx.sender);

let deployedPoolState = PoolFactoryContractState {
poolInfo: state.set_poolInfo(pair, ctx),
registeredPairs: set_registered_pairs(ctx.sender),
swap_fees: _swap_fees,
minimum_liquidity: initial_liquidity,
};

state.set_poolInfo(pair, ctx);

state.set_registered_pairs(ctx.sender);

return (deployedPoolState, vec![]);
}

pub fn setFeesTo(
   ctx:  ContractContext,
   fees: u128,
   mut state: PoolContractState
)  {

assert( ctx.sender, state.Owner, "factory: ZERO_ADDRESS");

state.set_fees(fees);
}