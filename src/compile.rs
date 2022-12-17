extern crate solc_build_script;
extern crate wasm_bindgen;

use std::process::Command;

pub fn run(args: Vec<String>) {

    // Access the first argument
    let path_to_contract = &args[1];
    println!("The path to contract is: {}", path_to_contract);


    // Compile the Solidity contract to WASM
    let output = Command::new("solc")
        .args(&["--bin", "--abi", path_to_contract])
        .output()
        .unwrap();
    let wasm = output.stdout;
    let abi = output.stderr;

    // Generate Rust bindings for the contract
    wasm_bindgen::Builder::new()
        .wasm_binary(wasm)
        .import_memory(true)
        .raw_line("extern crate solc_output;")
        .parse_callbacks(abi)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file("output.rs");
}
