The purpose of this tool is to transform the foundry broadcast JSON file to an Etheno-like JSON file for seamless Foundry integration with Echidna.


### Data Model
Etheno handles two main groups of events* (via `EventSummaryPlugin`):
- Contract creations
- Function calls

`ContractCreated` event has the following fields:
- `event` - this is just the name of the type of the event -> `ContractCreated`
- `from` - this is the address of the contract creator
- `contract_address` - deployed contract address
- `gas_used` - the amount of gas used in the transaction
- `gas_price` - gas price used in the transaction
- `data` - transaction data
- `value` - Ether sent in the transaction

`FunctionCall` event has the following fields:
- `event` - this is just the name of the type of the event -> `FunctionCall`
- `from` - address of an account that made the call
- `to` - address of an account that has been called
- `gas_used` - the amount of gas used in the transaction
- `gas_price` - gas price used in the transaction
- `data` - transaction data
- `value` - Ether sent in the transaction

*_There is also block mined event, but it's not crucial for Echidna setup (?)_

Foundry broadcast structure is more complicated than that, but we only care about a couple of fields.
Since we want to transform the broadcast into this Etheno-like structure, we must map appropriate fields together.

There are multiple ways of accessing specific fields. Here are some examples that I will test (where `i` is a specific transaction with a receipt):

| Etheno field | Foundry field |
| --- | --- |
| `event` | `transactions[i].transaction_type` |
| `from` | ~~`receipts[i].from`~~ `transactions[i].transaction.from`|
| `to` | ~~`receipts[i].to`~~ `transactions[i].transaction.to` |
| `contract_address` | ~~`receipts[i].contract_address`~~ `transactions[i].contract_address`| 
| `gas_used` | `receipts[i].gas_used` |
| `gas_price` | `receipts[i].effective_gas_price` |
| `data` | `transactions[i].transaction.data` |
| `value` | `transactions[i].transaction.value` |
