# Secure RPC Provider

:warning: Under Construction
> This crate is actively being developed. Breaking changes will occur until mainnet when we will start [Semantic Versioning](https://semver.org/).

Secure RPC Provider is a key component of the [Radius Block Building Solution](https://github.com/radiusxyz/radius-docs-bbs/blob/main/docs/radius_block_building_solution.md) written in Rust. It serves as a secure intermediary layer between wallet interfaces and transaction orderers, providing encryption capabilities for user transactions using SKDE (Shared Key Distributed Encryption).

## Architecture

The Secure RPC Provider implements a worker-based architecture with three operational modes:

### Operational Modes

1. **Blockchain Operator Mode**: Integrates with blockchain services for trusted setup and distributed key generation
2. **Basic Operator Mode**: Uses a simple DKG RPC URL for key generation
3. **Secure RPC Only Mode**: Provides only secure RPC functionality (implementation pending)

### Core Components

- **External RPC Worker**: Handles communication with external services (rollup, tx_orderer)
- **Operator Worker**: Manages key generation and blockchain interactions
- **Secure RPC Worker**: Processes encrypted transactions and RPC requests
- **CLI Interface**: Command-line tool for node management and configuration

## Installation

### Prerequisites

- Rust (latest stable version)
- Cargo package manager

### Building from Source

```bash
git clone https://github.com/radiusxyz/secure-rpc-provider
cd secure-rpc-provider
cargo build --release
```

The binary will be available at `target/release/secure-rpc-provider`.

## Usage

### Command Line Interface

The secure RPC provider supports various command-line options and configuration file management:

```bash
# Start with configuration file
secure-rpc-provider --config config.toml node blockchain

# Start in blockchain operator mode with CLI arguments
secure-rpc-provider node \
  --node-name "my-node" \
  --rpc-url "http://127.0.0.1:8545" \
  --rollup-id "0" \
  --tx-orderer-rpc-url "http://127.0.0.1:3000" \
  --rollup-rpc-url "http://127.0.0.1:8123" \
  --encrypt-mode \
  blockchain \
  --blockchain-http-rpc-url "http://127.0.0.1:8545" \
  --blockchain-ws-rpc-url "ws://127.0.0.1:8546" \
  --contract-address "0x..."

# Start in basic operator mode
secure-rpc-provider node \
  --node-name "my-node" \
  --rpc-url "http://127.0.0.1:8545" \
  basic \
  --dkg-rpc-url "http://127.0.0.1:7100"

# Mix configuration file with CLI overrides
secure-rpc-provider --config config.toml node blockchain \
  --rpc-url "http://custom:8080"
```

### Configuration Options

#### Configuration File Support

The secure RPC provider supports TOML configuration files for easier management of settings. CLI arguments take precedence over configuration file values.

**Global Options:**

| Option | Description | Default |
|--------|-------------|---------|
| `--config` | Path to TOML configuration file | Auto-detect `config.toml` |

**Example Configuration File (`config.toml`):**

```toml
[node]
is_dev = false
node_name = "production-node"

[secure_rpc]
rollup_id = "1"
rpc_url = "http://127.0.0.1:8545"
encrypt_mode = true
tx_orderer_rpc_url = "http://127.0.0.1:3000"
rollup_rpc_url = "http://127.0.0.1:8123"

[operator.blockchain]
blockchain_http_rpc_url = "http://127.0.0.1:8545"
blockchain_ws_rpc_url = "ws://127.0.0.1:8546"
contract_address = "0x1234567890abcdef..."

[operator.basic]
dkg_rpc_urls = ["http://dkg1:8080", "http://dkg2:8080"]
```

#### Global Node Options

| Option | Description | Default |
|--------|-------------|---------|
| `--node-name` | Name identifier for the node | - |
| `--rpc-url` | RPC server listening address | `http://127.0.0.1:8545` |
| `--rollup-id` | Rollup identifier | `0` |
| `--tx-orderer-rpc-url` | Transaction orderer RPC URL | Default list |
| `--rollup-rpc-url` | L2 rollup node URL | `http://127.0.0.1:8123` |
| `--encrypt-mode` | Enable transaction encryption | `true` |
| `--is-dev` | Enable development mode | `false` |

#### Blockchain Operator Options

| Option | Description | Default |
|--------|-------------|---------|
| `--blockchain-http-rpc-url` | Blockchain HTTP RPC URL | Default URL |
| `--blockchain-ws-rpc-url` | Blockchain WebSocket RPC URL | Default URL |
| `--contract-address` | Trusted setup contract address | Default address |

#### Basic Operator Options

| Option | Description | Default |
|--------|-------------|---------|
| `--dkg-rpc-url` | Distributed Key Generation RPC URL | Default URL |

### Using Scripts

The project includes convenience scripts for setup and execution:

```bash
# Initialize secure RPC configuration
./scripts/execute/01_init_secure_rpc.sh

# Run secure RPC server
./scripts/execute/02_run_secure_rpc.sh
```

Make sure to configure the environment variables in the `env.sh` file before running the scripts.

## RPC Methods

The secure RPC provider exposes Ethereum-compatible RPC methods with encryption support:

- `eth_sendRawTransaction`: Send raw transactions (with optional encryption)
- `send_encrypted_transaction`: Send pre-encrypted transactions
- Standard Ethereum JSON-RPC methods (proxied to rollup)

## Development

### Project Structure

```
secure-rpc-provider/
├── cli/                    # Command-line interface
│   └── src/
│       ├── config/        # Configuration file management
│       │   ├── types.rs   # TOML structure definitions
│       │   ├── loader.rs  # Configuration loading logic
│       │   └── mod.rs     # Module exports
│       └── commands/      # CLI command implementations
├── node/
│   ├── primitive/         # Core node primitives and configuration
│   └── service/          # Node service implementation
├── rpc/                   # RPC method implementations
├── primitives/           # Shared primitives and traits
├── scripts/              # Deployment and execution scripts
└── src/                  # Main binary entry point
```

### Dependencies

Key external dependencies:
- **alloy**: Ethereum library for Rust
- **tokio**: Async runtime
- **jsonrpsee**: JSON-RPC implementation
- **clap**: Command-line parsing
- **serde**: Serialization framework
- **toml**: TOML configuration file parsing
- **tracing**: Structured logging

Radius-specific dependencies:
- **skde**: Shared Key Distributed Encryption
- **tx_orderer**: Transaction ordering service
- **radius-sdk**: Radius SDK for Rust

### Running Tests

```bash
cargo test
```

### Development Mode

For development, you can run the node with the `--is-dev` flag to enable additional logging and development features.

## Contributing

We appreciate your contributions to our project. Visit [issues](https://github.com/radiusxyz/secure-rpc-provider/issues) page to start with or refer to the [Contributing guide](https://github.com/radiusxyz/radius-docs-bbs/blob/main/docs/contributing_guide.md).

## Getting Help

Our developers are willing to answer your questions. If you are first and bewildered, refer to the [Getting Help](https://github.com/radiusxyz/radius-docs-bbs/blob/main/docs/getting_help.md) page.

## License

This project is licensed under the MIT License.
