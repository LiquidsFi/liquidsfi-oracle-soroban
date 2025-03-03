use soroban_sdk::{contracttype, Bytes};

pub(crate) const DAY_IN_LEDGERS: u32 = 17280;
pub(crate) const BUMP_AMOUNT: u32 = 30 * DAY_IN_LEDGERS;
pub(crate) const LIFETIME_THRESHOLD: u32 = BUMP_AMOUNT - DAY_IN_LEDGERS;

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Oracle,
    NodeOperator,
    Payer,
    TransactionDataReceived(Bytes),
    TransactionList,
}
