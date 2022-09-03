use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::U128;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, AccountId, near_bindgen, Balance, PanicOnDefault, BorshStorageKey, Promise, PromiseOrValue, ext_contract, PromiseResult, Gas};
use near_sdk::collections::{LookupMap};
use uit-ecommerce-contract::*;
use near_units::parse_near;
use serde_json::json;
use workspaces::prelude::*;
use workspaces::{network::Sandbox, sandbox, Account, Contract, Worker};
const WASM_FILEPATH: &str = "../out/main.wasm";


[tokio::main]
async fn main() -> anyhow::Resutl<()> {
    let sandbox = sandbox().await?;
    let wasm = std::fs::read(WASM_FILEPATH)?;
    let contract = sandbox.dev_deploy(&wasm).await?;

    //create accounts
    let owner = sandbox.root_account().unwrap();
    let user = owner.create_subaccount(&sandbox, "user")
                    . initial_balance(parse_near!("30 N"))
                    .transact()
                    .await?;
                    .into_resutl()?;

    //begin tests
    test_fn_contracts()

    Ok(())
}


#[test]
async fn test_ft_contracts(
    user: &Account,
    contract: &Contract,
    sandbox: &Worker<Sandbox>
) -> anyhow::Result<()> {
    let mut context = get_context(false);
        let alice: AccountId = accounts(0);

        context.account_balance(1000)
        .predecessor_account_id(alice.clone())
        .attached_deposit(1000)
        .signer_account_id(alice.clone());

        testing_env!(context.build());

        let mut contract = EcommerceContract::new(alice.clone());
        let order_amount = U128(1000);
        contract.ft_on_transfer(alice, order_amount);

        let order = contract.get_order("order_1".to_owned());

        // Test
        assert_eq!(order.order_id, "order_1".to_owned());
        assert_eq!(order.amount, order_amount.0);
        assert_eq!(order.payer_id, alice);
        assert!(order.is_completed);
    }