#!/bin/bash

cargo build --release

# Kill any existing nodes
pkill -f solochain-template-node

# Create base paths
mkdir -p /tmp/node01
mkdir -p /tmp/node02

# Start Node A (Validator)
./target/release/solochain-template-node \
  --base-path /tmp/node01 \
  --chain local \
  --alice \
  --port 30333 \
  --rpc-port 9933 \
  --validator \
  --rpc-cors all \
  --rpc-external \
  --name "Node A" &

# Wait for Node A to start
sleep 5

# Get Node A's peer ID
NODE_A_PEER_ID=$(./target/release/solochain-template-node key inspect-node-key --file /tmp/node01/p2p/secret_ed25519 | grep "Peer ID" | awk '{print $3}')

# Start Node B (Full Node)
./target/release/solochain-template-node \
  --base-path /tmp/node02 \
  --chain local \
  --bob \
  --port 30334 \
  --rpc-port 9934 \
  --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/$NODE_A_PEER_ID \
  --rpc-cors all \
  --rpc-external \
  --name "Node B" &

echo "Nodes started!"
echo "Node A: ws://localhost:9944, rpc: http://localhost:9933"
echo "Node B: ws://localhost:9945, rpc: http://localhost:9934" 