#!/bin/bash
source neardev/dev-account.env
source neardev/dev-contracts.env

IDO_ID="reveal"
DATE="1663513608940000000"

near call $CONTRACT_NAME nft_ido_start --accountId $NFT_CONTRACT_ID "{ \"ido_id\": \"$IDO_ID\", \"contract_id\": \"$NFT_CONTRACT_ID\", \"date\": $DATE }"
