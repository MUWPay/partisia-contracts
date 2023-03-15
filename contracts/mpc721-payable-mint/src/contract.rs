use crate::{msg::PayableMintInitMsg, state::ContractState};

use contract_version_base::state::ContractVersionBase;
use pbc_contract_common::{
    address::Address,
    context::{CallbackContext, ContractContext},
    events::EventGroup,
};

use mpc20_base::msg::{TransferFromMsg as MPC20TransferFromMsg, TransferMsg as MPC20TransferMsg};
use mpc721_base::{
    actions::{
        execute_approve, execute_approve_for_all, execute_burn, execute_init, execute_mint,
        execute_multi_mint, execute_ownership_check, execute_revoke, execute_revoke_for_all,
        execute_set_base_uri, execute_transfer, execute_transfer_from, execute_update_minter,
    },
    msg::{
        ApproveForAllMsg, ApproveMsg, BurnMsg, CheckOwnerMsg, MintMsg, MultiMintMsg,
        RevokeForAllMsg, RevokeMsg, SetBaseUriMsg, TransferFromMsg, TransferMsg, UpdateMinterMsg,
    },
};
use utils::events::{build_msg_callback, IntoShortnameRPCEvent};

const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[init]
pub fn initialize(
    ctx: ContractContext,
    msg: PayableMintInitMsg,
) -> (ContractState, Vec<EventGroup>) {
    assert!(
        msg.payable_mint_info.amount > 0,
        "Payable amount for mint must be a non-zero value"
    );
    assert!(
        msg.mpc721.owner.is_some(),
        "Payable mpc721 version must have an owner"
    );

    let (mpc721, events) = execute_init(&ctx, &msg.mpc721);
    let state = ContractState {
        mpc721,
        payable_mint_info: msg.payable_mint_info,
        version: ContractVersionBase::new(CONTRACT_NAME, CONTRACT_VERSION),
    };

    (state, events)
}

#[action(shortname = 0x01)]
pub fn transfer(
    ctx: ContractContext,
    state: ContractState,
    to: Address,
    token_id: u128,
) -> (ContractState, Vec<EventGroup>) {
    let mut state = state;
    let events = execute_transfer(&ctx, &mut state.mpc721, &TransferMsg { to, token_id });

    (state, events)
}

#[action(shortname = 0x03)]
pub fn transfer_from(
    ctx: ContractContext,
    state: ContractState,
    from: Address,
    to: Address,
    token_id: u128,
) -> (ContractState, Vec<EventGroup>) {
    let mut state = state;
    let events = execute_transfer_from(
        &ctx,
        &mut state.mpc721,
        &TransferFromMsg { from, to, token_id },
    );

    (state, events)
}

#[action(shortname = 0x05)]
pub fn approve(
    ctx: ContractContext,
    state: ContractState,
    spender: Address,
    token_id: u128,
) -> (ContractState, Vec<EventGroup>) {
    let mut state = state;
    let events = execute_approve(&ctx, &mut state.mpc721, &ApproveMsg { spender, token_id });

    (state, events)
}

#[action(shortname = 0x07)]
pub fn set_base_uri(
    ctx: ContractContext,
    state: ContractState,
    new_base_uri: String,
) -> (ContractState, Vec<EventGroup>) {
    let mut state = state;
    let events = execute_set_base_uri(&ctx, &mut state.mpc721, &SetBaseUriMsg { new_base_uri });

    (state, events)
}

#[action(shortname = 0x09)]
pub fn mint(
    ctx: ContractContext,
    state: ContractState,
    token_id: u128,
    receiver: Address,
    token_uri: Option<String>,
) -> (ContractState, Vec<EventGroup>) {
    let mut payout_transfer_events = EventGroup::builder();

    MPC20TransferFromMsg {
        from: receiver,
        to: ctx.contract_address,
        amount: state.payable_mint_info.amount,
    }
    .as_interaction(&mut payout_transfer_events, &state.payable_mint_info.token);

    MPC20TransferMsg {
        to: state.mpc721.owner.unwrap(),
        amount: state.payable_mint_info.amount,
    }
    .as_interaction(&mut payout_transfer_events, &state.payable_mint_info.token);

    build_msg_callback(
        &mut payout_transfer_events,
        0x10,
        &MintMsg {
            token_id,
            to: receiver,
            token_uri,
        },
    );

    (state, vec![payout_transfer_events.build()])
}

#[callback(shortname = 0x10)]
pub fn on_mint_callback(
    ctx: ContractContext,
    callback_ctx: CallbackContext,
    mut state: ContractState,
    msg: MintMsg,
) -> ContractState {
    assert_callback_success(&callback_ctx);
    let _ = execute_mint(&ctx, &mut state.mpc721, &msg);

    state
}

#[action(shortname = 0x11)]
pub fn approve_for_all(
    ctx: ContractContext,
    state: ContractState,
    operator: Address,
) -> (ContractState, Vec<EventGroup>) {
    let mut state = state;
    let events = execute_approve_for_all(&ctx, &mut state.mpc721, &ApproveForAllMsg { operator });

    (state, events)
}

#[action(shortname = 0x13)]
pub fn revoke(
    ctx: ContractContext,
    state: ContractState,
    spender: Address,
    token_id: u128,
) -> (ContractState, Vec<EventGroup>) {
    let mut state = state;
    let events = execute_revoke(&ctx, &mut state.mpc721, &RevokeMsg { spender, token_id });

    (state, events)
}

#[action(shortname = 0x15)]
pub fn revoke_for_all(
    ctx: ContractContext,
    state: ContractState,
    operator: Address,
) -> (ContractState, Vec<EventGroup>) {
    let mut state = state;
    let events = execute_revoke_for_all(&ctx, &mut state.mpc721, &RevokeForAllMsg { operator });

    (state, events)
}

#[action(shortname = 0x17)]
pub fn burn(
    ctx: ContractContext,
    state: ContractState,
    token_id: u128,
) -> (ContractState, Vec<EventGroup>) {
    let mut state = state;
    let events = execute_burn(&ctx, &mut state.mpc721, &BurnMsg { token_id });

    (state, events)
}

#[action(shortname = 0x18)]
pub fn check_ownership(
    ctx: ContractContext,
    state: ContractState,
    owner: Address,
    token_id: u128,
) -> (ContractState, Vec<EventGroup>) {
    let mut state = state;
    let events =
        execute_ownership_check(&ctx, &mut state.mpc721, &CheckOwnerMsg { owner, token_id });
    (state, events)
}

#[action(shortname = 0x19)]
pub fn update_minter(
    ctx: ContractContext,
    state: ContractState,
    new_minter: Address,
) -> (ContractState, Vec<EventGroup>) {
    let mut state = state;
    let events = execute_update_minter(&ctx, &mut state.mpc721, UpdateMinterMsg { new_minter });
    (state, events)
}

#[action(shortname = 0x20)]
pub fn multi_mint(
    ctx: ContractContext,
    state: ContractState,
    mints: Vec<MintMsg>,
) -> (ContractState, Vec<EventGroup>) {
    assert!(!mints.is_empty(), "At least one mint msg should be passed");

    let receiver = mints[0].to.clone();
    assert!(
        !mints.iter().any(|mint| mint.to != receiver),
        "Only one receiver supported in payable multi-mint"
    );

    let mint_price = (mints.len() as u128) * state.payable_mint_info.amount;

    let mut payout_transfer_events = EventGroup::builder();

    MPC20TransferFromMsg {
        from: receiver,
        to: ctx.contract_address,
        amount: mint_price,
    }
    .as_interaction(&mut payout_transfer_events, &state.payable_mint_info.token);

    MPC20TransferMsg {
        to: state.mpc721.owner.unwrap(),
        amount: mint_price,
    }
    .as_interaction(&mut payout_transfer_events, &state.payable_mint_info.token);

    build_msg_callback(&mut payout_transfer_events, 0x21, &MultiMintMsg { mints });

    (state, vec![payout_transfer_events.build()])
}

#[callback(shortname = 0x21)]
pub fn on_multi_mint_callbacl(
    ctx: ContractContext,
    callback_ctx: CallbackContext,
    mut state: ContractState,
    msg: MultiMintMsg,
) -> ContractState {
    assert_callback_success(&callback_ctx);
    let _ = execute_multi_mint(&ctx, &mut state.mpc721, &msg);

    state
}

fn assert_callback_success(callback_ctx: &CallbackContext) {
    assert!(
        callback_ctx.success && callback_ctx.results.iter().all(|res| res.succeeded),
        "Callback has errors"
    );
}
