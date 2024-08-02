#!/bin/bash
SCRIPT_PATH="$( cd -- "$(dirname "$0")" >/dev/null 2>&1 ; pwd -P )"
source $SCRIPT_PATH/env.sh

DATA_PATH=$CURRENT_PATH/secure-rpc

rm -rf $DATA_PATH

$SECURE_RPC_BIN_PATH init --path $DATA_PATH