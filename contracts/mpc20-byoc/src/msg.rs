use create_type_spec_derive::CreateTypeSpec;
use mpc20_base::msg::Mpc20InitMsg;
use pbc_contract_common::address::{Address, Shortname};
use read_write_rpc_derive::ReadWriteRPC;

use rpc_msg_derive::IntoShortnameRPCEvent;
use utils::events::IntoShortnameRPCEvent;

#[derive(ReadWriteRPC, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct Mpc20ByocInitMsg {
    pub mpc20: Mpc20InitMsg,
    pub byoc: Address,
}

/// ## Description
/// This structure describes fields for mpc20-byoc wrap msg
#[derive(ReadWriteRPC, CreateTypeSpec, IntoShortnameRPCEvent, Clone, PartialEq, Eq, Debug)]
#[rpc_msg(action = 0x17)]
pub struct WrapMsg {
    pub amount: u128,
}
