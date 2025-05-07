# Substrate Username Storage Chain

This is a Substrate-based blockchain that demonstrates storing and retrieving usernames via custom JSON-RPC methods in a 2-node environment.

## Features

- Store username mappings (account_id → username) on-chain
- Custom JSON-RPC methods for storing and retrieving usernames
- Two-node environment support
- Signed transaction requirements for updates

## Project Structure

The project consists of several key components:

1. **Pallet Username Storage** 
   - Location [pallets/username-storage/](./pallets/username-storage) 
   - Implements the core storage functionality for usernames
   - Provides dispatchable calls for setting and getting usernames
   - Manages storage items and events

2. **Username Storage RPC** 
   - Location [rpc/username-storage/](./rpc/username-storage-rpc) 
   - Implements custom JSON-RPC methods
   - Provides `username_getUsername` and `username_setUsername` endpoints
   - Handles RPC requests and responses

3. **Runtime Integration**
   - The pallet is integrated into the runtime [runtime/src/lib.rs](runtime/src/lib.rs)
   - RPC extensions are configured in the node [node/src/rpc.rs](node/src/rpc.rs)

4. **Testing Scripts**
   - JavaScript-based test suite using Polkadot.js API
   - Located in `scripts/test_rpc.js`
   - Tests both RPC endpoints and chain functionality

## Prerequisites

- Rust and Cargo
- Substrate development environment
- Polkadot.js Apps (optional, for UI interaction)
- Node.js and NPM (for running test scripts)

## Building

```bash
# Clone the repository
git clone https://github.com/jproyo/solochain-kv-store.git
cd solochain-template

# Build the project
cargo build --release

## Running the Nodes

```bash
sh scripts/run_nodes.sh
```

## Using the JSON-RPC API

The chain exposes two custom RPC methods:

1. `username_getUsername(accountId)`: Retrieves the username for a given account ID
2. `username_setUsername(accountId, username)`: Sets a username for a given account ID

You can test these endpoints using the provided test script:

```bash
sh scripts/run_test.sh
```

## Testing

The project includes multiple testing approaches:

1. **Unit Tests**
```bash
# Run the test suite
cargo test
```

## Design Considerations

1. **Storage**: Usernames are stored in a bounded vector to prevent excessive storage usage.
2. **Security**: Only signed transactions can update usernames.
3. **RPC Interface**: Custom RPC methods are implemented for both setting and getting usernames.
4. **Two-Node Setup**: The system is designed to work in a multi-node environment, demonstrating data synchronization.
5. **Error Handling**: Comprehensive error handling for invalid inputs and edge cases.
6. **Testing**: Multiple testing approaches ensure reliability and correctness.

