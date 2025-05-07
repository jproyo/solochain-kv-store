# Substrate Username Storage Chain

This is a Substrate-based blockchain that demonstrates storing and retrieving usernames via custom JSON-RPC methods in a 2-node environment.

## Features

- Store username mappings (account_id → username) on-chain
- Custom JSON-RPC methods for storing and retrieving usernames
- Two-node environment support
- Signed transaction requirements for updates

## Prerequisites

- Rust and Cargo
- Substrate development environment
- Polkadot.js Apps (optional, for UI interaction)

## Building

```bash
# Clone the repository
git clone <repository-url>
cd solochain-template

# Build the project
cargo build --release
```

## Running the Nodes

```bash
sh scripts/run_nodes.sh
```

## Using the JSON-RPC API

You need to have NPM installed because the test script is done with JavaScript in order to use Polkadot Lib

```bash
sh scripts/run_tests.sh
```

## Testing

```bash
# Run the test suite
cargo test
```

## Design Considerations

1. **Storage**: Usernames are stored in a bounded vector to prevent excessive storage usage.
2. **Security**: Only signed transactions can update usernames.
3. **RPC Interface**: Custom RPC methods are implemented for both setting and getting usernames.
4. **Two-Node Setup**: The system is designed to work in a multi-node environment, demonstrating data synchronization.

