use futures::{SinkExt, StreamExt};
use radius_sdk::json_rpc::server::LocalRpcParameter;
use tokio::net::TcpListener;
use tokio_tungstenite::{accept_async, connect_async, tungstenite::Message};

use crate::{rpc::eth::EthSendRawTransaction, state::AppState};

pub async fn run_websocket_server(context: AppState) {
    tokio::spawn(async move {
        let external_ws_url = format!(
            "0.0.0.0:{}",
            context
                .config()
                .external_ws_port()
                .expect("Fail to get external ws port")
        );
        let rollup_ws_url = context.config().rollup_ws_url().to_string();
        let listener = TcpListener::bind(&external_ws_url).await.unwrap();

        tracing::info!("Successfully start websocket listener: {}", external_ws_url);

        const METHOD_SEND_RAW_TX: &str = "eth_sendRawTransaction";

        // Wait for user connection
        while let Ok((tcp_stream, socket_addr)) = listener.accept().await {
            let cloned_context = context.clone();
            let cloned_rollup_ws_url = rollup_ws_url.clone();

            tokio::spawn(async move {
                // Interaction with user
                let ws_stream = accept_async(tcp_stream)
                    .await
                    .expect("WebSocket handshake failure");
                let (mut user_ws_write, mut user_ws_read) = ws_stream.split();

                // Interaction with rollup
                let (rollup_ws_stream, _) = connect_async(cloned_rollup_ws_url).await.unwrap();
                let (mut rollup_ws_write, mut rollup_ws_read) = rollup_ws_stream.split();

                // User → Rollup
                let user_to_rollup = async {
                    while let Some(Ok(msg)) = user_ws_read.next().await {
                        if !msg.is_text() {
                            continue;
                        }

                        let text = match msg.to_text() {
                            Ok(t) => t,
                            Err(e) => {
                                tracing::warn!("Fail to convert msg: {}", e);
                                continue;
                            }
                        };

                        let json: serde_json::Value = match serde_json::from_str(text) {
                            Ok(j) => j,
                            Err(e) => {
                                eprintln!("Invalid JSON message: {} | {}", text, e);
                                continue;
                            }
                        };

                        let method = json.get("method").and_then(|m| m.as_str());

                        if method == Some(METHOD_SEND_RAW_TX) {
                            tracing::info!("Filtered eth_sendRawTransaction → {}", text);

                            let params = match json.get("params") {
                                Some(p) => p,
                                None => {
                                    tracing::warn!("No params: {}", text);
                                    continue;
                                }
                            };

                            let params_vec: Vec<String> =
                                match serde_json::from_value(params.clone()) {
                                    Ok(v) => v,
                                    Err(e) => {
                                        tracing::warn!(
                                            "Fail to convert Vec<String>: {} | {}",
                                            params,
                                            e
                                        );
                                        continue;
                                    }
                                };

                            let params = EthSendRawTransaction(params_vec);
                            match params.handler(cloned_context.clone()).await {
                                Ok(result) => {
                                    if let Err(e) = rollup_ws_write
                                        .send(Message::from(result.to_string()))
                                        .await
                                    {
                                        tracing::error!("Fail to send: {}", e);
                                    }
                                }
                                Err(e) => {
                                    tracing::error!("Rpc error: {}", e);
                                }
                            }
                        } else {
                            if let Err(e) = rollup_ws_write.send(msg).await {
                                tracing::error!("Rpc error: {}", e);
                            }
                        }
                    }
                };

                // Rollup → User
                let rollup_to_user = async {
                    while let Some(Ok(msg)) = rollup_ws_read.next().await {
                        if user_ws_write.send(msg).await.is_err() {
                            tracing::info!("Disconnected: {}", socket_addr.ip());
                        }
                    }
                };

                tokio::select! {
                    _ = user_to_rollup => {
                        tracing::info!("user_to_rollup end, disconnected");
                    },
                    _ = rollup_to_user => {
                        tracing::info!("rollup_to_user end, disconnected");
                    },
                }
            });
        }
    });
}
