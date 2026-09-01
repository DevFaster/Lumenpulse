use soroban_sdk::{Address, Symbol, Vec};

#[derive(Clone)]
pub struct ContractInfo {
    pub key: Symbol,
    pub address: Address,
    pub version: u32,
    pub environment: Symbol,
}

pub enum DataKey {
    Admin,
    Paused,
    Contract(Symbol), // maps contract key to ContractInfo
    ContractKeys,     // Vec<Symbol> of all registered contract keys
}
