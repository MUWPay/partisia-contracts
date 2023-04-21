#![allow(unused_variables)]

#[macro_use]
extern crate pbc_contract_codegen;

use mpc20::{
    TokenState, transfer_from, approve, core_transfer_from, core_transfer, approve, bulk_transfer_from, transfer_from, bulk_transfer, 
    transfer 
}

use pbc_contract_common::{
    address::{Address, AddressType},
    context::{CallbackContext, ContractContext},
    events::EventGroup,
};


// details (address , amount on the specific chanin )
pub struct WrappedTokenInfo {
/// mpc20 token name
pub name: String,
/// mpc20 token symbol
pub symbol: String,
/// mpc20 token decimals
pub decimals: u8,
}

#[derive(ReadWriteRPC, ReadWriteState, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]


pub struct Mpc20ByocInitMsg {
    //
    pub info: WrappedTokenInfo,
    pub total_supply: u128
    pub minter: Option<Address>,
    pub byoc: Address,
}

#[derive(ReadWriteRPC, ReadWriteState, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]

// mapping the ERC20 token and wrap it with corresponding external token.
pub struct WrappedTokenState {
    pub mpc20: TokenState;
    pub byoc: Address;

}


/**
 * this is additional information (ie the amount of ERC20 tokens that are to be encapsulated to the given MPC20 token)
 * 
 */
pub struct WrapMsg {
    pub amount: u128;
}

/**
 * for initialization of the contract.
 * ctx: is the current contract context for accessing the address and other prsential properties)
 * 
 * 
 * 
 */


#[init]
pub fn initialize(
    ctx: &ContractContext,
    msg: &Mpc20ByocInitMsg
) -> (WrappedTokenState, Vec<EventGroup>) {

assert(msg.byoc.address_type == AddressType::SystemContract, "BYOC::initialize-> only smart contract can be initialized");
// MPC20 token is initialized.
let (mpc20, events) = mpc20::initialize(&ctx, &msg.info.name, &msg.info.symbol, &msg.info.decimals, &msg.total_supply);
// define the information of the TokenState 

let wrappedTokenState =  WrappedTokenState {
    mpc20,
    msg.byoc
}; 
(wrappedTokenState, events);
}

/**
 * typical transfer function simimar to the one in MPC20 token, this will call the corresponding ERC20 token transfer function.
 * 
 * ctx: its the contract context of the token contract.
 * 
 */
#[action(shortname= 0x01)]
pub fn transfer(
    ctx: ContractContext,
    state: WrappedTokenState,
    to: Address,
    amount: u128,
) -> (WrappedTokenState, Vec<EventGroup>) {
let mut wrappedState : WrappedTokenState = state;
let events : Vec<EventGroup> = transfer_from(&ctx, state: &mut state.mpc20, to, amount);
(wrappedState, events);
}














}