use soroban_sdk::{Address, Env};

use crate::data::DataKey;

pub fn has_admin(e: &Env) -> bool {
    let key = DataKey::Admin;
    e.storage().instance().has(&key)
}

pub fn read_admin(e: &Env) -> Option<Address> {
    let key = DataKey::Admin;
    e.storage().instance().get(&key).expect("Admin not found!")
}

pub fn write_admin(e: &Env, admin: &Address) {
    let key = DataKey::Admin;
    e.storage().instance().set(&key, admin);
}

pub fn authenticate_admin(e: &Env) {
    let admin = read_admin(e).unwrap();
    admin.require_auth();
}

pub fn read_manager(e: &Env) -> Option<Address> {
    let key = DataKey::Managers;
    e.storage().instance().get(&key).expect("Admin not found!")
}

pub fn write_manager(e: &Env, manager: &Address) {
    let key = DataKey::Managers;
    e.storage().instance().set(&key, manager);
}

pub fn authenticate_manager(e: &Env) {
    let manager = read_manager(e).unwrap();
    manager.require_auth();
}

pub fn read_bridge_contract(e: &Env) -> Option<Address> {
    let key = DataKey::BridgeContract;
    e.storage().instance().get(&key).expect("Admin not found!")
}

pub fn write_bridge_contract(e: &Env, bridge_contract: &Address) {
    let key = DataKey::BridgeContract;
    e.storage().instance().set(&key, bridge_contract);
}

pub fn has_bridge_contract(e: &Env) -> bool {
    let key = DataKey::BridgeContract;
    e.storage().instance().has(&key)
}
