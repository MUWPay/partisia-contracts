use crate::state::TokenState;

use contract_version_base::state::ContractVersionBase;
use pbc_contract_common::{address::Address, context::ContractContext, events::EventGroup};

use mpc20_base::{
    actions::{
        execute_approve, execute_burn, execute_burn_from, execute_decrease_allowance,
        execute_increase_allowance, execute_init, execute_mint, execute_transfer,
        execute_transfer_from,
    },
    msg::{
        ApproveMsg, BurnFromMsg, BurnMsg, DecreaseAllowanceMsg, IncreaseAllowanceMsg, MintMsg,
        Mpc20InitMsg, TransferFromMsg, TransferMsg,
    },
};

const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[init]
pub fn initialize(ctx: ContractContext, msg: Mpc20InitMsg) -> (TokenState, Vec<EventGroup>) {
    let (mpc20, events) = execute_init(&ctx, &msg);
    let state = TokenState {
        mpc20,
        version: ContractVersionBase::new(CONTRACT_NAME, CONTRACT_VERSION),
    };

    (state, events)
}

#[action(shortname = 0x01)]
pub fn transfer(
    ctx: ContractContext,
    state: TokenState,
    to: Address,
    amount: u128,
) -> (TokenState, Vec<EventGroup>) {
    let mut state = state;
    let events = execute_transfer(&ctx, &mut state.mpc20, &TransferMsg { to, amount });

    (state, events)
}

#[action(shortname = 0x03)]
pub fn transfer_from(
    ctx: ContractContext,
    state: TokenState,
    from: Address,
    to: Address,
    amount: u128,
) -> (TokenState, Vec<EventGroup>) {
    let mut state = state;
    let events = execute_transfer_from(
        &ctx,
        &mut state.mpc20,
        &TransferFromMsg { from, to, amount },
    );

    (state, events)
}

#[action(shortname = 0x05)]
pub fn approve(
    ctx: ContractContext,
    state: TokenState,
    spender: Address,
    amount: u128,
) -> (TokenState, Vec<EventGroup>) {
    let mut state = state;
    let events = execute_approve(&ctx, &mut state.mpc20, &ApproveMsg { spender, amount });

    (state, events)
}

#[action(shortname = 0x07)]
pub fn mint(
    ctx: ContractContext,
    state: TokenState,
    recipient: Address,
    amount: u128,
) -> (TokenState, Vec<EventGroup>) {
    let mut state = state;
    let events = execute_mint(&ctx, &mut state.mpc20, &MintMsg { recipient, amount });

    (state, events)
}

#[action(shortname = 0x09)]
pub fn burn(
    ctx: ContractContext,
    state: TokenState,
    amount: u128,
) -> (TokenState, Vec<EventGroup>) {
    let mut state = state;
    let events = execute_burn(&ctx, &mut state.mpc20, &BurnMsg { amount });

    (state, events)
}

#[action(shortname = 0x11)]
pub fn burn_from(
    ctx: ContractContext,
    state: TokenState,
    owner: Address,
    amount: u128,
) -> (TokenState, Vec<EventGroup>) {
    let mut state = state;
    let events = execute_burn_from(&ctx, &mut state.mpc20, &BurnFromMsg { owner, amount });

    (state, events)
}

#[action(shortname = 0x13)]
pub fn increase_allowance(
    ctx: ContractContext,
    state: TokenState,
    spender: Address,
    amount: u128,
) -> (TokenState, Vec<EventGroup>) {
    let mut state = state;
    let events = execute_increase_allowance(
        &ctx,
        &mut state.mpc20,
        &IncreaseAllowanceMsg { spender, amount },
    );

    (state, events)
}

#[action(shortname = 0x15)]
pub fn decrease_allowance(
    ctx: ContractContext,
    state: TokenState,
    spender: Address,
    amount: u128,
) -> (TokenState, Vec<EventGroup>) {
    let mut state = state;
    let events = execute_decrease_allowance(
        &ctx,
        &mut state.mpc20,
        &DecreaseAllowanceMsg { spender, amount },
    );

    (state, events)
}
