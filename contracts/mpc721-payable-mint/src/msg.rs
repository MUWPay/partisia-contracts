use create_type_spec_derive::CreateTypeSpec;
use mpc721_base::msg::InitMsg;
use read_write_rpc_derive::ReadWriteRPC;

use crate::state::PayableMintInfo;

#[derive(ReadWriteRPC, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct PayableMintInitMsg {
    pub mpc721: InitMsg,
    pub payable_mint_info: PayableMintInfo,
}
