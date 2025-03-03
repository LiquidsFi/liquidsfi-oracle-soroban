use soroban_sdk::{Bytes, Env, String, Vec};

use crate::{
    data::{DataKey, BUMP_AMOUNT, LIFETIME_THRESHOLD},
    types::{TransferDataReceived, TransferStatus},
};

pub fn read_transaction(e: &Env, tx_id: Bytes) -> Option<TransferDataReceived> {
    let key = DataKey::TransactionDataReceived(tx_id.clone());

    e.storage()
        .persistent()
        .get::<DataKey, TransferDataReceived>(&key)
}

pub fn write_transaction(e: &Env, tx_id: Bytes, transfer_data: TransferDataReceived) {
    let key = DataKey::TransactionDataReceived(tx_id);

    e.storage().persistent().set(&key, &transfer_data);
    e.storage()
        .persistent()
        .extend_ttl(&key, LIFETIME_THRESHOLD, BUMP_AMOUNT);
}

pub fn delete_transaction(e: &Env, tx_id: Bytes) {
    let key = DataKey::TransactionDataReceived(tx_id);

    e.storage().persistent().remove(&key);
}

pub fn read_tx_list(e: &Env) -> Vec<Bytes> {
    let key = DataKey::TransactionList;
    let default_list: Vec<Bytes> = Vec::new(&e);

    e.storage()
        .instance()
        .get::<DataKey, Vec<Bytes>>(&key)
        .unwrap_or(default_list)
}

pub fn write_tx_list(e: &Env, tx_id_vec: Vec<Bytes>) {
    let key = DataKey::TransactionList;
    e.storage().instance().set(&key, &tx_id_vec);
}

pub fn add_tx_to_list(e: &Env, tx_id: Bytes) {
    let mut tx_list = read_tx_list(e);
    tx_list.push_back(tx_id);
    write_tx_list(e, tx_list);
}

pub fn remove_tx_from_list(e: &Env, tx_id: Bytes) {
    let mut tx_list = read_tx_list(e);
    if let Some(tx_index) = tx_list.first_index_of(tx_id) {
        tx_list.remove(tx_index);
        write_tx_list(e, tx_list);
    }
}
