//! factory contract allows to instantiate the pod with specific token  pool,
//! this  will allow the the person who wants to swap to search for the  best path swapping from token A -> C via multiple paths.
//! this is based on the Uniswap V2 factory contract
//!
#![allow(unused_variables)]

use crate::pools::{initialize, PoolContractState};
use std::collections::BTreeMap;

use crate::pools::{
    deposit, swap
};

use create_type_spec_derive::CreateTypeSpec;
use pbc_contract_common::address::Address;
use pbc_contract_common::context::{ContractContext};
use pbc_contract_common::events::EventGroup;
use pbc_contract_common::shortname::Shortname;
use read_write_rpc_derive::ReadWriteRPC;
use read_write_state_derive::ReadWriteState;

/**
 * data structure defining the LP pair along with owner
 * currently i consider the Lp owner to be unique in the given protocol, although in reality it is not.
 *
 */
#[derive(Debug, ReadWriteRPC, CreateTypeSpec, ReadWriteState)]
pub struct TokenPair {
    token_a_address: Address,
    token_b_address: Address,
    // here the owner corresponds to the liquidity provider to the given contract
    owner: Address,
}


#[derive(PartialEq, Eq,  Debug)]
pub enum State {
Started,
OrderPending,
OrderCancelled,
OrderWithdrawn,
OrderCompleted
}

#[derive(Debug, ReadWriteState)]

pub struct Order {
walletAddress: Address,
srcAddress: Address,
destAddress: Address,
srcAmount: u128,
destAmountMin: Option<u128>,
_state: u128
}


#[state]
#[derive(Debug, ReadWriteRPC)]
pub struct PoolFactoryContractState {
    // storing the various pool contract with corresponding pair of tokens
    poolinfo: BTreeMap<Address, BTreeMap<Address, Address>>,
    // these are the tokens that are registered with the pool with their liquidity.
    registeredpairs: Vec<TokenPair>,
    // this is minimum amount of tokens that are to be supplied (for each token in the token pair) to the pool contract
    minimum_liquidity: u128,
    // this is the initial fees required to be paid for each swap done by the contract.
    swap_fees: u128,
    // manager of the pool factory contract (it only has power to add and remove the token pairs).
    owner: Address,

    // current ongoing orders 
    current_orders: BTreeMap<Address, Order>,
}

impl PoolFactoryContractState {
    fn set_fees(&mut self, fees: u128) -> bool {
        self.swap_fees = fees;
        return true;
    }

    /**
     * for the creation of new pool contract
     * 
     */

    fn add_registered_pair(&mut self, pair: TokenPair) -> usize {
        self.registeredpairs.push(pair);

        return (self.registeredpairs.len() - 1);
    }

    fn remove_registered_pair(&mut self, index: usize) -> bool {        
        self.registeredpairs.remove(index);
        return true;
    }

    fn add_offer(&mut self, walletAddress: Address,  order: Order) -> bool {
        self.current_orders.insert(walletAddress, order);
        return true;
    }
  
}
    /**
     * This function sets the pairs for the Pool contract.
     * ctx: reference of the pool contract that is created.
     * PairRegistered : the pair of tokens for which the token pool is created.
     *
     */
pub fn set_poolInfo(
   ownerAddress: Address,
   pairRegistered: &TokenPair,
) -> BTreeMap<Address, BTreeMap<Address, Address>> {

   let mut poolinfo = BTreeMap::new();

   poolinfo
       .entry(ownerAddress)
       .or_insert_with(BTreeMap::new)
       .insert(
           pairRegistered.token_a_address,
           pairRegistered.token_b_address,
       );

   return poolinfo;
}

#[init]
pub fn initFactory(
    ctx: ContractContext,
    pair: TokenPair,
    _swap_fees: u128,
    initial_liquidity: u128,
    ownerAddress: Address,
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
        poolinfo: set_poolInfo(ownerAddress, &pair),
        registeredpairs: vec![pair],
        swap_fees: _swap_fees,
        minimum_liquidity: initial_liquidity,
        owner: ownerAddress,
        current_orders: BTreeMap::new(),
    };

    return (deployedPoolState, vec![]);
}

pub fn setFeesTo(ctx: ContractContext, fees: u128, mut state: PoolFactoryContractState) {
    assert!(ctx.sender == state.owner, "factory: ONLY-OWNER ");
    state.set_fees(fees);
}

/**
 * adds the new pool to the factory contract along with registered pairs
 * 
 * 
 */
#[action(shortname = 0x01)]
pub fn createPool(
    ctx: ContractContext,
    pair: TokenPair,
    _swap_fees: u128,
    initial_liquidity: u128,
    state: PoolContractState
) -> (PoolFactoryContractState, Vec<EventGroup>) 
{
assert!(pair.token_a_address != pair.token_b_address, "factory: IDENTICAL TOKENS");
let ownerAddress = ctx.sender;

let mut created_pool = EventGroup::builder();

created_pool.call(
state.initialize
)


let created_pool = initialize(ctx, pair.token_a_address, pair.token_b_address, state.swap_fees, ownerAddress);
// adding the liquidity to the pool contract.
state.add_registered_pair(pair);
state.set_fees(_swap_fees);

return created_pool;
}



/**
 * thuis function is called by multicall contract in order to create the offer based on the quotations of swap rates  
 * ctx: will be the reference object of the wallet account contract.
 * srcToken: is the user input token for which you want to swap.
 * amount: is the amount of token that you want to swap for the input token
 * destToken: is the token that you want to receive in exchange of the input token.
 * destTokenMin: is the minimum amount of token that you want to receive in exchange of the input token.
 */
// remove warnings for snake case
#[action(shortname = 0x02)]
pub fn createOffer(
ctx: ContractContext,
srcToken: Address,
amount: u128,
destToken: Address,
destTokenMin: Option<u128>,
mut state: PoolFactoryContractState
) -> (Order, Vec<EventGroup>) {
   
   let offer =  Order {
         srcAddress: srcToken,
         destAddress: destToken,
         srcAmount: amount,
         destAmountMin: destTokenMin,
         _state: State::OrderPending as u128,
         walletAddress: ctx.sender,
   }; 
    

    let mut event = EventGroup::builder(
    );

    event.call(state.owner, pool_contract_operation())
    .argument(ctx.sender)
    .argument(srcToken)
    .argument(destToken);


    return (offer, vec![event.build()]);

}

/// fallback function that is called after creation of offer 
/// which checks for the required pool which can has required liquidity.


#[action(shortname = 0x03)]
pub fn findPool(
    srcToken: Address,
    destToken: Address,
    amount: u128,
)
{

assert_ne!(srcToken, destToken, "factory: IDENTICAL TOKENS");
assert_ne!(amount, 0, "factory: INVALID AMOUNT");

for (key, value) in state.poolinfo.iter() {
    if value.contains_key(srcToken) && value.contains_key(destToken) {
        let poolAddress = value.get(srcToken).unwrap();
        let poolAddress = value.get(destToken).unwrap();

    }






}

}




// inline functions for doing pool operations for executing the operator

fn pool_contract_operation() -> Shortname {
Shortname::from_u32(0x03);

}




