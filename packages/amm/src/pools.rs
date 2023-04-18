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

impl Token {
    const A: Token = Token::TokenA {};
    const B: Token = Token::TokenB {};
    const LIQUIDITY: Token = Token::LiquidityToken {};

}

// this will be the core type to  determine the balance of the user in the pool.
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
/// function for getting balence of the user based on the token category
/// * `token`: [`Token`] - The token matching the desired amount.
///
/// # Returns
/// token held by address, of type [`u128`] .

fn get_amount_of(&self, token: &Token) -> u128 {
    if token == &Token::LIQUIDITY {
        self.liquidity_tokens
    } else if token == &Token::A {
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
    fn get_mut_amount_of(&mut self, token: &Token) -> &mut u128 {
        if token == &Token::LIQUIDITY {
            &mut self.liquidity_tokens
        } else if token == &Token::A {
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
/// Retrieves a copy of the token balance that matches `user`.
    ///
    /// ### Parameters:
    ///
    /// * `user`: [`&Address`] - A reference to the desired user address.
    ///
    /// # Returns
    /// A copy of the token balance that matches `user`.
fn get_balance_for(&self, user: &Address) -> &TokenBalance {
        let token_balance = self.token_balance.get(user).unwrap_or(&empty_pool_balance);
        token_balance
    }

    /// Retrieves a mutable reference to the token balance that matches `user`.
    ///
    /// ### Parameters:
    ///
    /// * `user`: [`&Address`] - A reference to the desired user address.
    ///
    /// # Returns
    /// The mutable reference to the token balance that matches `user`.
fn get_mut_balance_for(&mut self, user: &Address) -> &mut TokenBalance {
        let token_balance = self.token_balance.entry(*user).or_insert(empty_pool_balance);
        token_balance
    }

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
            self.token_balance.remove(&user);
        }
}

// function to check if pool contract liquidity is initialized. 
fn contract_pools_have_liquidity(&self) -> bool {
    let contract_token_balance = self.get_balance_for(&self.contract);
    contract_token_balance.a_tokens != 0 && contract_token_balance.b_tokens != 0
}

// function to fetch the the correct token reference. 



#[init]
pub fn initialize(
    ctx: ContractContext,
    token_a_address: Address,
    token_b_address: Address,
    swap_fee: u128,
    _owner: Address
) -> (PoolContractState, Vec<EventGroup>)
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

    let deployed_pool = PoolContractState {  
     contract:ctx.sender,
        owner: _owner,
     token_a_address,
     token_b_address,
     swap_fees: swap_fee,
     token_balance: BTreeMap::new(), 
    };

    (deployed_pool, vec![])

}

/// this function allows the owner (Liq Provider) to deposit the token liquidity {A,B} once the contract is initialized.
/// # Parameters
/// * `context`: [`ContractContext`] - The contract context containing sender and chain information.
/// * `state`: [`LiquiditySwapContractState`] - The current state of the contract.
///  * `token_address`: [`Address`] - The address of the deposited token contract.
/// * `amount`: [`u128`] - The amount of tokens to be deposited.
///  * `token_address`: [`Address`] - The address of the provided input token
///  * `token_amount`: [`u128`] - The amount to transfer
#[action(shortname = 0x01)]
pub fn deposit(
    context: ContractContext,
    state: PoolContractState,
    token_address: Address,
    token_amount: u128
) -> (PoolContractState, Vec<EventGroup>) {

let (inputToken, _) = state.deduce_tokens(token_address);

let mut deposit_event = EventGroup::builder();

// calling the byoc contract to transfer the tokens to this contract
deposit_event.call(token_address, Self::token_contract_transfer_from())
.argument(context.sender)
.argument(context.contract_address)
.argument(token_amount)
.done();

// then executing callback function after the token transfer in order to send the feedback message.
deposit_event
.with_callback(SHORTNAME_DEPOSIT_CALLBACK)
.argument(inputToken)
.argument(token_amount)
.done();

(state, vec![deposit_event.build()])

}


/// function for handling callback from [`deposit`] function that will register as user will additional amount addded in the contract.<br>
/// along with their address in the record
/// context: [`ContractContext`] - The contract context containing sender and chain information of the function call
/// callback_context: [`CallbackContext`] - The callback context containing the success: failure of the function for which its hooked for callback
#[callback(shortname = 0x10)]
pub fn deposit_callback(
    context: ContractContext,
    callback_context: CallbackContext,
    mut state: PoolContractState,
    token: Token,
    amount: u128,
    ) -> (PoolContractState, Vec<EventGroup>)
 {
    assert!(callback_context.success, "pool::deposity_callback :-> Transfer did not succeed");

    state.add_to_token_balance(context.sender, token, amount);
    
    (state, vec![])

    }


/// function for swapping the tokenA with corresponding amount to the corresponding pair, after deducting the fees from the conversion.
/// 
#[action(shortname = 0x02)]
pub fn swap(
    context: ContractContext,
    mut state: PoolContractState,
    token_address: Address,
    amount: u128,
) -> (PoolContractState, Vec<EventGroup>)

{

    assert!(
        state.contract_pools_have_liquidity(),
        "Pools::swap : no-liquidity"
    );


    let (provided_token, opposite_token)  = state.deduce_tokens(token_address);
    let contract_balance = state.get_balance_for(&state.contract);

    let output_token_amount = Self::calculate_swap_to_amount(
        contract_balance.get_amount_of(&provided_token),
        contract_balance.get_amount_of(&opposite_token),
        amount,
        state.swap_fees
    );

    // here comes the swap 
    /**
     * tokenA : sender -> smart_contract.
     * tokenB : converted_amount - fees -> sender.
     */
    state.move_tokens(context.sender, state.contract, provided_token, amount);

    state.move_tokens(state.contract, context.sender, opposite_token, output_token_amount);

    (state, vec![])

}
// function for liq providers to withdraw the tokens from the pool.

#[action(shortname = 0x03)]
pub fn withdraw(
    context: ContractContext,
    mut state: PoolContractState,
    token_address: Address,
    amount: u128,
) -> (PoolContractState, Vec<EventGroup>)

{

    let (input_token, _) = state.deduce_tokens(token_address);
    
    state.deduct_from_token_balance(context.sender, &input_token, amount);
    
    let mut event_group_builder = EventGroup::builder();

    // calling the functional event in order to transfer the token retrieved by the user direct from the pool.
    event_group_builder.call(token_address, Self::token_contract_transfer())
    .argument(context.sender)
    .argument(amount)
    .done();
    (state, vec![event_group_builder.build()])
}


/**
 * users can deposit liquidity for input,output token based on the exchange rate defined in the pool state
 * 
 * 
 */
#[action(shortname = 0x04)]
pub fn provide_liquidity(
    context: ContractContext,
    mut state: PoolContractState,
    token_address: Address,
    amount: u128,
) -> (PoolContractState, Vec<EventGroup>) {

// fetching the amount of tokens that are available in the .
let user = &context.sender;

let (input_token, output_token) = state.deduce_tokens(token_address);

let contract_balance = state.get_balance_for(&state.contract);

// now to determine how many liquidity tokens are to be generated for given input token supply, we will again use the conversion formula
let (output_supply_equivalent, minted_lp_tokens) = Self::calculate_equivalent_and_minted_tokens(
    amount,
    contract_balance.get_amount_of(&input_token),
    contract_balance.get_amount_of(&output_token),
    contract_balance.liquidity_tokens
    );

// then check if the equivalent liquidity token pool (of the amount) is sufficient in order to submit.
assert!(
    minted_lp_tokens > 0,
    "pool::provide_liquidity:-> Provided amount is insufficient"
);

Self::provide_liquidity_internal(
    &mut state,
    user,
    token_address,
    amount,
    output_supply_equivalent,
    minted_lp_tokens,
);
(state, vec![])
}

/// function for the liquidity providers to reclaim the liquidity (if there is sufficient token equivalent are available ).
/// user will deposit their LP tokens (managed in the pool contract state ) in order to determine the corresponding output value that are available. 
#[action(shortname = 0x05)]
pub fn reclaim_liquidity(
    context: ContractContext,
    mut state: PoolContractState,
    liquidity_token_amount: u128,
) -> (PoolContractState, Vec<EventGroup>) {

// first we will have to see whether the liquidity_token_amount asked by user is >= the value that it currently holds.

    assert!(
        liquidity_token_amount < state.get_balance_for(&context.sender).liquidity_tokens, "pool::reclaim_liquidity :-> not-enough-liqTokens"
    );

let user = &context.sender;

state.deduct_from_token_balance(*user, &Token::LIQUIDITY, liquidity_token_amount);

// now fetch the contract balance in order to generate the balance_for the final recoverable (input, output) tokens for the user

let contract_token_balance = state.get_balance_for(&state.contract);

let (reclaim_input,reclaim_output) = Self::calculate_reclaim_output(
    liquidity_token_amount,
    contract_token_balance.a_tokens,
    contract_token_balance.b_tokens,
    contract_token_balance.liquidity_tokens
);

// and now finally converting the LP tokens to the correponding collateral 

 state.move_tokens(state.contract, *user, Token::B, reclaim_input);
 state.move_tokens(state.contract, *user, Token::A, reclaim_output);
state.deduct_from_token_balance(state.contract, &Token::LIQUIDITY, liquidity_token_amount);
 
 (state, vec![])
}



fn deduce_tokens(&self, provided_token_address: Address) -> (Token, Token) {
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

// internal function for updating the mappings of the tokens once there are operations by user (swap, deposit and withdraw liquidity).
  ///
    /// ### Parameters:
    ///
    /// * `from`: [`Address`] - The address of the transferring party.
    ///
    /// * `to`: [`Address`] - The address of the receiving party.
    ///
    /// * `moved_token`: [`Token`] - The token being transferred.
    ///
    /// * `amount`: [`u128`] - The amount being transferred. 
fn move_tokens(&mut self, from: Address, to: Address, moved_token: Token, amount: u128) {

    self.deduct_from_token_balance(from, &moved_token, amount);
    self.add_to_token_balance(to, moved_token, amount);
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


/// Determines the initial amount of liquidity tokens, or shares that are to be added to the pool 
/// This implementation is derived from section 3.4 of: [Uniswap v2 whitepaper](https://uniswap.org/whitepaper.pdf). <br>
/// It guarantees that the value of a liquidity token becomes independent of the ratio at which liquidity was initially provided.
fn initial_liquidity_tokens(token_a_amount: u128, token_b_amount: u128) -> u128 {
    Self::u128_sqrt(token_a_amount * token_b_amount)
}


/// function for adding tokens to token_balances for the map
/// If the user isn't already present, creates an entry with an empty TokenBalance.
    ///
    /// ### Parameters:
    ///
    /// * `user`: [`&Address`] - A reference to the user to add `amount` to.
    ///
    /// * `token`: [`Token`] - The token to add to.
    ///
    /// * `amount`: [`u128`] - The amount to add.
    ///
fn add_to_token_balance(&mut self, user: Address, token: Token, amount: u128) {
        let token_balance = self.get_mut_balance_for(&user);
        *token_balance.get_mut_amount_of(&token) += amount;
    }


// inline function for generate the events

// for transfer event
#[inline]
fn token_contract_transfer_from() -> Shortname {
    Shortname::from_u32(0x03)
}

#[inline]
fn token_contract_transfer() -> Shortname {
    Shortname::from_u32(0x01)
}
// PUBLIC functions 
/// Initializes the contract with the given values.
/// # Parameters
///  * `context`: [`ContractContext`] - The contract context containing sender and chain information
///  * `token_a_address`: [`Address`] - The address of incoming token pair in the swap.
/// * `token_b_address`: [`Address`] - The address of outgoing token pair in the swap.
/// * `swap_fee`: [`u128`] - The fee (in 1000) that is to be subtracted from the final swap value, and paid to the corresponding contract owner.

/// this function is invoked initial liquidity and creation of contract (entrypoint), to be called by the pool_factory

pub fn provide_initial_liquidity(
    context: ContractContext,
    mut state: PoolContractState,
    input_token_amt: u128,
    output_token_amt: u128,
    ) -> (PoolContractState,Vec<EventGroup>){
  
  assert!(!state.contract_pools_have_liquidity(),
        "pods::provide_initial_liquidity : -> cant-add-liquidity-already-initialized"
);

let liquidity_tokens_init = Self::initial_liquidity_tokens(input_token_amt, output_token_amt);

assert!(liquidity_tokens_init > 0, "pods::provide_initial_liquidity : -> insuuficient-liq" );

let provided_address = state.token_a_address;
        Self::provide_liquidity_internal(
            &mut state,
            &context.sender,
            provided_address,
            input_token_amt,
            output_token_amt,
            liquidity_tokens_init,
        );
        (state, vec![])
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
(remainder_ratio * swap_from_amount * to_pool)
        / (1000 * from_pool + remainder_ratio * swap_from_amount)

}


// function for calculating the amount of LP that is generated for [`provided liquidity`] fnction
// follows the formula defined in the uniswap-v1 .
// provided_amount is the initial amount provided to the contract
// provided_pool is the token pool matching the provided amount(after the fees )
// opposite_pool is the corresponding pair token that are generated in the end.

fn calculate_equivalent_and_minted_tokens(
    provided_amount: u128,
    provided_pool: u128,
    opposite_pool: u128,
    total_minted_liquidity: u128,
) -> (u128, u128) {
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

fn provide_liquidity_internal(
    state: &mut PoolContractState,
    user: &Address,
    provided_token_address: Address,
    provided_amount: u128,
    exchanged_amount: u128,
    minted_liquidity_tokens: u128
)
{

    let (provided_token, exchanged_token) = state.deduce_tokens(provided_token_address);
    state.move_tokens(*user, state.contract, provided_token, provided_amount);
    state.move_tokens(*user,state.contract ,exchanged_token, exchanged_amount);

    state.add_to_token_balance(*user, Token::LIQUIDITY, minted_liquidity_tokens);
    state.add_to_token_balance(state.contract, Token::LIQUIDITY, minted_liquidity_tokens);
}


}