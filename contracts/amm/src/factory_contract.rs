//! factory contract allows to instantiate the pod with specific token  pool,
//! this  will allow the the person who wants to swap to search for the  best path swapping from token A -> C via multiple paths.
//! this is based on the Uniswap V2 factory contract
//!
use crate::pools::initialize;
use crate::pools::PoolContractState;
use std::collections::BTreeMap;

use create_type_spec_derive::CreateTypeSpec;
use pbc_contract_common::address::Address;
use pbc_contract_common::context::{CallbackContext, ContractContext};
use pbc_contract_common::events::EventGroup;
use read_write_rpc_derive::ReadWriteRPC;
use read_write_state_derive::ReadWriteState;

/**
 * data structure defining the LP pair along with owner
 * currently i consider the Lp owner to be unique in the given protocol, although in reality it is not.
 *
 */

#[derive(Debug, ReadWriteRPC, ReadWriteState, CreateTypeSpec)]
pub struct TokenPair {
    token_a_address: Address,
    token_b_address: Address,
    // here the owner corresponds to the liquidity provider to the given contract
    owner: Address,
}

#[state]
#[derive(Debug, CreateTypeSpec)]
pub struct PoolFactoryContractState {
    // storing the various pool contract with corresponding pair of tokens
    poolInfo: BTreeMap<Address, BTreeMap<Address, Address>>,
    // these are the tokens that are registered with the pool with their liquidity.
    registeredPairs: Vec<TokenPair>,
    // this is minimum amount of tokens that are to be supplied (for each token in the token pair) to the pool contract
    minimum_liquidity: u128,
    // this is the initial fees required to be paid for each swap done by the contract.
    swap_fees: u128,
    // manager of the pool factory contract (it only has power to add and remove the token pairs).
    Owner: Address,
}

impl PoolFactoryContractState {
    fn set_fees(&mut self, fees: u128) -> bool {
        self.swap_fees = fees;
        return true;
    }
  
}




    /**
     * this function sets the pairs for the Pool contract.
     * ctx: reference of the pool contract that is created.
     * pairRegistered : the pair of tokens  for which the token pool is created.
     *
     */
pub fn set_poolInfo(
   ownerAddress: Address,
   pairRegistered: &TokenPair,
) -> BTreeMap<Address, BTreeMap<Address, Address>> {
   let mut poolInfo = BTreeMap::new();

   poolInfo
       .entry(ownerAddress)
       .or_insert_with(BTreeMap::new)
       .insert(
           pairRegistered.token_a_address,
           pairRegistered.token_b_address,
       );

   return poolInfo;
}




#[init]
pub fn initFactory(
    ctx: ContractContext,
    pair: TokenPair,
    _swap_fees: u128,
    initial_liquidity: u128,
    ownerAddress: Address
) -> (PoolFactoryContractState, Vec<EventGroup>) {
    assert_ne!(pair.token_a_address, pair.token_b_address, "factory: IDENTICAL TOKENS");
    // now initializing the pool contract with the given pair along with some initial liquidity.
    initialize(
        ctx,
        pair.token_a_address,
        pair.token_b_address,
        _swap_fees,
        ownerAddress,
    );

    let deployedPoolState = PoolFactoryContractState {
        poolInfo: set_poolInfo(ownerAddress, &pair),
        registeredPairs: vec![pair],
        swap_fees: _swap_fees,
        minimum_liquidity: initial_liquidity,

        Owner: ownerAddress,
    };

    return (deployedPoolState, vec![]);
}

pub fn setFeesTo(ctx: ContractContext, fees: u128, mut state: PoolFactoryContractState) {
    assert_ne!(ctx.sender, state.Owner, "factory: ZERO_ADDRESS");

    state.set_fees(fees);
}
