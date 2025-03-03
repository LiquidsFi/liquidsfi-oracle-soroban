use soroban_sdk::{Bytes, Env, Vec};

use crate::data::DataKey;

pub fn read_consensus_threshold(e: &Env) -> u32 {
    let key = DataKey::ConsensusThreshold;
    e.storage().instance().get(&key).unwrap_or(0)
}

pub fn write_consensus_threshold(e: &Env, consensus_val: u32) {
    let key = DataKey::ConsensusThreshold;
    e.storage().instance().set(&key, &consensus_val);
}

pub fn read_consensus_count(e: &Env, data: Bytes) -> u32 {
    let key = DataKey::ConsensusCount(data);
    e.storage().instance().get(&key).unwrap_or(0)
}

pub fn write_consensus_count(e: &Env, data: Bytes, count: u32) {
    let key = DataKey::ConsensusCount(data);
    e.storage().instance().set(&key, &count);
}

pub fn clear_consensus_count(e: &Env, data: Bytes) {
    let key = DataKey::ConsensusCount(data);
    e.storage().instance().remove(&key);
}

pub fn threshold_validation(e: &Env, data_list: Vec<Bytes>) -> Option<Bytes> {
    if data_list.len() == 0 {
        return None;
    }
    let consensus_threshold = read_consensus_threshold(e);
    let mut distinct_data_list: Vec<Bytes> = Vec::new(e);

    // Count occurrences of each transaction hash
    for i in 0..data_list.len() {
        let tx_data = data_list.get_unchecked(i);
        let count = read_consensus_count(e, tx_data.clone()) + 1;

        if !distinct_data_list.contains(tx_data.clone()) {
            distinct_data_list.push_back(tx_data.clone());
        }

        write_consensus_count(e, tx_data, count);
    }

    // Find the most common hash that meets the consensus threshold
    for i in 0..distinct_data_list.len() {
        let validated_data = distinct_data_list.get_unchecked(i);
        let validation_count = read_consensus_count(e, validated_data.clone());
        if validation_count >= consensus_threshold {
            return Some(validated_data);
        }
    }

    None // No consensus reached
}
