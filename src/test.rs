#![cfg(test)]
use super::*;
use soroban_sdk::Env;

#[test]
fn test_add() {
      let env = Env::default();
      let contract_id = env.register_contract(None, ExampleContract);
      let client = ExampleContractClient::new(&env, &contract_id);

    let result = client.add(&10, &20);
      assert_eq!(result, 30);
}
