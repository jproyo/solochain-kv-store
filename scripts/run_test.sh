#!/bin/bash

# Check if package.json exists, if not initialize it
if [ ! -f "package.json" ]; then
    echo "Initializing npm project..."
    npm init -y
fi

# Install dependencies if node_modules doesn't exist
if [ ! -d "node_modules" ]; then
    echo "Installing dependencies..."
    npm install @polkadot/api @polkadot/util-crypto
fi

# Run the test script
echo "Running test script..."
node scripts/test_rpc.js 