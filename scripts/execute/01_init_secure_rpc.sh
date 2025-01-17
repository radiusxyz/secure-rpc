#!/bin/bash
SCRIPT_PATH="$( cd -- "$(dirname "$0")" >/dev/null 2>&1 ; pwd -P )"
source $SCRIPT_PATH/env.sh

rm -rf $DATA_PATH

echo "Initialize secure rpc" 

$BIN_PATH init --path $DATA_PATH

sed -i.temp "s|rollup_id = \"0\"|rollup_id = \"$ROLLUP_ID\"|g" $CONFIG_FILE_PATH
sed -i.temp "s|rollup_rpc_url = \"http://127.0.0.1:8123\"|rollup_rpc_url = \"$ROLLUP_RPC_URL\"|g" $CONFIG_FILE_PATH

sed -i.temp "s|external_rpc_url = \"http://127.0.0.1:9000\"|external_rpc_url = \"$SECURE_RPC_EXTERNAL_RPC_URL\"|g" $CONFIG_FILE_PATH

sed -i.temp "s|sequencer_rpc_url = \"[http://127.0.0.1:3000]\"|sequencer_rpc_url = \"$SEQUENCER_RPC_URL\"|g" $CONFIG_FILE_PATH

sed -i.temp "s|encrypted_transaction_type = \"skde\"|encrypted_transaction_type = \"$ENCRYPTED_TRANSACTION_TYPE\"|g" $CONFIG_FILE_PATH
sed -i.temp "s|distributed_key_generation_rpc_url = \"http://127.0.0.1:7100\"|distributed_key_generation_rpc_url = \"$DISTRIBUTED_KEY_GENERATOR_RPC_URL\"|g" $CONFIG_FILE_PATH

rm $CONFIG_FILE_PATH.temp