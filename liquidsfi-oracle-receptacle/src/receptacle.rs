use soroban_sdk::{contract, contractimpl, xdr::ToXdr, Address, Bytes, Env, String, Val, Vec};

use crate::{
    access::{
        has_operator, read_operator, read_oracle, read_payer, write_operator, write_oracle,
        write_payer,
    },
    decode::decode_transfer_data,
    transfers::{
        add_tx_to_list, delete_transaction, read_transaction, read_tx_list, remove_tx_from_list,
        write_transaction,
    },
    types::{TransferDataReceived, TransferStatus},
};

pub trait ReceptacleTrait {
    fn initialize(e: Env, oracle: Address, node_operator: Address);
    fn set_operator(e: Env, node_operator: Address);
    fn set_payer(e: Env, payer: Address);
    fn commit_transfer(
        e: Env,
        tx_id: Bytes,
        origin_chain: u32,
        sender: String,
        transfer_data: Bytes,
    );
    fn remove_transfer(e: Env, tx_id: Bytes);

    fn get_transfer_id_list(e: Env) -> Val;

    fn get_transfer(e: Env, tx_id: Bytes) -> Val;
    fn get_payer(e: Env) -> Address;
}

#[contract]
pub struct Receptacle;

#[contractimpl]
impl ReceptacleTrait for Receptacle {
    fn initialize(e: Env, oracle: Address, node_operator: Address) {
        if has_operator(&e) {
            panic!("Tentacle has already been initialized")
        }
        write_oracle(&e, &oracle);
        write_operator(&e, &node_operator);
        write_payer(&e, &node_operator)
    }

    fn set_operator(e: Env, node_operator: Address) {
        let cur_operator = read_operator(&e).unwrap();
        cur_operator.require_auth();
        write_operator(&e, &node_operator);
    }
    fn set_payer(e: Env, payer: Address) {
        let operator = read_operator(&e).unwrap();
        operator.require_auth();
        write_payer(&e, &payer);
    }

    fn commit_transfer(
        e: Env,
        tx_id: Bytes,
        origin_chain: u32,
        sender: String,
        transfer_data: Bytes,
    ) {
        let node_operator = read_operator(&e).unwrap();
        node_operator.require_auth();

        let decoded_transfer_data = decode_transfer_data(e.clone(), transfer_data);

        let received_data = TransferDataReceived {
            tx_id: tx_id.clone(),
            origin_chain: origin_chain,
            sender: sender,
            recipient: decoded_transfer_data.recipient,
            token: decoded_transfer_data.token,
            amount: decoded_transfer_data.amount,
            status: TransferStatus::Received,
        };
        write_transaction(&e, tx_id.clone(), received_data);
        add_tx_to_list(&e, tx_id);
    }

    fn remove_transfer(e: Env, tx_id: Bytes) {
        let oracle = read_oracle(&e).unwrap();
        oracle.require_auth();

        delete_transaction(&e, tx_id.clone());
        remove_tx_from_list(&e, tx_id);
    }

    fn get_transfer_id_list(e: Env) -> Val {
        let list = read_tx_list(&e);
        list.to_val()
    }

    // fn get_transfer(e: Env, tx_id: Bytes) -> Result<TransferDataReceived, ContractError> {
    //     if let Some(tx_data) = read_transaction(&e, tx_id) {

    //         Ok(tx_data)
    //     } else {
    //         Err(ContractError::TransferNotFound)
    //     }
    // }

    // fn get_payer(e: Env) -> Address {
    //     read_payer(&e).unwrap()
    // }

    fn get_transfer(e: Env, tx_id: Bytes) -> Val {
        let mut data_arr: Vec<Bytes> = Vec::new(&e);

        if let Some(tx_data) = read_transaction(&e, tx_id) {
            data_arr.push_back(tx_data.clone().tx_id);
            data_arr.push_back(tx_data.clone().origin_chain.to_xdr(&e));
            data_arr.push_back(tx_data.clone().sender.to_xdr(&e));
            data_arr.push_back(tx_data.clone().recipient.to_xdr(&e));
            data_arr.push_back(tx_data.clone().token.to_xdr(&e));
            data_arr.push_back(tx_data.clone().amount.to_xdr(&e));
        }

        data_arr.to_val()
    }

    fn get_payer(e: Env) -> Address {
        read_payer(&e).unwrap()
    }
}
