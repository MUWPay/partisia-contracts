//! This is an example of the pool contract, reimplemented from liquidity swapper contract from partisia contract <br>
//! 
//! these will be deployed onchain with different token base pairs, 
//! relative price of the token and transaction fees 
//! and the aggregator will be able to determine the best path
//! for swapping the tokens in order to determine the pool in order to swap.
//! Liquidity provider will need to instantiate the contract with 
//! some tokens deposited for both pairs based on the price.

#![allow(unused_variables)]

#[macro_use]
extern crate pbc_contract_codegen;
extern crate core;

use create_type_spec_derive::CreateTypeSpec;
use pbc_contract_common::address::{Address, AddressType, Shortname};
use pbc_contract_common::context::{CallbackContext, ContractContext};
use pbc_contract_common::events::EventGroup;
use read_write_rpc_derive::ReadWriteRPC;
use read_write_state_derive::ReadWriteState;
use std::collections::btree_map::BTreeMap;
use byoc::src::Contract::{
    initialize, transfer, transfer_from, approve, mint, burn, wrap
}


/// Enum for token types
#[derive(PartialEq, Eq, ReadWriteRPC, CreateTypeSpec)]
#[cfg_attr(test, derive(Debug))]
pub enum Token {
    /// The value representing token A.
    #[discriminant(0)]
    TokenA {},
    /// The value representing token B.
    #[discriminant(1)]
    TokenB {},
    /// The value representing a liquidity token (accessible by the liquidity provider in order to earn the amount when needed).
    #[discriminant(2)]
    LiquidityToken {},
}

/// Keeps track of token amounts (both pairs as well as LP).
#[derive(ReadWriteState, CreateTypeSpec)]
#[cfg_attr(test, derive())]
pub struct TokenBalance {

    /// The amount of token A that a user can withdraw from the contract.
    pub a_tokens: u128,
    /// The amount of token B that a user can withdraw from the contract.
    pub b_tokens: u128,
    /// The amount of liquidity tokens that a user may burn.
    pub liquidity_tokens: u128,
}


impl TokenBalance {

const A: Token = Token::TokenA {};
const B: Token = Token::TokenB {};
const LIQUIDITY: Token = Token::LiquidityToken {};


/// function for getting balence of the user based on the token category
/// * `token`: [`Token`] - The token matching the desired amount.
///
/// # Returns
/// token held by address, of type [`u128`] .

fn get_amount_of(&self, token: Token) -> u128 {
    if token == Token::LiquidityToken {
        self.liquidity_tokens
    } else if token == Token::TokenA {
        self.a_tokens
    } else {
        self.b_tokens
    }
}

    /// Retrieves a mutable reference to the amount that matches `token`.
    ///
    /// ### Parameters:
    ///
    /// * `token`: [`Token`] - The token matching the desired amount.
    ///
    /// # Returns
    /// A mutable value of type [`&mut u128`]
    fn get_mut_amount_of(&mut self, token: Token) -> &mut u128 {
        if token == Token::LIQUIDITY {
            &mut self.liquidity_tokens
        } else if token == Token::A {
            &mut self.a_tokens
        } else {
            &mut self.b_tokens
        }
    }

    /// modifier to verify if the user is not liquidity provider
    /// 
    /// # Returns
    /// True if the user has no tokens, false otherwise [`bool`]

    fn user_has_no_tokens(&self) -> bool {
        self.a_tokens == 0 && self.b_tokens == 0 && self.liquidity_tokens == 0
    }
}


const empty_pool_balance: TokenBalance = TokenBalance {
    a_tokens: 0,
    b_tokens: 0,
    liquidity_tokens: 0,
};


// the state of pool that is to be persisted onchain.
#[state]
pub struct PoolContractState {
    /// The address of this contract
    pub contract: Address,
    /// address of the owner.
    pub owner: Address,
    /// The address of the first token.
    pub token_a_address: Address,
    /// The address of the second token.
    pub token_b_address: Address,
    // this is swap fees (in ppm) that will be charged for each swap.
    pub swap_fees: u128, 
    // this is the value of token and corresponding balance, will have the input and swapped token and their balances submitted by the 
    pub token_balance: BTreeMap<Address, TokenBalance>,
}

impl PoolContractState {

// INTERNAL FUNCTIONS

/// internal function adding token to the balance.
/// Paramters:
 /// * `user`: [`&Address`] - A reference to the user to add `amount` to.
///
/// * `token`: [`Token`] - The token to add to.
///
/// * `amount`: [`u128`] - The amount to add.
///
fn add_token_balance(&mut self, user: Address, token: Token, amount: u128) {
    let token_balance = self.get_mut_balance_for(&user);
    //TODO: transfer the token from the user to the contract


    *token_balance.get_mut_amount_of(&token) += amount;
    
}
///  internal function for decucting the balance from the token_balances map (both token liquidity).
/// * `user`: [`&Address`] - address of the user that wants to  recover their liquidity
/// * `token`: [`Token`] - The token to which you want to recover the liquidity.
/// * `amount`: [`u128`] - The amount of token (defined by amountA, amountB) that is to be recovered.

fn deduct_from_token_balance(&mut self, user: Address, token: &Token, amount: u128) {
    let token_balance = self.get_mut_balance_for(&user);
    *token_balance.get_mut_amount_of(token) = token_balance
            .get_amount_of(token)
            .checked_sub(amount)
            .expect("Insufficient funds");

        if token_balance.user_has_no_tokens() {
            self.token_balances.remove(&user);
        }
}

/// swap the token1 tokens to the tokenB tokens based on the exchange.
///  * `user`: [`&Address`] - address of the user that wants to  swap their liquidity to the destination address.
// * `tokenA`: [`Token`] - The token from  which you want to swap from.
/// * `tokenB`: [`Token`] - The token to which  you want send the tokens.

fn swap_from_source(&mut self, userAddress: &Address, token_a: Token, token_b: Token, amountA: u128) {
    let mut token_balance = self.token_balance.get_mut(userAddress).unwrap();
    add_liquidity(userAddress,token_a,token_b,amountA);   

}



// function to check if pool contract liquidity is initialized. 
fn pool_has_liquidity(&self) -> bool {
    let contract_token_balance = self.get_balance_for(&self.contract);
    contract_token_balance.a_tokens != 0 && contract_token_balance.b_tokens != 0
}




// function to fetch the the correct token reference. 

fn get_tokens(&self, provided_token_address: Address) -> (Token, Token) {
    let provided_a = self.token_a_address == provided_token_address;
        let provided_b = self.token_b_address == provided_token_address;
        if !provided_a && !provided_b {
            panic!("Provided invalid token address")
        }

        if provided_a {
            (Token::A, Token::B)
        } else {
            (Token::B, Token::A)
        }
}








/// Find the u128 square root of `y` (using binary search) rounding down.
///
/// ### Parameters:
///
/// * `y`: [`u128`] - The number to find the square root of.
///
/// ### Returns:
/// The largest x, such that x*x is <= y of type [`u128`]
fn u128_sqrt(y: u128) -> u128 {
    let mut l: u128 = 0;
    let mut m: u128;
    let mut r: u128 = y + 1;

    while l != r - 1 {
        m = (l + r) / 2; // binary search (round down)

        if m * m <= y {
            l = m; // Keep searching in right side
        } else {
            r = m; // Keep searching in left side
        }
    }
    l
}



// Function for determining the amount of exit token the user can get in swap_for_amount given the deduction of fees (given in 1000)
/// ### Parameters:
/// * `from_pool`: [`u128`] - The token pool matching the token of `swap_from_amount`.
/// * `to_pool`: [`u128`] - The opposite token pool.
/// * `swap_from_amount`: [`u128`] - The amount being swapped.
/// * `swap_fee`: [`u128`] - The fee (in 1000) that is to be subtracted, its defined during instantiation phase by the owner.
/// Returns 
/// The amount received after swapping. [`u128`]

fn calculate_swap_to_amount(
    from_pool: u128,
    to_pool: u128,
    swap_from_amount: u128,
    swap_fee: u128,
) -> u128 {
let remainder_ratio = 1000 - swap_fee;
remainder_ratio * swap_from_amount * to_pool)
        / (1000 * from_pool + remainder_ratio * swap_from_amount)
}



/// fetches the corresponding exit tokens based on the given amount of input liquidity is for [provide_liquidity] function.
/// ### Parameters:
/// * `provided_amount`: [`u128`] - The amount being provided to the contract.
/// * `provided_pool`: [`u128`] - The token pool matching the provided amount.
/// * `opposite_pool`: [`u128`] - The opposite pool.
/// * `total_minted_liquidity` [`u128`] - The total current minted liquidity.
/// # Returns
/// The new A pool, B pool and minted liquidity values ([`u128`], [`u128`], [`u128`]). 

fn calculate_equivalent_minted_tokens(
    provided_amount: u128,
    provided_pool: u128,
    opposite_pool: u128,
    total_minted_liquidity: u128,
)
{
    // Handle zero-case
    let opposite_equivalent = if provided_amount > 0 {
        (provided_amount * opposite_pool / provided_pool) + 1
    } else {
        0
    };
    let minted_liquidity_tokens = provided_amount * total_minted_liquidity / provided_pool;
    (opposite_equivalent, minted_liquidity_tokens)
}
// function to determine what will be the corresponding output token based on the given input token
// implemented from the liquidity-swap contract with rounding errors.
///
/// ### Parameters:
///
/// * `liquidity_token_amount`: [`u128`] - The amount of liquidity tokens being reclaimed.
///
/// * `pool_a`: [`u128`] - Pool a of this contract.
///
/// * `pool_b`: [`u128`] - Pool b of this contract.
///
/// * `minted_liquidity` [`u128`] - The total current minted liquidity.
/// # Returns
/// The new A pool, B pool and minted liquidity values ([`u128`], [`u128`], [`u128`])
fn calculate_reclaim_output(
    liquidity_token_amount: u128,
    pool_a: u128,
    pool_b: u128,
    minted_liquidity: u128,
) -> (u128, u128) {
    let a_output = pool_a * liquidity_token_amount / minted_liquidity;
    let b_output = pool_b * liquidity_token_amount / minted_liquidity;
    (a_output, b_output)
}

/// Moves tokens from the providing user's balance to the contract's and mints liquidity tokens for the liquidity provider.
/// ### Parameters:
///
///  * `state`: [`PoolContractState`] - The current state of Pool contract.
///
/// * `user`: [`&Address`] - The address of the user providing liquidity.
///
/// * `provided_token_address`: [`Address`] - The address of the token being provided.
///
///  * `provided_amount`: [`u128`] - The amount provided.
///
///  * `opposite_amount`: [`u128`] - The amount equivalent to the provided amount of the opposite token.
///
///  * `minted_liquidity_tokens`: [`u128`] - The amount of liquidity tokens that the provided tokens yields.

fn provide_liquidity(
    state: &mut PoolContractState,
    user: &Address,
    provided_token_address: Address,
    provided_amount: u128,
    exchanged_amount: u128,
    minted_liquidity_tokens: u128
)
{

    let (provided_token, exchanged_token) = state.get_tokens(provided_token_address);
    // TODO: change the move_tokens to transfer the tokens from the Liq provider to address.
    state.move_tokens(*user, provided_token, provided_amount);
    state.move_tokens(*user, exchanged_token, exchanged_amount);

    state.add_to_token_balance(*user, Token::LIQUIDITY, minted_liquidity_tokens);
    state.add_to_token_balance(state.contract, Token::LIQUIDITY, minted_liquidity_tokens);
}

// PUBLIC functions 
/// Initializes the contract with the given values.
/// # Parameters
///  * `context`: [`ContractContext`] - The contract context containing sender and chain information
///  * `token_a_address`: [`Address`] - The address of incoming token pair in the swap.
/// * `token_b_address`: [`Address`] - The address of outgoing token pair in the swap.
/// * `swap_fee`: [`u128`] - The fee (in 1000) that is to be subtracted from the final swap value, and paid to the corresponding contract owner.

pub fn initialize(
    ctx: ContractContext,
    token_a_address: Address,
    token_b_address: Address,
    swap_fee: u128
)
{
    assert_ne!(
        token_a_address.address_type,
        AddressType::Account,
        "Tried to provide an account as token for token A"
    );
    assert_ne!(
        token_b_address.address_type,
        AddressType::Account,
        "Tried to provide an account as token for token B"
    );
    assert_ne!(
        token_a_address, token_b_address,
        "Cannot initialize swap with duplicate tokens"
    );

    assert_ne!(
        swap_fee, 0,
        "Swap fee cannot be zero"
    );

    let deployed_pool = PoolContractState{  
     ctx.sender,
     token_a_address,
     token_b_address,
     token_balances:: BTreeMap::new(), 
    }

    (new_state, vec![])
}
}



/// this function allows the owner (Liq Provider) to deposit the tokens once the contract is initialized.
/// # Parameters
/// * `context`: [`ContractContext`] - The contract context containing sender and chain information.
/// * `state`: [`LiquiditySwapContractState`] - The current state of the contract.
/// 