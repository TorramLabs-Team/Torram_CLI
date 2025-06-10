# TSB Reader Contract

CosmWasm contract for reading TSB (Token Sync Bridge) data.

## Query Examples

Get all tokens:
```rust
let msg = QueryMsg::GetAllTokens {};
let response: GetAllTokensResponse = deps.querier.query(&msg)?;
```

Get specific token:
```rust
let msg = QueryMsg::GetToken { 
    token_id: "token_123".to_string() 
};
let response: GetTokenResponse = deps.querier.query(&msg)?;
```

Get user balances:
```rust
let msg = QueryMsg::GetBalancesByOwner { 
    owner: "torram1...".to_string() 
};
let response: GetBalancesByOwnerResponse = deps.querier.query(&msg)?;
```

Get Bitcoin UTXOs:
```rust
let msg = QueryMsg::GetUtxos { 
    address: Some("bc1...".to_string()) 
};
let response: GetUTXOsResponse = deps.querier.query(&msg)?;
```

## Testing

Unit test:
```rust
#[test]
fn test_query() {
    let deps = mock_dependencies();
    let msg = QueryMsg::GetAllTokens {};
    let res = query(deps.as_ref(), mock_env(), msg).unwrap();
}
```

## Build and Deploy

Build:
```bash
cargo build
cargo wasm
```

Deploy:
```bash
torramd tx wasm store contract.wasm --from mykey --gas auto
torramd tx wasm instantiate $CODE_ID '{}' --from mykey --label "tsb-reader"
```

Query deployed contract:
```bash
torramd query wasm contract-state smart $CONTRACT '{"get_all_tokens":{}}'
torramd query wasm contract-state smart $CONTRACT '{"get_token":{"token_id":"123"}}'
```

## Schema

Generate schema:
```bash
cargo schema
```

Use in TypeScript:
```typescript
const query = {
    get_token: { token_id: "123" }
};
``` 