use soroban_sdk::{Address, Env, String, Vec};

use crate::{chains::read_chain, data::DataKey, types::DestinationChainDetails};

// ReceptacleId(Address),
// IsOperator(Address),
// ReceptaclesList,
// ConsensusThreshold,

pub fn read_token_is_supported(e: &Env, token_id: Address) -> bool {
    let key = DataKey::TokenSupportedChains(token_id);
    e.storage().instance().has(&key)
}

pub fn read_token_chain_is_supported(e: &Env, token_id: Address, chain_id: u32) -> bool {
    let key = DataKey::DestinationChainToken(token_id, chain_id);
    e.storage().instance().has(&key)
}

pub fn write_token_chain_map(e: &Env, token_id: Address, chain_id: u32, destination_token: String) {
    let key = DataKey::DestinationChainToken(token_id.clone(), chain_id);
    let chain_details = read_chain(e, chain_id).unwrap();
    let destination_token_details = DestinationChainDetails {
        chain_name: chain_details.chain_name,
        chain_id: chain_id,
        destination_token: destination_token,
    };

    if !read_token_chain_is_supported(e, token_id.clone(), chain_id) {
        let key_chain_ids = DataKey::TokenSupportedChains(token_id.clone());
        let mut list: Vec<u32> = read_token_supported_chain_list(e, token_id.clone());
        list.push_back(chain_id);

        e.storage().instance().set(&key_chain_ids, &list);
    }

    e.storage().instance().set(&key, &destination_token_details);
}

pub fn delete_token_chain_map(e: &Env, token_id: Address, chain_id: u32) {
    let key = DataKey::DestinationChainToken(token_id.clone(), chain_id);
    e.storage().instance().remove(&key);

    let chain_list: Vec<u32> = read_token_supported_chain_list(e, token_id.clone());
    let mut updated_list: Vec<u32> = Vec::new(&e);

    for i in 0..chain_list.len() {
        if chain_list.get_unchecked(i) != chain_id {
            updated_list.push_back(chain_list.get_unchecked(i))
        }
    }

    let key_chain = DataKey::TokenSupportedChains(token_id);

    e.storage().instance().set(&key_chain, &updated_list);
}

pub fn read_destination_chain_token(
    e: &Env,
    token_id: Address,
    chain_id: u32,
) -> Option<DestinationChainDetails> {
    let key = DataKey::DestinationChainToken(token_id, chain_id);
    e.storage()
        .instance()
        .get(&key)
        .expect("Destination token details not found!")
}

pub fn read_destination_token_list(e: &Env, token_id: Address) -> Vec<DestinationChainDetails> {
    let chain_list = read_token_supported_chain_list(e, token_id.clone());
    let mut default_list: Vec<DestinationChainDetails> = Vec::new(&e);
    if chain_list.len() == 0 {
        default_list
    } else {
        for i in 0..chain_list.len() {
            let chain_id = chain_list.get_unchecked(i);
            let chain_details: DestinationChainDetails =
                read_destination_chain_token(e, token_id.clone(), chain_id).unwrap();

            default_list.push_back(chain_details)
        }
        default_list
    }
}

pub fn read_token_supported_chain_list(e: &Env, token_id: Address) -> Vec<u32> {
    let key = DataKey::TokenSupportedChains(token_id);
    let default_list: Vec<u32> = Vec::new(&e);

    e.storage()
        .instance()
        .get::<DataKey, Vec<u32>>(&key)
        .unwrap_or(default_list)
}
