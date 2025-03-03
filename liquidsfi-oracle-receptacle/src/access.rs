use soroban_sdk::{Address, Env};

use crate::data::DataKey;

pub fn read_oracle(e: &Env) -> Option<Address> {
    let key = DataKey::Oracle;
    e.storage().instance().get(&key).unwrap()
}

pub fn write_oracle(e: &Env, oracle: &Address) {
    let key = DataKey::Oracle;
    e.storage().instance().set(&key, oracle);
}

pub fn has_operator(e: &Env) -> bool {
    let key = DataKey::NodeOperator;
    e.storage().instance().has(&key)
}

pub fn read_operator(e: &Env) -> Option<Address> {
    let key = DataKey::NodeOperator;
    e.storage().instance().get(&key).unwrap()
}

pub fn write_operator(e: &Env, node_operator: &Address) {
    let key = DataKey::NodeOperator;
    e.storage().instance().set(&key, node_operator);
}

pub fn read_payer(e: &Env) -> Option<Address> {
    let key = DataKey::Payer;
    e.storage().instance().get(&key).unwrap()
}

pub fn write_payer(e: &Env, payer: &Address) {
    let key = DataKey::Payer;
    e.storage().instance().set(&key, payer);
}
