#![allow(non_snake_case)]
#![no_std]
use soroban_sdk::{contract, contracttype, contractimpl, log, Env, Symbol, String, symbol_short, Address};

// Define contract status enum
#[contracttype]
#[derive(Clone)]
pub enum ContractStatus {
    Active,
    Inactive,
    Pending,
    Deprecated
}

// Structure for tracking deployed contracts
#[contracttype]
#[derive(Clone)]
pub struct DeployedContract {
    pub contract_id: Address,
    pub name: String,
    pub description: String, 
    pub owner: Address,
    pub deploy_timestamp: u64,
    pub status: ContractStatus,
    pub version: String
}

// Mapping contract name to its Address
#[contracttype]
pub enum ContractRegistry {
    Contract(String)
}

// Counter for total contracts deployed through the dashboard
const CONTRACTS_COUNT: Symbol = symbol_short!("COUNT");

#[contract]
pub struct DeploymentDashboard;

#[contractimpl]
impl DeploymentDashboard {
    // Register a newly deployed contract in the dashboard
    pub fn register_contract(
        env: Env, 
        contract_id: Address, 
        name: String, 
        description: String, 
        owner: Address,
        version: String
    ) -> Address {
        // Update the total count of contracts
        let mut count: u64 = env.storage().instance().get(&CONTRACTS_COUNT).unwrap_or(0);
        count += 1;
        
        // Get current timestamp
        let deploy_timestamp = env.ledger().timestamp();
        
        // Create new contract record
        let contract = DeployedContract {
            contract_id: contract_id.clone(),
            name: name.clone(),
            description,
            owner,
            deploy_timestamp,
            status: ContractStatus::Active,
            version
        };
        
        // Store contract data
        env.storage().instance().set(&ContractRegistry::Contract(name.clone()), &contract);
        
        // Update contract count
        env.storage().instance().set(&CONTRACTS_COUNT, &count);
        
        // Extend TTL to ensure data persistence
        env.storage().instance().extend_ttl(10000, 10000);
        
        log!(&env, "Contract registered: {}", name);
        contract_id
    }
    
    // Update the status of a contract
    pub fn update_contract_status(
        env: Env, 
        name: String, 
        status: ContractStatus
    ) {
        let key = ContractRegistry::Contract(name.clone());
        let mut contract: DeployedContract = env.storage().instance().get(&key)
            .expect("Contract not found");
        
        contract.status = status;
        
        // Store updated contract data
        env.storage().instance().set(&key, &contract);
        env.storage().instance().extend_ttl(10000, 10000);
        
        log!(&env, "Contract status updated: {}", name);
    }
    
    // Get contract information by name
    pub fn get_contract(env: Env, name: String) -> DeployedContract {
        let key = ContractRegistry::Contract(name);
        env.storage().instance().get(&key).expect("Contract not found")
    }
    
    // Get total number of contracts registered
    pub fn get_total_contracts(env: Env) -> u64 {
        env.storage().instance().get(&CONTRACTS_COUNT).unwrap_or(0)
    }
}