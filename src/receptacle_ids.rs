use soroban_sdk::{Address, Env, Vec};

use crate::{
    consensus::{read_consensus_threshold, write_consensus_threshold},
    data::DataKey,
    types::ReceptacleDetails,
};

pub fn read_is_operator(e: &Env, operator: Address) -> bool {
    let key = DataKey::ReceptacleId(operator);
    e.storage().instance().has(&key)
}

pub fn read_receptacle_address(e: &Env, operator: Address) -> Option<Address> {
    let key = DataKey::ReceptacleId(operator);
    if let Some(receptacle_addr) = e.storage().instance().get(&key).unwrap() {
        receptacle_addr
    } else {
        None
    }
}

pub fn write_receptacle_address(e: &Env, operator: Address, receptacle_addr: Address) {
    let key = DataKey::ReceptacleId(operator.clone());
    e.storage().instance().set(&key, &receptacle_addr);

    let key_opr = DataKey::OperatorList;
    let mut list: Vec<Address> = read_operator_list(e);
    list.push_back(operator);

    e.storage().instance().set(&key_opr, &list);
    let consensus_val = read_consensus_threshold(e) + 1;
    write_consensus_threshold(e, consensus_val);
}

pub fn delete_receptacle(e: &Env, operator: Address) {
    let key = DataKey::ReceptacleId(operator.clone());
    e.storage().instance().remove(&key);

    let opr_list: Vec<Address> = read_operator_list(e);
    let mut updated_list: Vec<Address> = Vec::new(&e);

    for i in 0..opr_list.len() {
        if opr_list.get_unchecked(i) != operator {
            updated_list.push_back(opr_list.get_unchecked(i))
        }
    }

    let key_opr = DataKey::OperatorList;

    e.storage().instance().set(&key_opr, &updated_list);

    let consensus_val = read_consensus_threshold(e) - 1;
    write_consensus_threshold(e, consensus_val);
}

pub fn read_receptacle_list(e: &Env) -> Vec<ReceptacleDetails> {
    let opr_list = read_operator_list(e);
    let mut default_list: Vec<ReceptacleDetails> = Vec::new(&e);
    if opr_list.len() == 0 {
        default_list
    } else {
        for i in 0..opr_list.len() {
            let opr = opr_list.get_unchecked(i);
            let receptacle_addr = read_receptacle_address(e, opr.clone()).unwrap();

            let receptacle_details = ReceptacleDetails {
                operator: opr,
                receptacle_address: receptacle_addr,
            };

            default_list.push_back(receptacle_details)
        }
        default_list
    }
}

pub fn read_operator_list(e: &Env) -> Vec<Address> {
    let key = DataKey::OperatorList;
    let default_list: Vec<Address> = Vec::new(&e);

    e.storage()
        .instance()
        .get::<DataKey, Vec<Address>>(&key)
        .unwrap_or(default_list)
}
