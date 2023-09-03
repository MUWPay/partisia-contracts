use mpc1_multisig_base::{
    msg::{
        CreateProposalMsg, ProposalCloseMsg, ProposalExecuteCallMsg, ProposalExecuteMsg,
        ProposalVoteMsg,
    },
    state::YES_VOTE,
};
use pbc_contract_common::{
    address::{Address, AddressType, Shortname},
    events::EventGroup,
};
use utils::events::IntoShortnameRPCEvent;

fn mock_address(le: u8) -> Address {
    Address {
        address_type: AddressType::Account,
        identifier: [
            le, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
            0u8, 0u8, 0u8,
        ],
    }
}

const CREATE_PROPOSAL: u32 = 0x01;
const VOTE: u32 = 0x03;
const EXECUTE_PROPOSAL: u32 = 0x05;
const CLOSE_PROPOSAL: u32 = 0x07;

#[test]
fn proper_create_proposal_action_call() {
    let dest = mock_address(30u8);

    let msg = CreateProposalMsg {
        title: "title".to_string(),
        description: "desc".to_string(),
        voting_phase_period: Some(1),
        calls: vec![ProposalExecuteCallMsg {
            contract: mock_address(1u8),
            base64_encoded_payload: "payload".to_string(),
        }],
    };

    let mut event_group = EventGroup::builder();
    msg.as_interaction(&mut event_group, &dest);

    let mut test_event_group = EventGroup::builder();
    test_event_group
        .call(dest.clone(), Shortname::from_u32(CREATE_PROPOSAL))
        .argument("title".to_string())
        .argument("desc".to_string())
        .argument(Some(1u64))
        .argument(vec![ProposalExecuteCallMsg {
            contract: mock_address(1u8),
            base64_encoded_payload: "payload".to_string(),
        }])
        .done();

    assert_eq!(event_group.build(), test_event_group.build());
}

#[test]
fn proper_vote_action_call() {
    let dest = mock_address(30u8);

    let msg = ProposalVoteMsg {
        proposal_id: 1,
        vote: YES_VOTE,
    };

    let mut event_group = EventGroup::builder();
    msg.as_interaction(&mut event_group, &dest);

    let mut test_event_group = EventGroup::builder();
    test_event_group
        .call(dest.clone(), Shortname::from_u32(VOTE))
        .argument(1u64)
        .argument(YES_VOTE)
        .done();

    assert_eq!(event_group.build(), test_event_group.build());
}

#[test]
fn proper_execute_proposal_action_call() {
    let dest = mock_address(30u8);

    let msg = ProposalExecuteMsg { proposal_id: 1 };

    let mut event_group = EventGroup::builder();
    msg.as_interaction(&mut event_group, &dest);

    let mut test_event_group = EventGroup::builder();
    test_event_group
        .call(dest.clone(), Shortname::from_u32(EXECUTE_PROPOSAL))
        .argument(1u64)
        .done();

    assert_eq!(event_group.build(), test_event_group.build());
}

#[test]
fn proper_close_proposal_action_call() {
    let dest = mock_address(30u8);

    let msg = ProposalCloseMsg { proposal_id: 1 };

    let mut event_group = EventGroup::builder();
    msg.as_interaction(&mut event_group, &dest);

    let mut test_event_group = EventGroup::builder();
    test_event_group
        .call(dest.clone(), Shortname::from_u32(CLOSE_PROPOSAL))
        .argument(1u64)
        .done();

    assert_eq!(event_group.build(), test_event_group.build());
}
