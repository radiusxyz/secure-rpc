#!/bin/bash
SCRIPT_PATH="$( cd -- "$(dirname "$0")" >/dev/null 2>&1 ; pwd -P )"
source $SCRIPT_PATH/env.sh

DATA_PATH=$CURRENT_PATH/secure-rpc

rm -rf $DATA_PATH

$SECURE_RPC_BIN_PATH init --path $DATA_PATH

config_file_path=$DATA_PATH/config.toml
    
sed -i.temp "s/secure_rpc_url = \"http:\/\/127.0.0.1:9000\"/secure_rpc_url = \"http:\/\/$HOST:9000\"/g" $config_file_path

sed -i.temp "s/sequencer_rpc_url = \"http:\/\/127.0.0.1:3000\"/sequencer_rpc_url = \"http:\/\/$HOST:3000\"/g" $config_file_path

sed -i.temp "s/rollup_rpc_url = \"http:\/\/127.0.0.1:8123\"/rollup_rpc_url = \"http:\/\/$ROLLUP_HOST:8123\"/g" $config_file_path

sed -i.temp "s/key_management_system_rpc_url = \"http:\/\/127.0.0.1:7100\"/key_management_system_rpc_url = \"http:\/\/$HOST:7100\"/g" $config_file_path

sed -i.temp "s/rollup_id = \"0\"/rollup_id = \"rollup_id\"/g" $config_file_path

rm $config_file_path.temp