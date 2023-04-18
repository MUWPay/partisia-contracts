use crate::{
    msg::{Mpc20ByocInitMsg, WrapMsg},
    state::TokenState,
};

use pbc_contract_common::{
    address::{Address, AddressType},
    context::{CallbackContext, ContractContext},
    events::EventGroup,
};

use mpc20_base::{
    actions::{
        execute_approve, execute_burn, execute_burn_from, execute_decrease_allowance,
        execute_increase_allowance, execute_init, execute_mint, execute_transfer,
        execute_transfer_from,
    },
    msg::{
        ApproveMsg, BurnFromMsg, BurnMsg, DecreaseAllowanceMsg, IncreaseAllowanceMsg, MintMsg,
        TransferFromMsg, TransferMsg,
    },
};

use utils::events::{assert_callback_success, build_msg_callback, IntoShortnameRPCEvent};


#[init]
pub fn initialize(ctx: ContractContext, msg: Mpc20ByocInitMsg, name: &str, version: &str ) -> (TokenState, Vec<EventGroup>) {
    assert!(
        msg.byoc.address_type == AddressType::SystemContract,
        "BYOC Token Contract must be a System Contract"
    );

    let (mpc20, events) = execute_init(&ctx, &msg.mpc20);
    let state = TokenState {
        mpc20,
        version: ContractVersionBase::new(CONTRACT_NAME, CONTRACT_VERSION),
        byoc: msg.byoc,
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
    let _ = execute_burn(&ctx, &mut state.mpc20, &BurnMsg { amount });

    let mut byoc_transfer_events = EventGroup::builder();

    TransferMsg {
        to: ctx.sender,
        amount,
    }
    .as_interaction(&mut byoc_transfer_events, &state.byoc);

    (state, vec![byoc_transfer_events.build()])
}



#[action(shortname = 0x11)]
pub fn burn_from(
    ctx: ContractContext,
    state: TokenState,
    owner: Address,
    amount: u128,
) -> (TokenState, Vec<EventGroup>) {
    let mut state = state;
    let _ = execute_burn_from(&ctx, &mut state.mpc20, &BurnFromMsg { owner, amount });

    let mut byoc_transfer_events = EventGroup::builder();

    TransferMsg { to: owner, amount }.as_interaction(&mut byoc_transfer_events, &state.byoc);

    (state, vec![byoc_transfer_events.build()])
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

/**
 * function for wrapping the traditional ERC20 contract into corresponding MPC20 contract.
 * 
 * 
 */

#[action(shortname = 0x17)]
pub fn wrap(
    ctx: ContractContext,
    state: TokenState,
    amount: u128,
) -> (TokenState, Vec<EventGroup>) {
    let mut byoc_transfer_events = EventGroup::builder();

    TransferFromMsg {
        from: ctx.sender,
        to: ctx.contract_address,
        amount,
    }
    .as_interaction(&mut byoc_transfer_events, &state.byoc);

    build_msg_callback(&mut byoc_transfer_events, 0x18, &WrapMsg { amount });

    (state, vec![byoc_transfer_events.build()])
}


/**
 * callback function for the wrap contract action.
 *
 */
pub fn on_wrap_callback(
    ctx: ContractContext,
    callback_ctx: CallbackContext,
    mut state: TokenState,
    msg: WrapMsg,
) -> TokenState {
    assert_callback_success(&callback_ctx);

    state.mpc20.mint_to(&ctx.sender, msg.amount);
    state
}

