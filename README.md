
The purpose of this tool is to transform the foundry broadcast JSON file to an Etheno-like JSON file for seamless Foundry integration with Echidna.

### Demo

![Demo GIF](https://github.com/ChmielewskiKamil/foundry2echidna/blob/master/Foundry2Echidna%20demo.gif)

### Installation

Make sure you have [Rust installed](https://www.rust-lang.org/tools/install).

Install from crates.io:
`cargo install foundry2echidna`

Install from source:

```shell
git clone https://github.com/ChmielewskiKamil/foundry2echidna/foundry2echidna &&
cd foundry2echidna &&
cargo install --path .
```

### How to use it

Once you have `foundry2echidna` installed, you are ready to transform broadcast files.

1. In the root of your Foundry project, run the command `foundry2echidna`
By default, if no arguments were passed, the tool will look for the following:

- Your broadcast in `broadcast/*.s.sol/31337/run-latest.json`
- And will output to `src/crytic/init.json`

You can pass custom input and output paths like this:

`foundry2echidna --input-path path/to/broadcast.json --output-path path/to/init.json`

Or, for short:
`foundry2echidna -i path/to/broadcast.json -o path/to/init.json`

2. Seed Echidna with the generated `init.json` file. Add the following to your `echidna_config.yaml`:

- `initialize: path/to/init.json` (for your custom path)
- `initialize: src/crytic/init.json` (for the default path, if no arguments were provided)

3. Update your `EchidnaTest` contract,
just like you would be interacting with the contracts deployed on the blockchain.

- Get appropriate contract addresses from the `broadcast.json` or `init.json`

```solidity
// Get address of the Counter contract from one of the json files
counter = Counter(0x1234...)
```

4. Run Echidna.

### Current limitations

- Right now `foundry2echidna` does not detect contracts created via Factory contracts.
If a contract was created as a result of a function call,
(like calling `deployMyContract` on a Factory), as of now, it won't be detected.
I am working on a fix, so this shouldn't be an issue soon.

- Getting contract addresses from the JSON files is painful for complex contracts.
I am working on a printer that will list all of the deployed contracts.

### Data Model (inner workings)

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
