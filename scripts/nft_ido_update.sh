#!/bin/bash
source neardev/dev-account.env
source neardev/dev-contracts.env

IDO_ID="test"
PER_TX_MIN=1
PER_TX_MAX=1
BUY_MAX=20
DATE="1643280497042"

near call $CONTRACT_NAME nft_ido_update --accountId $$NFT_CONTRACT_ID "{ \"ido_id\": \"$IDO_ID\", \"contract_id\": \"$NFT_CONTRACT_ID\", \"date\": $DATE, \"per_transaction_min\": $PER_TX_MIN, \"per_transaction_max\": $PER_TX_MAX, \"buy_max\": $BUY_MAX }"
