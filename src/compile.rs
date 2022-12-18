extern crate solang;
use std::env;
use std::fs;

pub fn run(path_to_contract: &str) {
    
    let contract_source = fs::read_to_string(path_to_contract).expect("Error reading file");
    
    let wasm = solang::compile(&contract_source, solang::Target::Wasm).unwrap();
    let bindings = solang::generate_rust_bindings("zeroex", &wasm).unwrap();
}


