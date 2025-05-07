#!/bin/bash

# Test account (Alice's address)
ACCOUNT="5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"

echo -e "\n\nTest username"
USERNAME="alice_username"

echo "Setting username for $ACCOUNT to $USERNAME... in Node A"
curl -H "Content-Type: application/json" -d "{
  \"id\": 1,
  \"jsonrpc\": \"2.0\",
  \"method\": \"usernameStorage_setUsername\",
  \"params\": [\"$ACCOUNT\", \"$USERNAME\"]
}" http://localhost:9933

echo "\n\nWaiting for block finalization..."
sleep 20 

echo "\nGetting username from Node B..."
curl -H "Content-Type: application/json" -d "{
  \"id\": 1,
  \"jsonrpc\": \"2.0\",
  \"method\": \"usernameStorage_getUsername\",
  \"params\": [\"$ACCOUNT\"]
}" http://localhost:9934 