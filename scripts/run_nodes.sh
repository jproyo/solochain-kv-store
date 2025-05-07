#!/bin/bash

# Build the node
echo "Building the node..."
cargo build --release

# Kill any existing nodes
echo "Cleaning up existing nodes..."
pkill -f solochain-template-node || true

# Create base paths and clean them
echo "Setting up node directories..."
rm -rf /tmp/node01 /tmp/node02
mkdir -p /tmp/node01
mkdir -p /tmp/node02

# Generate node keys
echo "Generating node keys..."
NODE_A_KEY=$(./target/release/solochain-template-node key generate-node-key --file /tmp/node01/node-key)
NODE_B_KEY=$(./target/release/solochain-template-node key generate-node-key --file /tmp/node02/node-key)

# Start Node A (Validator)
echo "Starting Node A (Validator)..."
./target/release/solochain-template-node \
  --base-path /tmp/node01 \
  --chain local \
  --alice \
  --port 30333 \
  --rpc-port 9944 \
  --validator \
  --rpc-cors all \
  --rpc-external \
  --rpc-methods=unsafe \
  --unsafe-rpc-external \
  --name "Node A" \
  --node-key-file /tmp/node01/node-key \
  --telemetry-url "wss://telemetry.polkadot.io/submit/ 0" \
  --prometheus-external \
  --prometheus-port 9615 \
  --no-mdns \
  --no-private-ipv4 \
  --no-telemetry &

# Wait for Node A to start
echo "Waiting for Node A to start..."
sleep 5

# Start Node B (Full Node)
echo "Starting Node B (Full Node)..."
./target/release/solochain-template-node \
  --base-path /tmp/node02 \
  --chain local \
  --bob \
  --port 30334 \
  --rpc-port 9945 \
  --bootnodes "/ip4/127.0.0.1/tcp/30333/p2p/$(./target/release/solochain-template-node key inspect-node-key --file /tmp/node01/node-key)" \
  --rpc-cors all \
  --rpc-external \
  --rpc-methods=unsafe \
  --unsafe-rpc-external \
  --name "Node B" \
  --node-key-file /tmp/node02/node-key \
  --telemetry-url "wss://telemetry.polkadot.io/submit/ 0" \
  --prometheus-external \
  --prometheus-port 9616 \
  --no-mdns \
  --no-private-ipv4 \
  --no-telemetry &

echo "Nodes started successfully!"
echo "Node A:"
echo "  WebSocket: ws://localhost:9944"
echo "  RPC: http://localhost:9944"
echo "  Prometheus: http://localhost:9615"
echo "Node B:"
echo "  WebSocket: ws://localhost:9945"
echo "  RPC: http://localhost:9945"
echo "  Prometheus: http://localhost:9616"

# Keep the script running
wait 