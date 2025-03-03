use crate::types::DecodedTransferData;
use soroban_sdk::{
    xdr::{FromXdr, ToXdr},
    Address, Bytes, Env, String,
};

pub fn decode_transfer_data(env: Env, transfer_bytes: Bytes) -> DecodedTransferData {
    //address starts from 63
    let default = String::from_str(&env, "missing");
    let mut recipient_bytes = "".to_xdr(&env);
    let mut token_bytes = "".to_xdr(&env);
    let mut amount: i128 = 0;
    const FACTOR: i128 = 256;

    recipient_bytes.set(7, transfer_bytes.get_unchecked(159));
    token_bytes.set(7, transfer_bytes.get_unchecked(255));

    let recipient_slice = transfer_bytes.slice(160..216);
    recipient_bytes.append(&recipient_slice);
    let recipient_string = String::from_xdr(&env, &recipient_bytes).unwrap_or(default.clone());

    let recipient_address = Address::from_string(&recipient_string);

    let token_slice = transfer_bytes.slice(256..312);
    token_bytes.append(&token_slice);
    let token_string = String::from_xdr(&env, &token_bytes).unwrap_or(default);
    let token_address = Address::from_string(&token_string);

    let amount_slice = transfer_bytes.slice(112..128);
    for byte in amount_slice {
        amount = amount * FACTOR + byte as i128;
    }

    let transfer_data = DecodedTransferData {
        recipient: recipient_address,
        token: token_address,
        amount: amount,
    };

    transfer_data
}
