#!/bin/bash

# Test account (Alice's address)
ACCOUNT="5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"

# Test username
USERNAME="alice_username"

echo "Setting username for $ACCOUNT to $USERNAME..."
curl -H "Content-Type: application/json" -d '{
  "id": 1,
  "jsonrpc": "2.0",
  "method": "usernameStorage_set_username",
  "params": ["'$ACCOUNT'", "'$USERNAME'"]
}' http://localhost:9933

echo -e "\n\nWaiting for block finalization..."
sleep 12

echo -e "\nGetting username from Node B..."
curl -H "Content-Type: application/json" -d '{
  "id": 1,
  "jsonrpc": "2.0",
  "method": "usernameStorage_get_username",
  "params": ["'$ACCOUNT'"]
}' http://localhost:9934 