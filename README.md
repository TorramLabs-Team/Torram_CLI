# Torram Network - Bitcoin Testnet 3 Documentation
## Smart Contracts with Bitcoin-Native TSB Tokens

Build DApps that combine CosmWasm smart contracts with Bitcoin-secured tokens through event-driven automation.

---

## 🚀 What Torram Provides

**Torram Network** gives you:
- **CosmWasm Smart Contracts** - Deploy Rust-based contracts 
- **TSB Token Standard** - Bitcoin-native tokens secured by Taproot scripts
- **Built-in TSB Commands** - Create and transfer tokens directly through Torram CLI
- **Built-in Oracle** - Access 30+ crypto and RWA prices through our in built oracle. For example: Bitcoin, S&P500, etc
- **Built-in UTXO Indexing** - Access UTXOs for a Bitcoin address directly with CosmWasm smart contracts
- **Automation though Contract Events** - Emit contract events that can trigger CLI operations 

---

## 📊 Oracle Price Data

**Torram's built-in oracle provides real-time price feeds for cryptocurrency and real-world assets (RWAs).**

### **Supported Cryptocurrency Prices**

Access live prices for these 19 cryptocurrencies:

| Crypto | Full Name        |
|--------|------------------|
| BTC    | Bitcoin          |
| ETH    | Ethereum         |
| USDT   | Tether           |
| USDC   | USD Coin         |
| SOL    | Solana           |
| DOGE   | Dogecoin         |
| BNB    | Binance Chain    |
| NEAR   | Near Protocol    |
| DAI    | Dai              |
| UNI    | Uniswap          |
| TON    | Toncoin          |
| ADA    | Cardano          |
| OM     | Mantra           |
| ONDO   | Ondo Finance     |
| TRX    | Tron             |
| AVAX   | Avalanche        |
| AAVE   | Aave             |
| STX    | Stacks           |
| XLM    | Stellar          |

### **Supported Real-World Asset (RWA) Prices**

Access traditional financial market data for these 12 assets:

| RWAs   | Full Name                             |
|--------|---------------------------------------|
| DJI    | Dow Jones Industrial Average          |
| BAI    | Blackrock AI Innovation and Tech      |
| FTBFX  | Fidelity Total Bond Fund              |
| FVX    | 5-Year Treasury Yield                 |
| NDAQ   | Nasdaq Composite                      |
| QQQ    | Invesco QQQ                           |
| RUT    | Russell 2000 Index                    |
| SPX    | S&P 500 Index                         |
| TNX    | 10-Year Treasury Yield                |
| TYX    | 30-Year Treasury Yield                |
| VGT    | Vanguard Information Technology       |
| VIX    | Volatility Index                      |

---

## 🛠️ Development Workflow

### **Step 1: Create Your TSB Token**

First, create a Bitcoin-secured token using Torram's built-in TSB functionality:

```bash
# Create your token on Bitcoin via Torram
torramd tx tsb create-token \
  --token-id="MYTOKEN" \
  --amount=1000000 \
  --type-code=0 \
  --metadata='{"name":"MYTOKEN","symbol":"MYTOKEN","decimals":6,"description":"My DApp Token"}' \
  --from=your-key \
  --keyring-backend=test \
  --chain-id=torram \
  --gas=auto \
  --gas-adjustment=1.3 \
  --fees=100torram \
  --node tcp://34.57.91.248:26657
```

**What this does:**
- Creates a Bitcoin TSB token using Taproot scripts
- Embeds token data in Bitcoin transaction witness
- Stores token reference in Torram network
- Returns Bitcoin transaction ID for your records

**Check your token:**
```bash
# Verify token creation
torramd query tsb token-balance --token-id MYTOKEN --owner $(torramd keys show your-key -a) --node tcp://34.57.91.248:26657

# Check token balances
torramd query tsb token-balance --token-id MYTOKEN --owner your-torram-address --node tcp://34.57.91.248:26657
```

### **Step 2: Create Your Smart Contract**

Deploy your CosmWasm contract that will emit events for token operations:

```bash
# Store your contract code
torramd tx wasm store contract.wasm \
  --from your-key \
  --keyring-backend test \
  --chain-id torram \
  --gas auto \
  --gas-adjustment 1.3 \
  --fees 1000torram \
  --node tcp://34.57.91.248:26657 \
  --broadcast-mode sync

# Instantiate your contract
torramd tx wasm instantiate 1 \
  '{"token_name":"MYTOKEN","initial_config":"your-config"}' \
  --from your-key \
  --keyring-backend test \
  --chain-id torram \
  --label "My DApp Contract" \
  --no-admin \
  --gas auto \
  --gas-adjustment 1.3 \
  --fees 1000torram \
  --node tcp://34.57.91.248:26657 \
  --broadcast-mode sync
```

**Your contract should emit events when TSB transfers are needed:**

Event format your contract needs to emit:
```
Event Type: "tsb_transfer_needed"
Required Attributes:
- transfer_id: "unique_id"
- token_name: "MYTOKEN"
- to_address: "bitcoin_address"
- amount: "transfer_amount"
- reason: "why_transferring"
- contract: "your_contract_address"
```

### **Step 3: Build Your Automation Server**

Create a server that listens to your contract events and executes TSB transfers:

**Your server needs to:**

1. **Listen to contract events** via WebSocket connection
2. **Parse contract events** to extract transfer details
3. **Execute TSB transfers** using Torram CLI commands
4. **Report results back** to your contract

**WebSocket Connection:**
```
Endpoint: wss://34.57.91.248:26657/websocket
Subscribe to: wasm.contract_address='your_contract' AND wasm.action='tsb_transfer_needed'
```

**CLI Command Your Server Executes:**
```bash
# When your server sees a transfer event, it runs:
torramd tx tsb transfer-token MYTOKEN recipient-address amount \
  --from server-key \
  --chain-id torram \
  --node tcp://34.57.91.248:26657 \
  --gas auto \
  --gas-adjustment 1.3 \
  --keyring-backend test \
  --gas-prices 0.1torram
```

**Transfer Result:**
Your CLI command returns Bitcoin transaction IDs that your server reports back to the contract:
- Funding transaction hash
- Recipient reveal transaction hash  
- Change transaction hash (if applicable)

### **Step 4: Complete the Loop**

Your server reports Bitcoin transaction completion back to your contract:

```bash
# Your server calls your contract to confirm completion
torramd tx wasm execute $CONTRACT_ADDR \
  '{"confirm_transfer":{"transfer_id":"123","bitcoin_funding_tx":"abc...","bitcoin_recipient_tx":"def...","bitcoin_change_tx":"ghi..."}}' \
  --from server-key \
  --keyring-backend test \
  --chain-id torram \
  --gas auto \
  --node tcp://34.57.91.248:26657
```

---

## 📋 Available TSB Commands

### **Token Management**
```bash
# Create new token
torramd tx tsb create-token \
  --token-id="TOKEN" \
  --amount=1000000 \
  --type-code=0 \
  --metadata='{"name":"TOKEN","symbol":"TOKEN","decimals":6,"description":"Token description"}' \
  --from=your-key \
  --keyring-backend=test \
  --chain-id=torram \
  --gas=auto \
  --gas-adjustment=1.3 \
  --fees=100torram \
  --node tcp://34.57.91.248:26657

# Transfer tokens
torramd tx tsb transfer-token TOKEN recipient-address amount \
  --from your-key \
  --chain-id torram \
  --node tcp://34.57.91.248:26657 \
  --gas auto \
  --gas-adjustment 1.3 \
  --keyring-backend test \
  --gas-prices 0.1torram
```

### **Token Information Queries**
```bash
# Get specific token details
torramd query tsb token-balance --token-id TOKEN --owner owner-address --node tcp://34.57.91.248:26657

# Check specific token balance for an address
torramd query tsb token-balance --token-id TOKEN --owner owner-address --node tcp://34.57.91.248:26657

# Get token operations for a specific token
torramd query wasm contract-state smart CONTRACT_ADDRESS '{"get_token_operations":{"token_id":"TOKEN"}}' --node tcp://34.57.91.248:26657
```

### **Smart Contract Operations**
```bash
# Deploy contract
torramd tx wasm store contract.wasm \
  --from key \
  --keyring-backend test \
  --chain-id torram \
  --gas 3000000 \
  --fees 1000torram \
  --node tcp://34.57.91.248:26657 \
  --broadcast-mode sync

torramd tx wasm instantiate CODE_ID '{}' \
  --from key \
  --keyring-backend test \
  --chain-id torram \
  --label "Contract Label" \
  --no-admin \
  --gas auto \
  --gas-adjustment 1.3 \
  --fees 1000torram \
  --node tcp://34.57.91.248:26657 \
  --broadcast-mode sync

# Execute contract
torramd tx wasm execute CONTRACT_ADDR 'EXECUTE_MSG' \
  --from key \
  --keyring-backend test \
  --chain-id torram \
  --gas auto \
  --node tcp://34.57.91.248:26657

# Query contract
torramd query wasm contract-state smart CONTRACT_ADDR 'QUERY_MSG' --node tcp://34.57.91.248:26657

# Check contract info
torramd query wasm contract CONTRACT_ADDR --node tcp://34.57.91.248:26657
```

---

## 🌐 Network Information

### **Connection Details**
- **Chain ID**: `torram`
- **RPC Endpoint**: `tcp://34.57.91.248:26657`
- **WebSocket**: `wss://34.57.91.248:26657/websocket`
- **Explorer**: `https://explorer-testnet3.torram.network`
- **Faucet**: Request testnet tokens in telgram @torrambuilders

---

## 📡 Event Monitoring

### **WebSocket Event Subscription**

Your server subscribes to contract events:
```json
{
  "jsonrpc": "2.0",
  "method": "subscribe", 
  "id": 1,
  "params": {
    "query": "wasm.contract_address='torram1your-contract-address' AND wasm.action='tsb_transfer_needed'"
  }
}
```

### **Event Data Structure**

When your contract emits a transfer event, your server receives:
```json
{
  "result": {
    "events": [
      {
        "type": "wasm",
        "attributes": [
          {"key": "action", "value": "tsb_transfer_needed"},
          {"key": "transfer_id", "value": "123"},
          {"key": "token_name", "value": "MYTOKEN"},
          {"key": "to_address", "value": "tb1p..."},
          {"key": "amount", "value": "1000000"},
          {"key": "reason", "value": "amm_trade"},
          {"key": "contract", "value": "torram1..."}
        ]
      }
    ]
  }
}
```

### **Transaction Monitoring**
```bash
# Check transaction status
torramd query tx TRANSACTION_HASH --node tcp://34.57.91.248:26657

# Monitor specific contract
torramd query txs --events "wasm.contract_address=torram1..." --node tcp://34.57.91.248:26657

# View recent TSB operations
torramd query tsb recent-transfers --node tcp://34.57.91.248:26657
```

---

## 🔄 Example DApp Flows

### **Automated Market Maker (AMM)**
1. **User calls contract** - Execute swap function
2. **Contract calculates trade** - AMM formula determines output
3. **Contract emits event** - `tsb_transfer_needed` with swap details
4. **Automated server sees event** - Extracts recipient and amount
5. **Server executes transfer** - Calls `torramd tx tsb transfer-token`
6. **Server confirms completion** - Reports Bitcoin tx IDs to contract


### **Liquidity Pool Management**
1. **Pool rebalancing needed** - Contract algorithm detects imbalance
2. **Rebalancing transfers** - Multiple transfer events emitted
3. **Batch execution** - Automated server processes all transfers
4. **Pool state update** - Contract updated with new balances

---

## 🧪 Testing Your Integration

### **Test Flow Checklist**
- [ ] Create TSB token successfully
- [ ] Deploy contract that emits events
- [ ] Verify WebSocket connection to Torram
- [ ] Test event parsing in your automated server
- [ ] Execute test TSB transfer via CLI
- [ ] Confirm contract receives Bitcoin tx IDs
- [ ] Monitor transaction completion

---

## 📞 Support & Resources

**Getting Help:**
- **Telegram**: @torrambuilders
- **Explorer**: [Torram Explorer](https://www.torramexplorer.xyz/)

---