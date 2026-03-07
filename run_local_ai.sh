#!/bin/bash

# Exit on error
set -e

# ANSI Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

if [ -z "$1" ]; then
    echo -e "${RED}Error: No prompt provided.${NC}"
    echo -e "Usage: ./run_local_ai.sh \"Your natural language prompt here\""
    exit 1
fi

PROMPT="$1"

echo -e "\n${BLUE}=== Qazaq AI Bridge ===${NC}"
echo -e "${YELLOW}⚙️  Compiling Ollama Model Profile (qazaq-ai)...${NC}"
ollama create qazaq-ai -f Modelfile

echo -e "\n${YELLOW}🧠 LLM Translation in progress...${NC}"
echo -e "Prompt: \"$PROMPT\""

# Call the local Ollama instance
RESPONSE=$(curl -s -X POST http://localhost:11434/api/generate -d "{
  \"model\": \"qazaq-ai\",
  \"prompt\": \"$PROMPT\",
  \"format\": \"json\",
  \"stream\": false
}")

# Extract the response field using jq if available, otherwise fallback to grep/sed
if command -v jq &> /dev/null; then
    JSON_OUTPUT=$(echo "$RESPONSE" | jq -r '.response')
else
    # Fallback basic extraction if jq isn't installed
    JSON_OUTPUT=$(echo "$RESPONSE" | grep -o '"response":"[^"]*"' | sed 's/"response":"\(.*\)"/\1/' | sed 's/\\n/\n/g' | sed 's/\\"/"/g')
fi

# Ensure output is clean JSON (strip potential markdown wrapping just in case Phi-3 hallucinates)
CLEAN_JSON=$(echo "$JSON_OUTPUT" | sed 's/^```json//g' | sed 's/^```//g' | sed 's/```$//g')

echo -e "\n${GREEN}» JSON Payload Extracted:${NC}"
echo "$CLEAN_JSON"
echo "$CLEAN_JSON" > temp_intent.json

echo -e "\n${YELLOW}🌐 Sending Intent to Orda Node API Gateway...${NC}"
# Send the parsed JSON to the running Orda Node
curl -s -X POST -H 'Content-Type: application/json' -d @temp_intent.json http://127.0.0.1:3000/intent
echo ""

echo -e "\n${YELLOW}⏳ Waiting for background Execution Engine...${NC}"
sleep 1

echo -e "\n${GREEN}💰 Fetching New Balance from State Machine...${NC}"
curl -s http://127.0.0.1:3000/balance/1 | jq .
echo ""

echo -e "\n${GREEN}=== Operation Complete ===${NC}"
