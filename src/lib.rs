use near_sdk::borsh::{ self, BorshDeserialize, BorshSerialize };
use near_sdk::json_types::{ U128 };
use near_sdk::{ env, near_bindgen, AccountId, PanicOnDefault, BorshStorageKey };
use near_sdk::collections::{ LookupMap, TreeMap, UnorderedSet };
use std::collections::HashMap;
use mfight_sdk::pause::PauseFeature;
use mfight_sdk::owner::OwnerFeature;
use mfight_sdk::blacklist::BlacklistFeature;
use mfight_sdk::whitelist::WhitelistFeature;
use mfight_sdk::nft_ido::{ NftIdoFeature, TokenId, IdoId, Ido };

mod nft_callbacks;
mod ft_callbacks;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    pause: PauseFeature,
    owner: OwnerFeature,
    blacklist: BlacklistFeature,
    whitelist: WhitelistFeature,
    nft_ido: NftIdoFeature,
}

/// Helper structure to for keys of the persistent collections.
#[derive(BorshStorageKey, BorshSerialize)]
pub enum StorageKey {
    BlacklistAccounts,
    WhitelistAccounts,

    IdoByToken,
    IdoTokens,
    IdoRandomTokens,
    IdoMintCounter,
    IdosAvailable,
    IdoByFt,
    IdoTokensByContract,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new_with_default_meta(owner_id: AccountId) -> Self {
        Self::new(owner_id)
    }

    #[init]
    pub fn new(owner_id: AccountId) -> Self {
        let this = Self {
            pause: PauseFeature::new(),
            owner: OwnerFeature::new(owner_id.clone()),
            blacklist: BlacklistFeature::new(StorageKey::BlacklistAccounts),
            whitelist: WhitelistFeature::new(StorageKey::WhitelistAccounts),
            nft_ido: NftIdoFeature::new(
                StorageKey::IdoByToken,
                StorageKey::IdoTokens,
                StorageKey::IdoRandomTokens,
                StorageKey::IdoMintCounter,
                StorageKey::IdosAvailable,
                StorageKey::IdoByFt,
                StorageKey::IdoTokensByContract
            ),
        };

        this
    }

    pub fn assert_owner(&self) {
        self.owner.assert_owner();
    }

    pub fn assert_access(&self) {
        // self.whitelist.assert_whitelist(&env::predecessor_account_id());
    }

    #[init(ignore_state)]
    #[private]
    pub fn migrate() -> Self {
        #[derive(BorshDeserialize, BorshSerialize)]
        pub struct OldNftIdo {
            pub nft_contract_id: AccountId,
            pub ido_by_token: LookupMap<TokenId, IdoId>,
            pub ido_tokens: LookupMap<IdoId, UnorderedSet<TokenId>>,
            pub idos_available: UnorderedSet<IdoId>,
            pub ido_by_id: HashMap<IdoId, Ido>,
            pub ido_date_by_id: HashMap<IdoId, u64>,
            pub ido_random_tokens: LookupMap<IdoId, Vec<TokenId>>,
            pub ido_mint_counter: LookupMap<IdoId, LookupMap<AccountId, u64>>,
            pub ido_by_ft_token: LookupMap<IdoId, AccountId>,
            pub ido_tokens_by_contract: TreeMap<AccountId, UnorderedSet<TokenId>>,
        }

        #[derive(BorshDeserialize)]
        struct Old {
            pause: PauseFeature,
            owner: OwnerFeature,
            blacklist: BlacklistFeature,
            nft_ido: OldNftIdo,
            whitelist: WhitelistFeature,
        }

        let old: Old = env::state_read().expect("Error");
        let nft_ido = NftIdoFeature {
            ido_by_token: old.nft_ido.ido_by_token,
            ido_tokens: old.nft_ido.ido_tokens,
            idos_available: old.nft_ido.idos_available,
            ido_by_id: old.nft_ido.ido_by_id,
            ido_date_by_id: old.nft_ido.ido_date_by_id,
            ido_random_tokens: old.nft_ido.ido_random_tokens,
            ido_mint_counter: old.nft_ido.ido_mint_counter,
            ido_by_ft_token: old.nft_ido.ido_by_ft_token,
            ido_tokens_by_contract: old.nft_ido.ido_tokens_by_contract,
        };

        Self {
            owner: old.owner,
            pause: old.pause,
            blacklist: old.blacklist,
            nft_ido,
            whitelist: old.whitelist,
        }
    }
}

mfight_sdk::impl_nft_ido_core!(Contract, nft_ido, assert_access);
mfight_sdk::impl_nft_ido_enumeration!(Contract, nft_ido);

mfight_sdk::impl_pause_feature!(Contract, pause, assert_owner);
mfight_sdk::impl_owner_feature!(Contract, owner);
mfight_sdk::impl_blacklist_feature!(Contract, blacklist, assert_owner);
mfight_sdk::impl_whitelist_feature!(Contract, whitelist, assert_owner);