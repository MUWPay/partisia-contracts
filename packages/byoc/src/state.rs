use contract_version_base::state::ContractVersionBase;
use mpc20_base::state::MPC20ContractState;
use pbc_contract_common::address::Address;

#[state]
#[derive(PartialEq, Eq, Debug)]
pub struct TokenState {
    pub mpc20: MPC20ContractState,
    pub version: ContractVersionBase,
    /// byoc system contract token address
    pub byoc: Address,
    
}