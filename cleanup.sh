#!/bin/bash

JSON_FILE=$(find ~/.near-credentials/testnet/ -regextype posix-extended -regex ".*/[A-Za-z0-9_,\\s]+\.testnet\.json")
# shellcheck disable=SC2002
ACCOUNT_ID=$(cat "$JSON_FILE" | jq -r ".account_id")

near delete near_project_management."$ACCOUNT_ID" "$ACCOUNT_ID"