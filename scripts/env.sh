#!/bin/bash
CURRENT_PATH="$( cd -- "$(dirname "$0")" >/dev/null 2>&1 ; pwd -P )"

SECURE_RPC_BIN_PATH=$CURRENT_PATH/../target/release/secure-rpc

HOST="192.168.12.14"
ROLLUP_HOST="192.168.12.68"