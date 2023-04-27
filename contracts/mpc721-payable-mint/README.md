# MPC721 Standard Spec

Theres a set of required actions and state fields that contract must follow to be assumed as a MPC721 Standard.

## Actions
- `transfer(to: Address, token_id: u128)`
- `transfer_from(from: Address, to: Address, token_id: u128)`
- `approve(spender: Address, token_id: u128)`
- `set_base_uri(new_base_uri: String)`
- `mint(token_id: u128, to: Address, token_uri: Option\<String\>)`

## State Fields
Minimal State struct in Json format:

```json
{
    "mpc721": {
        "owner": "<address>" | null,
        "name": "nft_name",
        "symbol": "nft_symbol",
        "base_uri": "nft_base_uri" | null,
        "minter": "<address>",
        "supply": 1,
        "tokens": [
            {
                "key": 1, // token_id
                "value": {
                    "owner": "<token_owner_address>",
                    "approvals": [
                        "<approved_address_1>",
                        "<approved_address_2>",
                    ],
                    "token_uri": "token_uri" | null,
                }
            }
        ]
    }
}
```

## Note
In order to follow the standard by default this module can be imported: [`mpc721-base`](../../packages/mpc721-base/) and following contract methods taken from this contract: [`mpc721-methods`](src/contract.rs)
