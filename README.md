# Stablecoin Oracle Smart Contract

This CosmWasm smart contract implements a stablecoin oracle price registry where only the admin can update on-chain price data. Prices are supplied explicitly via execution, and the contract emits USDC, USDT, and DAI values along with their average. It also includes a query mechanism to fetch price data from an external custom oracle using a Stargate-compatible query interface.

## 🔧 Features

- **Admin-controlled price updates**
- **Price emission as blockchain events**
- **Query integration with custom Cosmos SDK oracle module**
- **Merged single-file architecture for simplicity**

---

## 📁 File Structure

> All logic is contained within `src/lib.rs` for simplicity and ease of portability.

---

## ⚙️ Instantiate

```json
{
  "admin": "torram1..."
}
```

The instantiation message sets the admin who is allowed to call `update_prices`.

---

## 🛠 Execute Messages

### `update_prices`
Admin-only. Updates the stored price data and emits them as an event.

```json
{
  "update_prices": {
    "btc": "10929.05",
    "eth": "2627.48",
    "usdc": "0.9998",
    "usdt": "1.0001",
    "dai": "1.0000"
  }
}
```

📤 Emits an event:
```json
{
  "oracle_prices": {
    "usdc": "0.9998",
    "usdt": "1.0001",
    "dai": "1.0000",
    "average": "1.0000"
  }
}
```

---

## 🔍 Query Messages

### `get_prices`
Returns the latest stored prices:
```json
{
  "get_prices": {}
}
```
Response:
```json
{
  "btc": "10929.05",
  "eth": "2627.48",
  "usdc": "0.9998",
  "usdt": "1.0001",
  "dai": "1.0000"
}
```

### `fetch_from_oracle`
Makes a Stargate custom query to the `tssconsensus` module's `get_all_contacts` method to fetch and parse prices.

```json
{
  "fetch_from_oracle": {}
}
```

Response:
```json
{
  "btc": "10929.05",
  "eth": "2627.48",
  "usdc": "0.9999",
  "usdt": "1.0000",
  "dai": "1.0001"
}
```

---

## 🧪 Testing
You can test this contract using `torramd`, or any other CosmWasm-compatible blockchain.

For example, using `torramd`:
```bash
# 1. Store the contract on-chain
$ torramd tx wasm store price.wasm \
  --from torram-node \
  --keyring-backend test \
  --chain-id torram \
  --node tcp://localhost:26657 \
  --gas auto \
  --gas-adjustment 1.3 \
  --fees 1000torram \
  -y \
  --output json

# 2. List all stored codes to get your code_id
$ torramd query wasm list-code \
  --chain-id torram \
  --node tcp://localhost:26657 \
  --output json

# 3. Instantiate the contract
$ torramd tx wasm instantiate 23 '{"admin":"<your-address>"}' \
  --from torram-node \
  --admin <your-address> \
  --label "oracle_price_contract" \
  --keyring-backend test \
  --chain-id torram \
  --node tcp://localhost:26657 \
  --gas auto \
  --gas-adjustment 1.3 \
  --fees 1000torram \
  -y \
  --output json

# 4. Query contract address by code_id
$ torramd query wasm list-contract-by-code 23 \
  --chain-id torram \
  --node tcp://localhost:26657 \
  --output json

# 5. Query oracle prices via contract
$ torramd query wasm contract-state smart <contract_addr> '{"fetch_from_oracle":{}}' \
  --chain-id torram \
  --node tcp://localhost:26657 \
  --output json
```

---

## 🔐 Admin Permissions
Only the admin is allowed to:
- Update prices
- Trigger emission of updated price data

Other queries are permissionless.

---

## 📦 Build
To build the optimized contract Wasm binary:
```bash
cargo wasm
```

---

## 📚 Dependencies
- `cosmwasm-std`
- `cw-storage-plus`
- `serde`, `schemars`
- `cosmwasm-schema`
- `thiserror`
