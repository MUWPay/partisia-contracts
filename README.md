[![Tests and Linter checks](https://github.com/partisiablockchainapplications/CoreContracts/actions/workflows/basic.yml/badge.svg)](https://github.com/partisiablockchainapplications/CoreContracts/actions/workflows/basic.yml)

# MUWPay application

Monorepo showcasing the development of the Muwpay  multi token wallet application (inclusing the smart contracts on partisia and web application).

## Packages 
                                              |

## Build instructions:

- build the smart contracts for specific package: `nx run build:contracts`.
- pass the wasm runtime and file to the online ABI format


## Test:


## How to build contracts


The package/contracts folder contains 


1. Clone this repo
2. Create a new contract outside this folder
3. Import {contract}-base package from `packages/` folder.
4. Copy all the files from selected contract, for example from `contracts/mpc20`
5. Run `cargo partisia-contract build --release` command.

Or you can download pre-compiled artifacts from [`here`](https://github.com/partisiablockchainapplications/CoreContracts/releases)
