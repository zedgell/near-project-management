#!/bin/bash

JSON_FILE=$(find ~/.near-credentials/testnet/ -regextype posix-extended -regex ".*/[A-Za-z0-9_,\\s]+\.testnet\.json")
# shellcheck disable=SC2002
ACCOUNT_ID=$(cat "$JSON_FILE" | jq -r ".account_id")

near create-account near_project_management."$ACCOUNT_ID" --masterAccount "$ACCOUNT_ID"

near deploy near_project_management."$ACCOUNT_ID" ./target/wasm32-unknown-unknown/release/near_project_management.wasm

near call near_project_management."$ACCOUNT_ID" new --accountId near_project_management."$ACCOUNT_ID"
