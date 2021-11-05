#!/bin/bash

# Simple script to download all symbols from https://api.binance.com/api/v3/exchangeInfo
# jq is required
# prints it to stdout

echo $(curl -s -H 'Content-Type: application/json'  https://api.binance.com/api/v3/exchangeInfo | jq -r '.symbols | sort_by(.symbol) | .[] | .symbol')
