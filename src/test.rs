#![cfg(test)]
//import Contract and client
use super::{Contract, ContractClient};
//import soroban SDK
use soroban_sdk::{symbol, vec, Env};

#[test]
fn test() {
    //env calls the ENV=Environment that soroban contrat runs on
    let env = Env::default();
    //contract id is the used to register the contract in the enviroment.
    let contract_id = env.register_contract(None, Contract);
    //create a new instance of the contract from the id in the soroban environment.
    let client = ContractClient::new(&env, &contract_id);
    //pass an arguement to our to paramwters in the hello function
    let words = client.hello(&symbol!("Dev"));
    //check that the function returns the right value/string for the first value in our vec,
    // it should be "Hello" or whatever you change it to
    //for it to pass
    assert_eq!(words, vec![&env, symbol!("Hello"), symbol!("Dev"),]);
}
