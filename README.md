# Secure RPC

:warning: Under Construction
> This crate is actively being developed. Breaking changes will occur until mainnet when we will start [Semantic Versioning](https://semver.org/).

Secure RPC server of [Radius Block Building Solution]() written in Rust programming language.

Secure RPC is a layer between wallet interface and sequencer that can receive unencrypted user transactions, encrypt them and forward them to sequencers belonging to a particular cluster. Except for encryption functionality, it works just like a proxy between users and sequencers. Secure RPC exists solely because the wallet interface does not support Radius encryption methods (PVDE and SKDE) and will be removed if the wallet supports a plugin or add-on features (e.g. MetaMask Snaps) and allows third-party modules.

## Contributing
We appreciate your contributions to our project. To get involved, refer to the [Contributing guide](https://github.com/radiusxyz/radius-docs-bbs/blob/main/contributing_guide.md).

## Getting Help
Our developers are willing to answer your questions. If you are first and bewildered, refer to the [Getting Help](https://github.com/radiusxyz/radius-docs-bbs/blob/main/getting_help.md) page.
