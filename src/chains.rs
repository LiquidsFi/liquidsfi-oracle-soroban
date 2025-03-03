use soroban_sdk::{Env, String, Vec};

use crate::{data::DataKey, types::ChainOracleDetails};

pub fn chain_is_supported(e: &Env, chain_id: u32) -> bool {
    let key = DataKey::ChainOracle(chain_id);
    e.storage().instance().has(&key)
}

pub fn read_chain(e: &Env, chain_id: u32) -> Option<ChainOracleDetails> {
    let key = DataKey::ChainOracle(chain_id);
    e.storage()
        .instance()
        .get(&key)
        .expect("Oracle address not found!")
}

pub fn write_chain(e: &Env, chain_name: String, chain_id: u32, oracle_address: &String) {
    let key = DataKey::ChainOracle(chain_id);
    let chain_details = ChainOracleDetails {
        chain_name: chain_name,
        chain_id: chain_id,
        oracle_address: oracle_address.clone(),
    };
    e.storage().instance().set(&key, &chain_details);

    let mut list = read_chain_id_list(e);
    list.push_back(chain_id);

    let key_list = DataKey::ChainIdList;
    e.storage().instance().set(&key_list, &list);
}

pub fn delete_chain(e: &Env, chain_id: u32) {
    let key = DataKey::ChainOracle(chain_id);
    e.storage().instance().remove(&key);

    let list = read_chain_id_list(e);

    let mut updated_list: Vec<u32> = Vec::new(&e);

    for i in 0..list.len() {
        if list.get_unchecked(i) != chain_id {
            updated_list.push_back(list.get_unchecked(i))
        }
    }

    let key_list = DataKey::ChainIdList;
    e.storage().instance().set(&key_list, &updated_list);
}

pub fn read_chain_id_list(e: &Env) -> Vec<u32> {
    let key = DataKey::ChainIdList;
    let default_list: Vec<u32> = Vec::new(&e);

    e.storage()
        .instance()
        .get::<DataKey, Vec<u32>>(&key)
        .unwrap_or(default_list)
}

pub fn read_supported_chain_list(e: &Env) -> Vec<ChainOracleDetails> {
    let chain_id_list = read_chain_id_list(e);

    let mut chain_list: Vec<ChainOracleDetails> = Vec::new(&e);

    let length = chain_id_list.len();

    if length == 0 {
        return chain_list;
    }

    for i in 0..length {
        let chain_details: ChainOracleDetails =
            read_chain(e, chain_id_list.get_unchecked(i)).unwrap();

        chain_list.push_back(chain_details)
    }

    chain_list
}
