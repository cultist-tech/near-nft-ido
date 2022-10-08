use near_sdk::{ AccountId, PromiseOrValue, env, near_bindgen };
use crate::*;
use near_sdk::json_types::U128;
use mfight_sdk::ft::FungibleTokenReceiver;
use mfight_sdk::nft_ido::NftIdoOnFtTransferArgs;
use near_sdk::serde::{ Deserialize, Serialize };
use schemars::JsonSchema;

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(crate = "near_sdk::serde")]
enum Args {
    NftIdo(NftIdoOnFtTransferArgs),
}

#[near_bindgen]
impl FungibleTokenReceiver for Contract {
    fn ft_on_transfer(
        &mut self,
        sender_id: AccountId,
        amount: U128,
        msg: String
    ) -> PromiseOrValue<U128> {
        let ft_token_id = env::predecessor_account_id();
        let signer_id = env::signer_account_id();

        assert_ne!(
      ft_token_id,
      signer_id,
      "nft_on_transfer should only be called via cross-contract call"
    );

        match near_sdk::serde_json::from_str(&msg).expect("Invalid Args") {
            Args::NftIdo(ftIdoArgs) => {
                self.nft_ido.internal_on_ft_transfer(&ftIdoArgs, &ft_token_id, &amount, &sender_id)
            }
        }
    }
}