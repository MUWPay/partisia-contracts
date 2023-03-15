use contract_version_base::state::ContractVersionBase;
use create_type_spec_derive::CreateTypeSpec;
use mpc721_base::state::MPC721ContractState;
use pbc_contract_common::address::Address;
use read_write_rpc_derive::ReadWriteRPC;
use read_write_state_derive::ReadWriteState;

#[state]
#[derive(PartialEq, Eq, Debug)]
pub struct ContractState {
    pub mpc721: MPC721ContractState,
    pub payable_mint_info: PayableMintInfo,
    pub version: ContractVersionBase,
}

#[derive(ReadWriteRPC, ReadWriteState, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct PayableMintInfo {
    pub token: Address,
    pub amount: u128,
}
