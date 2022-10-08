#!/bin/bash
source neardev/dev-account.env
source neardev/dev-contracts.env

near call $CONTRACT_NAME whitelist_add --accountId $CONTRACT_NAME "{ \"account_id\": \"$NFT_CONTRACT_ID\" }"
