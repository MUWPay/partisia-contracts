use contract_version_base::state::ContractVersionBase;
use mpc1_multisig_base::state::MPC1MultisigContractState;

#[state]
#[derive(PartialEq, Eq, Debug)]
pub struct ContractState {
    pub mpc1_multisig: MPC1MultisigContractState,
    pub version: ContractVersionBase,
}
