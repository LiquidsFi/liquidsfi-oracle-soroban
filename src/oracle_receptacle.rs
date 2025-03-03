#![allow(unused)]
use crate::{error::ContractError, types::TransferDataReceived};
use soroban_sdk::{xdr::ToXdr, Address, Bytes, BytesN, Env, String};

soroban_sdk::contractimport!(
    file = "./liquidsfi-oracle-receptacle/target/wasm32-unknown-unknown/release/liquidsfi_oracle_receptacle.wasm"
);

const WASM_IMPORT: &[u8] = include_bytes!(".././liquidsfi-oracle-receptacle/target/wasm32-unknown-unknown/release/liquidsfi_oracle_receptacle.wasm");

pub fn create_receptacle(e: &Env, operator: &Address) -> Address {
    let tentacle_wasm = e.deployer().upload_contract_wasm(WASM_IMPORT);
    let mut salt = Bytes::new(e);
    salt.append(&operator.to_xdr(e));
    let salt = e.crypto().sha256(&salt);
    let tentacle_addr = e
        .deployer()
        .with_current_contract(salt)
        .deploy_v2(tentacle_wasm, ());
    tentacle_addr
}
