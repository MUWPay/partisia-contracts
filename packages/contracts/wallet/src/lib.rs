//! Wallet contract is onchain wallet that allows user to pre-approve some amount of tokens in various denominations to be spend

use std::collections::{BTreeMap, HashMap};
use crate::state::ContractState;
use pbc_contract_common::{
    context::ContractContext, events::EventGroup, address::Address,
    events::EventGroup
};
use mpc20_byoc::contract::{approve};

#[state]
struct WalletContractState {
// these are the amount of tokens pre-approved by the EOA in order to be transported by the wallet
tokenAddressesWithAmountsApproved: BTreeMap<String, u128>,    
// EOA owner of the wallet
addressOwner: String
}



impl WalletContractState {

/**
 * function for adding the approval for the given tokens from the owner to the contract.
 * transferred to this address via a batch call.
 * approvalAddress_accounts is the hashmap of token address to amount approved for the correct
 */

fn batchApproval(
    ctx: ContractContext,
    approvalAddress_accounts: HashMap<Address,u128>,   
) {
    // check if the caller is the owner
    let caller = ctx.caller();
    if &caller == &state.addressOwner {
    // then call the values for all the address token (in for loop) to approve the mpc20 token for this contract address
    for (address, amount) in approvalAddress_accounts {
        approve(ctx, address, amount);
    }
// sending the request of the fact that users are 
}




}
/**
 * here the wallet contract is initialized with the owner address along with the signature signed by the user to approve its creation
 * 
 * 
 * 
 */
#[init] 
fn init_Contract(
    context: ContractContext,
    signature: HashMap<Address, String>, 
    addressOwner: Address
)
{

let state = WalletContractState::new(&context,signature);




}



