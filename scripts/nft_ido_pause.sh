#!/bin/bash
source neardev/dev-account.env
source neardev/dev-contracts.env

OWNER_ID="muzikant.testnet"
IDO_ID="test"
PAUSE="false"

near call $CONTRACT_NAME nft_ido_pause --accountId $NFT_CONTRACT_ID "{ \"ido_id\": \"$IDO_ID\", \"contract_id\": \"$NFT_CONTRACT_ID\", \"pause\": $PAUSE }"
