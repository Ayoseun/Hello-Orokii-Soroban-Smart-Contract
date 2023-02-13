// This is to ensure that the contract does not compile with standard Rust libraries
#![no_std]
pub mod test;



use soroban_sdk::{contractimpl, symbol, vec, Env, Symbol, Vec};

//declare the contract as public
pub struct Contract;
//Begin contract implementation
#[contractimpl]

impl Contract {

    /**
     * This is a public function that returns Hello
     *It returns a Vec that accepts only Symbol.
     *Vec is a sequential and indexable growable collection type.
     * Symbol type can contain a-z or A-Z or 0-9 but is limted to only 10 characters.
     */
    pub fn hello(env: Env, to: Symbol) -> Vec<Symbol> {
        // Our Vec returns Hello and any other symbol type
        vec![&env, symbol!("Hello"), to]

       
    }
}
