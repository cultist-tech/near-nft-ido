#!/bin/bash
source neardev/dev-account.env
source neardev/dev-contracts.env

IDO_ID="reveal"
PRICE="2000000000000000000000000"
AMOUNT="50"
PER_TX_MIN="1"
PER_TX_MAX="1"
BUY_MAX="8"
NAME="Reveal test sale"
MEDIA="\"https://mfight.io/static/image/defaults/mint.png\""
FT_TOKEN_ID="\"mfight-ft.testnet\""
# FT_TOKEN_ID="null"

near call $CONTRACT_NAME nft_ido_add --accountId $NFT_CONTRACT_ID "{ \"ido_id\": \"$IDO_ID\", \"contract_id\": \"$NFT_CONTRACT_ID\", \"name\": \"$NAME\", \"amount\": $AMOUNT, \"price\": \"$PRICE\", \"per_transaction_min\": $PER_TX_MIN, \"per_transaction_max\": $PER_TX_MAX, \"buy_max\": $BUY_MAX, \"media\": $MEDIA, \"ft_token\": $FT_TOKEN_ID }" --amount 0.1
