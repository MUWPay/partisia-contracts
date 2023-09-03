use crate::state::ContractState;

use contract_version_base::state::ContractVersionBase;
use pbc_contract_common::{context::ContractContext, events::EventGroup};

use mpc1_multisig_base::{
    actions::{
        execute_close_proposal, execute_create_proposal, execute_execute_proposal, execute_init,
        execute_vote,
    },
    msg::{
        CreateProposalMsg, InitMsg, ProposalCloseMsg, ProposalExecuteCallMsg, ProposalExecuteMsg,
        ProposalVoteMsg,
    },
    state::Vote,
};

const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[init]
pub fn initialize(ctx: ContractContext, msg: InitMsg) -> (ContractState, Vec<EventGroup>) {
    let (mpc1_multisig, events) = execute_init(&ctx, &msg);
    let state = ContractState {
        mpc1_multisig,
        version: ContractVersionBase::new(CONTRACT_NAME, CONTRACT_VERSION),
    };

    (state, events)
}

#[action(shortname = 0x01)]
pub fn create_proposal(
    ctx: ContractContext,
    state: ContractState,
    title: String,
    description: String,
    voting_phase_period: Option<u64>,
    calls: Vec<ProposalExecuteCallMsg>,
) -> (ContractState, Vec<EventGroup>) {
    let mut state = state;
    let events = execute_create_proposal(
        &ctx,
        &mut state.mpc1_multisig,
        &CreateProposalMsg {
            title,
            description,
            voting_phase_period,
            calls,
        },
    );

    (state, events)
}

#[action(shortname = 0x03)]
pub fn vote(
    ctx: ContractContext,
    state: ContractState,
    proposal_id: u64,
    vote: Vote,
) -> (ContractState, Vec<EventGroup>) {
    let mut state = state;
    let events = execute_vote(
        &ctx,
        &mut state.mpc1_multisig,
        &ProposalVoteMsg { proposal_id, vote },
    );

    (state, events)
}

#[action(shortname = 0x05)]
pub fn execute_proposal(
    ctx: ContractContext,
    state: ContractState,
    proposal_id: u64,
) -> (ContractState, Vec<EventGroup>) {
    let mut state = state;
    let events = execute_execute_proposal(
        &ctx,
        &mut state.mpc1_multisig,
        &ProposalExecuteMsg { proposal_id },
    );

    (state, events)
}

#[action(shortname = 0x07)]
pub fn close_proposal(
    ctx: ContractContext,
    state: ContractState,
    proposal_id: u64,
) -> (ContractState, Vec<EventGroup>) {
    let mut state = state;
    let events = execute_close_proposal(
        &ctx,
        &mut state.mpc1_multisig,
        &ProposalCloseMsg { proposal_id },
    );

    (state, events)
}
