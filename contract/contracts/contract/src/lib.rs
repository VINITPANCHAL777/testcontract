#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, Address, Env, Symbol, Map
};

#[contract]
pub struct PiggyBank;

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Owners,
    Balances,
}

#[contractimpl]
impl PiggyBank {

    // 🪙 Create a new piggy bank (permissionless)
    pub fn create(env: Env, owner: Address, id: Symbol) {
        owner.require_auth();

        let mut owners: Map<Symbol, Address> =
            env.storage()
                .instance()
                .get(&DataKey::Owners)
                .unwrap_or(Map::new(&env));

        // Prevent duplicate IDs
        if owners.contains_key(id.clone()) {
            panic!("Bank ID already exists");
        }

        owners.set(id.clone(), owner);
        env.storage().instance().set(&DataKey::Owners, &owners);

        let mut balances: Map<Symbol, i128> =
            env.storage()
                .instance()
                .get(&DataKey::Balances)
                .unwrap_or(Map::new(&env));

        balances.set(id, 0);
        env.storage().instance().set(&DataKey::Balances, &balances);
    }

    // 💰 Deposit (anyone can deposit → permissionless)
    pub fn deposit(env: Env, id: Symbol, amount: i128) {
        if amount <= 0 {
            panic!("Invalid amount");
        }

        let mut balances: Map<Symbol, i128> =
            env.storage()
                .instance()
                .get(&DataKey::Balances)
                .unwrap();

        let current = balances.get(id.clone()).unwrap_or(0);
        balances.set(id, current + amount);

        env.storage().instance().set(&DataKey::Balances, &balances);
    }

    // 🏧 Withdraw (only owner)
    pub fn withdraw(env: Env, id: Symbol, caller: Address) -> i128 {
        caller.require_auth();

        let owners: Map<Symbol, Address> =
            env.storage()
                .instance()
                .get(&DataKey::Owners)
                .unwrap();

        let owner = owners.get(id.clone()).unwrap();

        if owner != caller {
            panic!("Not owner");
        }

        let mut balances: Map<Symbol, i128> =
            env.storage()
                .instance()
                .get(&DataKey::Balances)
                .unwrap();

        let amount = balances.get(id.clone()).unwrap_or(0);

        if amount <= 0 {
            panic!("No funds");
        }

        balances.set(id, 0);
        env.storage().instance().set(&DataKey::Balances, &balances);

        amount
    }

    // 📊 Get balance
    pub fn get_balance(env: Env, id: Symbol) -> i128 {
        let balances: Map<Symbol, i128> =
            env.storage()
                .instance()
                .get(&DataKey::Balances)
                .unwrap();

        balances.get(id).unwrap_or(0)
    }

    // 👤 Get owner
    pub fn get_owner(env: Env, id: Symbol) -> Address {
        let owners: Map<Symbol, Address> =
            env.storage()
                .instance()
                .get(&DataKey::Owners)
                .unwrap();

        owners.get(id).unwrap()
    }
}