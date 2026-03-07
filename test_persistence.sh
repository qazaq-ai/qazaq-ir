#!/bin/bash
set -e

echo "Starting Node (Run 1)..."
cargo run --bin orda-node > node1.log 2>&1 &
NODE1_PID=$!

echo "Waiting for Port 3000 to open..."
while ! nc -z 127.0.0.1 3000; do
  sleep 0.5
done
echo "Node 1 is LIVE!"

echo "Feeding Transaction to Mempool..."
curl -s -X POST -H 'Content-Type: application/json' -d @temp_intent.json http://127.0.0.1:3000/intent > /dev/null
sleep 1

echo "Balance before crash (should be 100):"
curl -s http://127.0.0.1:3000/balance/1 | jq .

echo "Killing Node 1..."
kill $NODE1_PID
sleep 1

echo "Restarting Node (Run 2)..."
cargo run --bin orda-node > node2.log 2>&1 &
NODE2_PID=$!

echo "Waiting for Port 3000 to open..."
while ! nc -z 127.0.0.1 3000; do
  sleep 0.5
done
echo "Node 2 is LIVE!"

echo "Balance after restart (should SURVIVE and still be 100):"
curl -s http://127.0.0.1:3000/balance/1 | jq .

echo "Killing Node 2..."
kill $NODE2_PID
echo "Sled Persistence Verification Complete."
