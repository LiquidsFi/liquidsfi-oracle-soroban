use soroban_sdk::{contracttype, Address, Bytes};

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Admin,
    Managers,
    BridgeContract,
    ReceptacleId(Address),
    OperatorList,
    ReceptaclesList,
    ConsensusThreshold,
    ConsensusCount(Bytes),

    ChainOracle(u32),
    IsSuportedChain(u32),
    ChainIdList,

    TokenSupportedChains(Address), //array of all chain ids a token supports
    DestinationChainToken(Address, u32), //destination token address
}
