The purpose of this tool is to transform foundry broadcast JSON file to Etheno-like JSON file for seamless Foundry integration with Echidna. 


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

*_There is also block mined event, but it is not supported yet_