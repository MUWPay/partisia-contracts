use std::collections::BTreeMap;

use pbc_contract_common::{context::ContractContext, events::EventGroup};
use rust_decimal::prelude::*;
use utils::events::into_rpc_call;

use crate::{
    msg::{ClaimMsg, CompoundMsg, InitMsg, StakeMsg, UnstakeMsg},
    state::MPC20StakingContractState,
    ContractError,
};

use mpc20::{
    actions::execute_init as mpc20_execute_init,
    msg::{InitMsg as Mpc20InitMsg, TransferMsg as Mpc20TransferMsg},
    state::Minter as Mpc20Minter,
};
use utils::decimal::DecimalRatio;

pub fn execute_init(
    ctx: ContractContext,
    msg: InitMsg,
) -> (MPC20StakingContractState, Vec<EventGroup>) {
    msg.validate();

    let deposit_token = if let Some(token) = msg.deposit_token {
        token
    } else {
        ctx.contract_address
    };

    let last_distributed = ctx.block_time as u64;

    let minter = msg.minter.map(|minter_addr| Mpc20Minter {
        minter: minter_addr,
        capacity: None,
    });

    let (mpc20_base_state, _) = mpc20_execute_init(
        ctx,
        Mpc20InitMsg {
            info: msg.info,
            initial_balances: msg.initial_balances,
            minter,
        },
    );

    let state = MPC20StakingContractState {
        deposit_token,
        distribution_amount: msg.distribution_amount,
        distribution_epoch: msg.distribution_epoch,
        global_index: DecimalRatio::zero(),
        total_staked: 0,
        last_distributed,
        stakers: BTreeMap::new(),
        compound_frequency: msg.compound_frequency,
        mpc20_base_state,
    };

    (state, vec![])
}

pub fn execute_stake(
    ctx: ContractContext,
    state: MPC20StakingContractState,
    msg: StakeMsg,
) -> (MPC20StakingContractState, Vec<EventGroup>) {
    let mut state = state;
    let mut staker = state.get_staker(&ctx.sender);

    state.distribute_rewards(ctx.block_time as u64);
    staker.compute_reward(state.global_index);
    state.increase_stake_amount(&ctx.sender, &mut staker, msg.amount);

    let mut event_group = EventGroup::new();
    event_group.send_from_original_sender(
        &state.deposit_token,
        into_rpc_call(Mpc20TransferMsg {
            to: ctx.contract_address,
            amount: msg.amount,
        }),
        None,
    );

    (state, vec![event_group])
}

pub fn execute_unstake(
    ctx: ContractContext,
    state: MPC20StakingContractState,
    msg: UnstakeMsg,
) -> (MPC20StakingContractState, Vec<EventGroup>) {
    let mut state = state;
    let mut staker = state.get_staker(&ctx.sender);

    assert!(
        staker.staked_amount >= msg.amount,
        "{}",
        ContractError::CannotUnstakeMoreThenStaked,
    );

    state.distribute_rewards(ctx.block_time as u64);
    staker.compute_reward(state.global_index);
    state.decrease_stake_amount(&ctx.sender, &mut staker, msg.amount);

    let mut event_group = EventGroup::new();
    event_group.send_from_contract(
        &state.deposit_token,
        into_rpc_call(Mpc20TransferMsg {
            to: ctx.sender,
            amount: msg.amount,
        }),
        None,
    );

    (state, vec![event_group])
}

pub fn execute_claim(
    ctx: ContractContext,
    state: MPC20StakingContractState,
    msg: ClaimMsg,
) -> (MPC20StakingContractState, Vec<EventGroup>) {
    let mut state = state;
    let mut staker = state.get_staker(&ctx.sender);

    state.distribute_rewards(ctx.block_time as u64);
    staker.compute_reward(state.global_index);

    assert!(
        !staker.pending_reward.is_zero(),
        "{}",
        ContractError::NothingToClaim
    );

    let claim_amount = if let Some(amount) = msg.amount {
        assert!(
            amount <= staker.pending_reward && !amount.is_zero(),
            "{}",
            ContractError::CannotClaimMoreThenRewarded
        );
        amount
    } else {
        staker.pending_reward
    };

    staker.pending_reward = staker.pending_reward.checked_sub(claim_amount).unwrap();
    state.store_staker(&ctx.sender, &staker);
    state.mpc20_base_state.mint_to(&ctx.sender, claim_amount);

    (state, vec![])
}

// Works only when deposit_token == ctx.contract_address
pub fn execute_compound(
    ctx: ContractContext,
    state: MPC20StakingContractState,
    msg: CompoundMsg,
) -> (MPC20StakingContractState, Vec<EventGroup>) {
    let mut state = state;
    let mut staker = state.get_staker(&ctx.sender);

    state.distribute_rewards(ctx.block_time as u64);
    staker.compute_reward(state.global_index);

    assert!(
        state.deposit_token == ctx.contract_address,
        "{}",
        ContractError::CompoundOnlyWorksWithSelfToken
    );

    assert!(
        (staker.last_compound + state.compound_frequency) < (ctx.block_time as u64),
        "{}",
        ContractError::ForbiddenToCompoundToOften,
    );

    let compound_amount = if let Some(amount) = msg.amount {
        assert!(
            amount <= staker.pending_reward && !amount.is_zero(),
            "{}",
            ContractError::CannotCompoundMoreThenRewarded
        );
        amount
    } else {
        staker.pending_reward
    };

    staker.last_compound = ctx.block_time as u64;
    staker.pending_reward = staker.pending_reward.checked_sub(compound_amount).unwrap();
    state.increase_stake_amount(&ctx.sender, &mut staker, compound_amount);

    state
        .mpc20_base_state
        .mint_to(&ctx.contract_address, compound_amount);

    (state, vec![])
}
