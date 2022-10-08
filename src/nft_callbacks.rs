use near_sdk::{ env, near_bindgen, AccountId, PromiseOrValue };
use near_sdk::serde_json::from_str;
use crate::*;
use mfight_sdk::nft::base::NonFungibleTokenReceiver;
use mfight_sdk::nft_ido::{ TokenId, NftIdoOnNftTransferArgs };
use near_sdk::serde::{ Deserialize, Serialize };
use schemars::JsonSchema;

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(crate = "near_sdk::serde")]
enum Args {
    NftIdo(NftIdoOnNftTransferArgs),
}

/// approval callbacks from NFT Contracts

#[near_bindgen]
impl NonFungibleTokenReceiver for Contract {
    /// where we add the ido because we know nft owner can only call nft_approve

    fn nft_on_transfer(
        &mut self,
        sender_id: AccountId,
        previous_owner_id: AccountId,
        token_id: TokenId,
        msg: String
    ) -> PromiseOrValue<bool> {
        let nft_contract_id = env::predecessor_account_id();
        let signer_id = env::signer_account_id();

        assert_eq!(
          nft_contract_id,
          signer_id,
          "nft_on_transfer should only be called via cross-contract call"
      );
        assert_eq!(
          &sender_id,
          &signer_id,
          "owner_id should be signer_id"
      );

        match near_sdk::serde_json::from_str(&msg).expect("Invalid Args") {
            Args::NftIdo(nftIdoArgs) => {
                self.nft_ido.internal_on_nft_transfer(
                    &nftIdoArgs,
                    &nft_contract_id,
                    &token_id,
                    &sender_id
                )
            }
        }
    }
}
