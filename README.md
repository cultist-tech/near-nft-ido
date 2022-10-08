# NFT IDO

Реализовать функционал для упрощенной первичной распродажи нфт (наподобие распродаж NFT на Binance). Базовый функционал: случайная NFT из списка, ограничения на покупку, отложенный по времени минт, минт за FT или NEAR. Проблема: малое количество NFT платформ предлагает подобный функционал, из-за чего многие разработчики сами создают подобный функционал.

```rust
pub trait IdoCore {
  // создать новый сэйл в рамках одного нфт контракта
  fn nft_ido_add(
    &mut self,
    // нфт контракт адрес
    contract_id: AccountId,
    // id сэйла
    ido_id: IdoId,
    // название сэйла
    name: String,
    // количество нфт в сэйле
    amount: u64,
    // цена за 1 нфт
    price: U128,
    // минимальное количество нфт в рамках одной транзакции
    per_transaction_min: u64,
    // максимальные количество нфт в рамках одной транзакции
    per_transaction_max: u64,
    // максимальное количество нфт которое можно заминтить одному пользователю
    buy_max: u64,
    // ft токен для оплаты, если не указан то оплата в near
    ft_token: Option<AccountId>
  ) -> JsonIdo;

  // запуск сэйла (указывается дата запуска)
  fn nft_ido_start(&mut self, contract_id: AccountId, ido_id: IdoId, date: u64) -> JsonIdo;

  // обновление параметров сэйла
  fn nft_ido_update(&mut self, contract_id: AccountId, ido_id: IdoId, date: u64, per_transaction_min: u64, per_transaction_max: u64, buy_max: u64) -> JsonIdo;

  // поставить сэйл на паузу
  fn nft_ido_pause(&mut self, contract_id: AccountId, ido_id: IdoId, pause: bool) -> JsonIdo;

  // покупка нфт из сэйла (за near или ft)
  fn nft_buy(&mut self, contract_id: AccountId, receiver_id: AccountId, ido_id: IdoId, amount: u64);
}

pub trait IdoEnumeration {
  // просмотр списка сэйлов
  fn nft_idos(&self) -> Vec<JsonIdo>;

  // просмотр информации о сэйле
  fn nft_ido(&self, contract_id: AccountId, ido_id: IdoId) -> JsonIdo;

  // количество незаминченных нфт
  fn nft_ido_not_minted(&self, contract_id: AccountId, ido_id: IdoId) -> u64;

  // просмотр списка нфт в рамках сэйла
  fn nft_ido_tokens(
    &self,
    contract_id: AccountId,
    ido_id: IdoId,
    from_index: Option<U128>,
    limit: Option<u64>,
  ) -> Vec<TokenId>;

  // сколько заминтил нфт пользователь в рамках сэйла
  fn nft_ido_account_minted(&self, contract_id: AccountId, ido_id: IdoId, account_id: AccountId) -> u64;

  // список сэйлов в рамках нфт контракта
  fn nft_idos_by_contract(&self, contract_id: AccountId, from_index: Option<U128>, limit: Option<u64>,) -> Vec<JsonIdo>;

  // кол-во сэйлов в рамках нфт контракта
  fn nft_idos_supply_by_contract(&self, contract_id: AccountId) -> U128;
}
```
