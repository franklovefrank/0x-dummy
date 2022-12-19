use solana_sdk::{
    account::Account,
    client::Client,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    transaction::Transaction,
};

pub fn run(path_to_wasm: String) {
    // Connect to a local Solana node
    let client = Client::new("http://localhost:8899");

    // Load the WASM bytecode of the 0x contract
    let wasm = include_bytes!(&path_to_wasm);

    // Create a keypair to sign the transaction
    let keypair = Keypair::new();

    // Create a new account to deploy the contract to
    let contract_account = Account::new(wasm.len() as u64, 0, &wasm);

    // Deploy the contract
    let tx = Transaction::new_contract_create(
        &[&keypair],
        contract_account,
        0,
        100_000,
    );

    let signature = client.send_and_confirm_transaction(&[&keypair], tx).unwrap();

    // Get the contract's pubkey
    let contract_pubkey = Pubkey::new_from_array(signature.as_ref());
}

