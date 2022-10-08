#!/bin/bash
source neardev/dev-account.env
source neardev/dev-contracts.env

near view $CONTRACT_NAME nft_idos_by_contract "{ \"contract_id\": \"$NFT_CONTRACT_ID\", \"from_index\": \"0\", \"limit\": 10 }"
