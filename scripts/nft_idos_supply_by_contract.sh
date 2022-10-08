#!/bin/bash
source neardev/dev-account.env
source neardev/dev-contracts.env

near view $CONTRACT_NAME nft_idos_supply_by_contract "{ \"contract_id\": \"$NFT_CONTRACT_ID\" }"
