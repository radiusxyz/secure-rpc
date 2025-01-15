#!/bin/bash
SCRIPT_PATH="$( cd -- "$(dirname "$0")" >/dev/null 2>&1 ; pwd -P )"
PROJECT_ROOT_PATH="$( cd $SCRIPT_PATH/../.. >/dev/null 2>&1 ; pwd -P )"

BIN_FILE_NAME="secure-rpc"
BIN_PATH="$PROJECT_ROOT_PATH/scripts/$BIN_FILE_NAME"

DATA_PATH=$PROJECT_ROOT_PATH/data
CONFIG_FILE_PATH=$DATA_PATH/Config.toml

# Copy the new version's binary to the scripts directory
if [[ -f "$PROJECT_ROOT_PATH/target/release/$BIN_FILE_NAME" ]]; then
  cp $PROJECT_ROOT_PATH/target/release/$BIN_FILE_NAME $PROJECT_ROOT_PATH/scripts
fi

# Check if the binary exists
if [[ ! -f "$BIN_PATH" ]]; then
    echo "Error: Secure RPC binary not found at $BIN_PATH"
    echo "Please run this command 'cp $PROJECT_ROOT_PATH/target/release/$BIN_FILE_NAME $PROJECT_ROOT_PATH/scripts' after building the project"
    exit 1
fi

# Secure RPC
SECURE_RPC_EXTERNAL_RPC_URL="http://127.0.0.1:5000" # External IP - Please change this IP.

# Rollup
ROLLUP_ID="rollup_id"                  # Please change this rollup id.
ROLLUP_RPC_URL="http://127.0.0.1:8123" # Please change this rollup rpc url.

# Sequencer
SEQUENCER_RPC_URL="http://127.0.0.1:6000" # Please change this sequencer (external) rpc url.

# Encrypted Transaction Type - skde / pvde
ENCRYPTED_TRANSACTION_TYPE="skde"

# DKG (for ENCRYPTED_TRANSACTION_TYPE=skde)
DISTRIBUTED_KEY_GENERATOR_RPC_URL="http://127.0.0.1:7100" # Please change this distribured key generator rpc url.