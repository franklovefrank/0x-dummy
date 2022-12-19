
# APPROACH
I researched for a little over an hour and came up with the following approach. We'll see :) 

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


# UPDATES
 
1. have 0x contract loaded and rust code to (hopefully) compile it. Not sure if it works because I have to update xcode tools. snack break 
2. switched to solang compiler and was still having issues, particularly with
indirect dependencies not being able to access each other. after i finally got that resolved
I was having type errors for memory/data store. then went on a evm rabbit hole 
3. conclusion from 2: instead of writing new solidity file that imports 0x, better to just 
clone the 0x repo, build it, then compile the necessary wrapper files and create rust bindings with wasm crate. will pick up in the morning. so far spent ~3 hours

## NEW DAY
4. took about 2 minutes of googling to realize 3 was not feasible. i prefer literally anything to js but the 0x project
environment is simply not compatible with rust. maybe it is possible but i think given the current state of
cross-compatibility it is a bad engineering decision. hopefully there will be tools to remedy this kind of limitation
in the near future. 
5. I cloned 0x-starter project during lunch and added a scenario that fulfills the given requirements. the documentation 
is good so making a working example only took a cool 10 min. however it only works right now because i'm transforming 0 eth.
 I still need to figure out what the appropriate transform function is. 
