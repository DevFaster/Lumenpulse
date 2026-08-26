use soroban_sdk::{Address, Symbol};

#[derive(Clone)]
pub struct InitializedEvent {
    pub admin: Address,
}

#[derive(Clone)]
pub struct ContractRegisteredEvent {
    pub key: Symbol,
    pub address: Address,
    pub version: u32,
    pub env: Symbol,
}

#[derive(Clone)]
pub struct ContractUpdatedEvent {
    pub key: Symbol,
    pub version: u32,
    pub env: Symbol,
}
