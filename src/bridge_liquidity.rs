#![allow(unused)]
use crate::{error::ContractError, types::AllFees, types::TransferDataReceived};
use soroban_sdk::{xdr::ToXdr, Address, Bytes, BytesN, Env, String};

soroban_sdk::contractimport!(
    file = "../liquidsfi-bridge-liquidity/target/wasm32-unknown-unknown/release/liquidsfi_bridge_liquidity.wasm"
);
