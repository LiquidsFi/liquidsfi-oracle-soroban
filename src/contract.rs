use soroban_sdk::{
    contract, contractimpl, symbol_short,
    xdr::{FromXdr, ToXdr},
    Address, Bytes, BytesN, Env, FromVal, String, Symbol, Val, Vec,
};

use crate::{
    access::{
        authenticate_admin, has_admin, has_bridge_contract, read_admin, read_bridge_contract,
        write_admin, write_bridge_contract,
    },
    bridge_liquidity,
    chains::{
        chain_is_supported, delete_chain, read_chain, read_supported_chain_list, write_chain,
    },
    consensus::{clear_consensus_count, read_consensus_threshold, threshold_validation},
    error::ContractError,
    oracle_receptacle::{self, create_receptacle},
    receptacle_ids::{
        delete_receptacle, read_is_operator, read_receptacle_address, read_receptacle_list,
        write_receptacle_address,
    },
    tokens::{
        delete_token_chain_map, read_destination_chain_token, read_destination_token_list,
        read_token_chain_is_supported, read_token_is_supported, write_token_chain_map,
    },
    types::{ChainOracleDetails, DestinationChainDetails, ReceptacleDetails, TransferData},
};

const TRANSFER: Symbol = symbol_short!("TRANSFER");

pub trait OracleTrait {
    fn initialize(e: Env, admin: Address) -> Result<(), ContractError>;
    fn setup_new_node(e: Env, operator: Address) -> Result<Address, ContractError>;
    fn setup_bridge(e: Env, bridge_contract: Address);
    fn remove_node(e: Env, operator: Address) -> Result<(), ContractError>;

    fn add_new_chain(
        e: Env,
        chain_name: String,
        chain_id: u32,
        oracle_address: String,
    ) -> Result<(), ContractError>;
    fn remove_chain(e: Env, chain_id: u32) -> Result<(), ContractError>;

    fn add_token_destination_map(
        e: Env,
        token_id: Address,
        chain_id: u32,
        destination_token: String,
    ) -> Result<(), ContractError>;

    fn remove_token_destination_map(
        e: Env,
        token_id: Address,
        chain_id: u32,
    ) -> Result<(), ContractError>;

    fn get_destination_chain_token(
        e: Env,
        token_id: Address,
        chain_id: u32,
    ) -> Result<DestinationChainDetails, ContractError>;

    fn get_destination_chain_list(
        e: Env,
        token_id: Address,
    ) -> Result<Vec<DestinationChainDetails>, ContractError>;

    fn get_consensus_threshold(e: Env) -> u32;

    fn initiate_outgoing_transfer(
        e: Env,
        user: Address,
        chain_id: u32,
        recipient: String,
        token_id: Address,
        amount: i128,
    ) -> Result<u64, ContractError>;
    fn initiate_rebalancing(
        e: Env,
        user: Address,
        chain_id: u32,
        recipient: String,
        token_id: Address,
        amount: i128,
    ) -> Result<u64, ContractError>;

    fn perform_upkeep(e: &Env);
    fn get_upkeep_required(e: &Env) -> bool;

    fn get_receptacle_address(e: Env, operator: Address) -> Result<Address, ContractError>;

    fn get_receptacle_list(e: Env) -> Vec<ReceptacleDetails>;

    fn get_all_supported_chains(e: Env) -> Vec<ChainOracleDetails>;
    // fn get_consensus_threshold(e: Env) -> u32;

    fn get_admin(e: Env) -> Address;

    fn upgrade(e: Env, new_wasm_hash: BytesN<32>);
}

#[contract]
pub struct Oracle;

#[contractimpl]
impl OracleTrait for Oracle {
    fn initialize(e: Env, admin: Address) -> Result<(), ContractError> {
        let is_initialized = has_admin(&e);
        if is_initialized {
            return Err(ContractError::AlreadyInitialized);
        }
        write_admin(&e, &admin);
        Ok(())
    }

    fn setup_new_node(e: Env, operator: Address) -> Result<Address, ContractError> {
        authenticate_admin(&e);

        if read_is_operator(&e, operator.clone()) {
            return Err(ContractError::AlreadyNodeOperator);
        }
        let receptacle_address = create_receptacle(&e, &operator.clone());

        let receptacle_contract = oracle_receptacle::Client::new(&e, &receptacle_address);

        receptacle_contract.initialize(&e.current_contract_address(), &operator);

        write_receptacle_address(&e, operator, receptacle_address.clone());

        Ok(receptacle_address)
    }

    fn setup_bridge(e: Env, bridge_contract: Address) {
        authenticate_admin(&e);

        write_bridge_contract(&e, &bridge_contract);
    }

    fn remove_node(e: Env, operator: Address) -> Result<(), ContractError> {
        authenticate_admin(&e);

        if !read_is_operator(&e, operator.clone()) {
            return Err(ContractError::NodeNotFound);
        }

        delete_receptacle(&e, operator);

        Ok(())
    }

    fn add_new_chain(
        e: Env,
        chain_name: String,
        chain_id: u32,
        oracle_address: String,
    ) -> Result<(), ContractError> {
        authenticate_admin(&e);

        if chain_is_supported(&e, chain_id) {
            return Err(ContractError::ChainAlreadyAdded);
        }

        write_chain(&e, chain_name, chain_id, &oracle_address);

        Ok(())
    }

    fn remove_chain(e: Env, chain_id: u32) -> Result<(), ContractError> {
        authenticate_admin(&e);

        if !chain_is_supported(&e, chain_id) {
            return Err(ContractError::ChainNotFound);
        }
        delete_chain(&e, chain_id);

        Ok(())
    }

    fn add_token_destination_map(
        e: Env,
        token_id: Address,
        chain_id: u32,
        destination_token: String,
    ) -> Result<(), ContractError> {
        authenticate_admin(&e);

        if !chain_is_supported(&e, chain_id) {
            return Err(ContractError::ChainNotFound);
        }

        write_token_chain_map(&e, token_id, chain_id, destination_token);

        Ok(())
    }

    fn remove_token_destination_map(
        e: Env,
        token_id: Address,
        chain_id: u32,
    ) -> Result<(), ContractError> {
        authenticate_admin(&e);

        if !read_token_is_supported(&e, token_id.clone()) {
            return Err(ContractError::TokenNotSupported);
        }

        if !chain_is_supported(&e, chain_id.clone()) {
            return Err(ContractError::ChainNotFound);
        }

        if !read_token_chain_is_supported(&e, token_id.clone(), chain_id.clone()) {
            return Err(ContractError::TokenChainMapNotFound);
        }

        delete_token_chain_map(&e, token_id, chain_id);

        Ok(())
    }

    fn get_destination_chain_token(
        e: Env,
        token_id: Address,
        chain_id: u32,
    ) -> Result<DestinationChainDetails, ContractError> {
        if !read_token_is_supported(&e, token_id.clone()) {
            return Err(ContractError::TokenNotSupported);
        }

        if !chain_is_supported(&e, chain_id.clone()) {
            return Err(ContractError::ChainNotFound);
        }

        if !read_token_chain_is_supported(&e, token_id.clone(), chain_id.clone()) {
            return Err(ContractError::TokenChainMapNotFound);
        }

        let details: DestinationChainDetails =
            read_destination_chain_token(&e, token_id, chain_id).unwrap();

        Ok(details)
    }

    fn get_destination_chain_list(
        e: Env,
        token_id: Address,
    ) -> Result<Vec<DestinationChainDetails>, ContractError> {
        if !read_token_is_supported(&e, token_id.clone()) {
            return Err(ContractError::TokenNotSupported);
        }

        let list: Vec<DestinationChainDetails> = read_destination_token_list(&e, token_id);

        Ok(list)
    }

    fn initiate_outgoing_transfer(
        e: Env,
        user: Address,
        chain_id: u32,
        recipient: String,
        token_id: Address,
        amount: i128,
    ) -> Result<u64, ContractError> {
        user.require_auth();
        if !has_bridge_contract(&e) {
            return Err(ContractError::BridgeNotFound);
        }

        if !read_token_is_supported(&e, token_id.clone()) {
            return Err(ContractError::TokenNotSupported);
        }

        if !chain_is_supported(&e, chain_id.clone()) {
            return Err(ContractError::ChainNotFound);
        }

        if !read_token_chain_is_supported(&e, token_id.clone(), chain_id.clone()) {
            return Err(ContractError::TokenChainMapNotFound);
        }

        let bridge_client = bridge_liquidity::Client::new(&e, &read_bridge_contract(&e).unwrap());

        bridge_client.transfer_soroban_to_evm(&user, &token_id, &amount);

        let tx_id: u64 = e.prng().gen();

        let destination_token = read_destination_chain_token(&e, token_id, chain_id)
            .unwrap()
            .destination_token;

        let chain_oracle = read_chain(&e, chain_id).unwrap().oracle_address;

        let data = TransferData {
            recipient: recipient,
            token: destination_token,
            amount: amount,
        };

        e.events().publish(
            (TRANSFER, symbol_short!("outgoing")),
            (tx_id, chain_id, chain_oracle, user, data),
        );

        Ok(tx_id)
    }

    fn initiate_rebalancing(
        e: Env,
        user: Address,
        chain_id: u32,
        recipient: String,
        token_id: Address,
        amount: i128,
    ) -> Result<u64, ContractError> {
        user.require_auth();
        if !has_bridge_contract(&e) {
            return Err(ContractError::BridgeNotFound);
        }

        if !read_token_is_supported(&e, token_id.clone()) {
            return Err(ContractError::TokenNotSupported);
        }

        if !chain_is_supported(&e, chain_id.clone()) {
            return Err(ContractError::ChainNotFound);
        }

        if !read_token_chain_is_supported(&e, token_id.clone(), chain_id.clone()) {
            return Err(ContractError::TokenChainMapNotFound);
        }

        let bridge_client = bridge_liquidity::Client::new(&e, &read_bridge_contract(&e).unwrap());

        bridge_client.execute_rebalancing(&user, &token_id, &amount);

        let tx_id: u64 = e.prng().gen();

        let destination_token = read_destination_chain_token(&e, token_id, chain_id)
            .unwrap()
            .destination_token;

        let chain_oracle = read_chain(&e, chain_id).unwrap().oracle_address;

        let data = TransferData {
            recipient: recipient,
            token: destination_token,
            amount: amount,
        };

        e.events().publish(
            (TRANSFER, symbol_short!("outgoing")),
            (tx_id, chain_id, chain_oracle, user, data),
        );

        Ok(tx_id)
    }

    fn perform_upkeep(e: &Env) {
        let all_receptacles: Vec<ReceptacleDetails> = read_receptacle_list(e);

        if all_receptacles.len() == 0 {
            panic!("No receptacles found")
        }

        // let length: u64 = all_receptacles.len() as u64;
        // let seeder_node_index: u64 = e.prng().gen_range(0..length);

        let seeder_node_address: Address =
            all_receptacles.get_unchecked(0 as u32).receptacle_address;

        let receptacle_contract = oracle_receptacle::Client::new(&e, &seeder_node_address);

        let tx_ids_val: Val = receptacle_contract.get_transfer_id_list();
        let tx_id_list: Vec<Bytes> = Vec::from_val(e, &tx_ids_val);

        // if tx_id_list.len() == 0 {
        //     panic!("No transaction found");
        // }

        if tx_id_list.len() > 0 {
            let mut data_list_reset: Vec<Bytes> = Vec::new(e);

            let limit = u32::min(tx_id_list.len(), 5);

            for i in 0..limit {
                let mut data_list: Vec<Bytes> = Vec::new(e);

                let tx_id = tx_id_list.get_unchecked(i);
                for j in 0..all_receptacles.len() {
                    let receptacle_addr = all_receptacles.get_unchecked(j).receptacle_address;
                    let receptacle_contract = oracle_receptacle::Client::new(&e, &receptacle_addr);
                    let tx_data_val = receptacle_contract.get_transfer(&tx_id);
                    let tx_data_array: Vec<Bytes> = Vec::from_val(e, &tx_data_val);
                    let data_bytes = tx_data_array.clone().to_xdr(&e);
                    data_list.push_back(data_bytes.clone());
                    data_list_reset.push_back(data_bytes.clone());
                }

                if let Some(validated_data_bytes) = threshold_validation(e, data_list.clone()) {
                    let default: Vec<Bytes> = Vec::new(&e);
                    let validated_data_array: Vec<Bytes> =
                        Vec::from_xdr(&e, &validated_data_bytes).unwrap_or(default);

                    if validated_data_array.len() > 5 {
                        let tx_id = validated_data_array.get_unchecked(0);

                        let recipient =
                            Address::from_xdr(e, &validated_data_array.get_unchecked(3))
                                .ok()
                                .unwrap();
                        let token = Address::from_xdr(e, &validated_data_array.get_unchecked(4))
                            .ok()
                            .unwrap();
                        let amount = i128::from_xdr(e, &validated_data_array.get_unchecked(5))
                            .ok()
                            .unwrap();

                        let bridge_client =
                            bridge_liquidity::Client::new(&e, &read_bridge_contract(e).unwrap());

                        bridge_client.execute_evm_to_soroban_transfer(&recipient, &token, &amount);

                        for j in 0..all_receptacles.len() {
                            let receptacle_addr =
                                all_receptacles.get_unchecked(j).receptacle_address;
                            let receptacle_contract =
                                oracle_receptacle::Client::new(&e, &receptacle_addr);
                            receptacle_contract.remove_transfer(&tx_id);
                        }
                    }
                };

                for i in 0..data_list_reset.len() {
                    let tx_data = data_list_reset.get_unchecked(i);

                    clear_consensus_count(e, tx_data);
                }
            }
        }
    }

    fn get_upkeep_required(e: &Env) -> bool {
        let all_receptacles: Vec<ReceptacleDetails> = read_receptacle_list(e);

        if all_receptacles.is_empty() {
            return false;
        }

        let seeder_node_address: Address = all_receptacles.get_unchecked(0).receptacle_address;

        let receptacle_contract = oracle_receptacle::Client::new(&e, &seeder_node_address);

        let tx_ids_val: Val = receptacle_contract.get_transfer_id_list();
        let tx_id_list: Vec<Bytes> = Vec::from_val(e, &tx_ids_val);

        !tx_id_list.is_empty()
    }

    fn get_receptacle_address(e: Env, operator: Address) -> Result<Address, ContractError> {
        if let Some(receptacle_address) = read_receptacle_address(&e, operator) {
            Ok(receptacle_address)
        } else {
            return Err(ContractError::ReceptacleNotFound);
        }
    }

    fn get_receptacle_list(e: Env) -> Vec<ReceptacleDetails> {
        read_receptacle_list(&e)
    }

    fn get_all_supported_chains(e: Env) -> Vec<ChainOracleDetails> {
        read_supported_chain_list(&e)
    }

    fn get_consensus_threshold(e: Env) -> u32 {
        read_consensus_threshold(&e)
    }

    fn get_admin(e: Env) -> Address {
        let admin = read_admin(&e).unwrap();
        admin
    }

    fn upgrade(e: Env, new_wasm_hash: BytesN<32>) {
        authenticate_admin(&e);
        e.deployer().update_current_contract_wasm(new_wasm_hash);
    }
}
