#!/bin/bash
SCRIPT_PATH="$( cd -- "$(dirname "$0")" >/dev/null 2>&1 ; pwd -P )"
PROJECT_ROOT_PATH="$( cd $SCRIPT_PATH/../.. >/dev/null 2>&1 ; pwd -P )"
SECURE_RPC_BIN_PATH="$PROJECT_ROOT_PATH/scripts/secure-rpc"

if [[ ! -f "$SECURE_RPC_BIN_PATH" ]]; then
    echo "Error: Secure RPC binary not found at $SECURE_RPC_BIN_PATH"
    echo "Please run this command 'cp $PROJECT_ROOT_PATH/target/release/secure-rpc $PROJECT_ROOT_PATH/scripts'"
    exit 1
fi

DATA_PATH=$PROJECT_ROOT_PATH/data
CONFIG_FILE_PATH=$DATA_PATH/Config.toml
PRIVATE_KEY_PATH=$DATA_PATH/signing_key

# Secure RPC
SECURE_RPC_EXTERNAL_RPC_URL="http://127.0.0.1:5000"

# Rollup
ROLLUP_ID="rollup_id"
ROLLUP_RPC_URL="http://127.0.0.1:8123"

# Sequencer
SEQUENCER_RPC_RPC_URL="http://127.0.0.1:6001"

# Encrypted Transaction Type
ENCRYPTED_TRANSACTION_TYPE="skde"

# DKG (for ENCRYPTED_TRANSACTION_TYPE=skde)
DISTRIBUTED_KEY_GENERATOR_RPC_URL="http://131.153.159.15:7100"





