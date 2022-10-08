#!/bin/bash
source neardev/dev-account.env
source neardev/dev-contracts.env

IDO_ID="test"

near view $CONTRACT_NAME nft_ido_not_minted "{ \"ido_id\": \"$IDO_ID\", \"contract_id\": \"$NFT_CONTRACT_ID\" }"
