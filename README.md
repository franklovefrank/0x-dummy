

## Part 1: set up 

1. Install the Solana CLI and Rust toolchain. You can follow the instructions at https://www.solana.com/docs/getting-started/install-solana-cli to install the Solana CLI and https://www.rust-lang.org/tools/install to install the Rust toolchain.


## Part 2: 0x protocol 

1. import 0x protocol contract, compile to WASM using the solc compiler and the wasm-bindgen crate 

2. use the solana-sdk-rust library to connect to a local Solana node and deploy the contract to the Solana blockchain. 

3. Should return pubkey that we can use to call functions on the contract 

## Part 3: Create 2 test ERC20 tokens

1. import 0x erc20 contract,  compile to WASM using the solc compiler and the wasm-bindgen crate 

2. use the solana-sdk-rust library to connect to a local Solana node and deploy the contract to the Solana blockchain. 

3. create 2 new tokens using solana-evm crate and calling constructor function from erc20 contract

## Part 4: Execute Test

1.  connect to a local Solana node

2. load the wasm bytecode of the approriate contracts 

3. fill in appropriate keys/addresses from parts 2 and 3

4. validate result 

