#![no_std]

use soroban_sdk::{contract, contractimpl, symbol_short, Address, Env, Map, Symbol};

#[contract]
pub struct VotingContract;

#[contractimpl]
impl VotingContract {
    pub fn vote(env: Env, candidate: Symbol) {
        let sender: Address = env.invoker(); // doğru metot

        // "voted_<addr>" şeklinde anahtar üretelim
        let voted_key = create_storage_key(&env, "voted", &sender.to_string());

        let has_voted: bool = env.storage().instance().get(&voted_key).unwrap_or(false);
        if has_voted {
            panic!("Zaten oy verdiniz!");
        }

        // "votes_<candidate>" şeklinde anahtar üretelim
        let votes_key = create_storage_key(&env, "votes", &candidate.to_string());
        let current_votes: i32 = env.storage().instance().get(&votes_key).unwrap_or(0);
        env.storage().instance().set(&votes_key, &(current_votes + 1));

        // Oy kullandığını işaretle
        env.storage().instance().set(&voted_key, &true);
    }

    pub fn get_votes(env: Env) -> Map<Symbol, i32> {
        let mut results = Map::new(&env);
        for name in ["Alice", "Bob", "Carol"] {
            let sym = Symbol::new(&env, name);
            let key = create_storage_key(&env, "votes", name);
            let count: i32 = env.storage().instance().get(&key).unwrap_or(0);
            results.set(sym, count);
        }
        results
    }
}

// Yardımcı fonksiyon: "prefix_value" şeklinde Symbol üretir
fn create_storage_key(env: &Env, prefix: &str, value: &str) -> Symbol {
    let full = [prefix, value].join("_");
    Symbol::new(env, &full)
}
