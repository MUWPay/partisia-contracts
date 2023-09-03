use mpc20_base::msg::{
    ApproveMsg, BurnFromMsg, BurnMsg, DecreaseAllowanceMsg, IncreaseAllowanceMsg, MintMsg,
    TransferFromMsg, TransferMsg,
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

const TRANSFER: u32 = 0x01;
const TRANSFER_FROM: u32 = 0x03;
const APPROVE: u32 = 0x05;
const MINT: u32 = 0x07;
const BURN: u32 = 0x09;
const BURN_FROM: u32 = 0x11;
const INCREASE_ALLOWANCE: u32 = 0x13;
const DECREASE_ALLOWANCE: u32 = 0x15;

#[test]
fn proper_transfer_action_call() {
    let dest = mock_address(30u8);

    let msg = TransferMsg {
        to: mock_address(1u8),
        amount: 100,
    };

    let mut event_group = EventGroup::builder();
    msg.as_interaction(&mut event_group, &dest);

    let mut test_event_group = EventGroup::builder();
    test_event_group
        .call(dest.clone(), Shortname::from_u32(TRANSFER))
        .argument(mock_address(1u8))
        .argument(100u128)
        .done();

    assert_eq!(event_group.build(), test_event_group.build());
}

#[test]
fn proper_transfer_from_action_call() {
    let dest = mock_address(30u8);

    let msg = TransferFromMsg {
        from: mock_address(1u8),
        to: mock_address(2u8),
        amount: 100,
    };

    let mut event_group = EventGroup::builder();
    msg.as_interaction(&mut event_group, &dest);

    let mut test_event_group = EventGroup::builder();
    test_event_group
        .call(dest.clone(), Shortname::from_u32(TRANSFER_FROM))
        .argument(mock_address(1u8))
        .argument(mock_address(2u8))
        .argument(100u128)
        .done();

    assert_eq!(event_group.build(), test_event_group.build());
}

#[test]
fn proper_approve_action_call() {
    let dest = mock_address(30u8);

    let msg = ApproveMsg {
        spender: mock_address(1u8),
        amount: 100,
    };

    let mut event_group = EventGroup::builder();
    msg.as_interaction(&mut event_group, &dest);

    let mut test_event_group = EventGroup::builder();
    test_event_group
        .call(dest.clone(), Shortname::from_u32(APPROVE))
        .argument(mock_address(1u8))
        .argument(100u128)
        .done();

    assert_eq!(event_group.build(), test_event_group.build());
}

#[test]
fn proper_mint_action_call() {
    let dest = mock_address(30u8);

    let msg = MintMsg {
        recipient: mock_address(1u8),
        amount: 100,
    };

    let mut event_group = EventGroup::builder();
    msg.as_interaction(&mut event_group, &dest);

    let mut test_event_group = EventGroup::builder();
    test_event_group
        .call(dest.clone(), Shortname::from_u32(MINT))
        .argument(mock_address(1u8))
        .argument(100u128)
        .done();

    assert_eq!(event_group.build(), test_event_group.build());
}

#[test]
fn proper_burn_action_call() {
    let dest = mock_address(30u8);

    let msg = BurnMsg { amount: 100 };

    let mut event_group = EventGroup::builder();
    msg.as_interaction(&mut event_group, &dest);

    let mut test_event_group = EventGroup::builder();
    test_event_group
        .call(dest.clone(), Shortname::from_u32(BURN))
        .argument(100u128)
        .done();

    assert_eq!(event_group.build(), test_event_group.build());
}

#[test]
fn proper_burn_from_action_call() {
    let dest = mock_address(30u8);

    let msg = BurnFromMsg {
        owner: mock_address(1u8),
        amount: 100,
    };

    let mut event_group = EventGroup::builder();
    msg.as_interaction(&mut event_group, &dest);

    let mut test_event_group = EventGroup::builder();
    test_event_group
        .call(dest.clone(), Shortname::from_u32(BURN_FROM))
        .argument(mock_address(1u8))
        .argument(100u128)
        .done();

    assert_eq!(event_group.build(), test_event_group.build());
}

#[test]
fn proper_increase_allowance_action_call() {
    let dest = mock_address(30u8);

    let msg = IncreaseAllowanceMsg {
        spender: mock_address(1u8),
        amount: 100,
    };

    let mut event_group = EventGroup::builder();
    msg.as_interaction(&mut event_group, &dest);

    let mut test_event_group = EventGroup::builder();
    test_event_group
        .call(dest.clone(), Shortname::from_u32(INCREASE_ALLOWANCE))
        .argument(mock_address(1u8))
        .argument(100u128)
        .done();

    assert_eq!(event_group.build(), test_event_group.build());
}

#[test]
fn proper_decrease_allowance_action_call() {
    let dest = mock_address(30u8);

    let msg = DecreaseAllowanceMsg {
        spender: mock_address(1u8),
        amount: 100,
    };

    let mut event_group = EventGroup::builder();
    msg.as_interaction(&mut event_group, &dest);

    let mut test_event_group = EventGroup::builder();
    test_event_group
        .call(dest.clone(), Shortname::from_u32(DECREASE_ALLOWANCE))
        .argument(mock_address(1u8))
        .argument(100u128)
        .done();

    assert_eq!(event_group.build(), test_event_group.build());
}
