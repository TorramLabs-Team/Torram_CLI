use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
    CustomQuery, StdError, QueryRequest, Empty,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// TSB Query types that match the Go bindings
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TSBQuery {
    GetAllTokens {},
    GetToken { token_id: String },
    GetTokensByCreator { creator: String },
    GetAllBalances {},
    GetTokenBalance { token_id: String, owner: String },
    GetBalancesByOwner { owner: String },
    GetTokenOperations { token_id: String },
    GetTokenOperation { operation_id: String },
    GetPendingBitcoinSync {},
    GetTokensForSync {},
    GetUtxos { address: Option<String> },
}

impl CustomQuery for TSBQuery {}

// TSB Response types
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TSBToken {
    pub token_id: String,
    pub amount: String,
    pub type_code: u32,
    pub metadata: String,
    pub creator: String,
    pub creation_time: String,
    pub bitcoin_tx_id: String,
    pub synced_with_bitcoin: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TSBBalance {
    pub token_id: String,
    pub owner: String,
    pub amount: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TSBOperation {
    pub operation_id: String,
    pub token_id: String,
    pub r#type: u32,
    pub from: String,
    pub to: String,
    pub amount: String,
    pub timestamp: String,
    pub bitcoin_tx_id: String,
    pub torram_tx_id: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TSBCTXO {
    pub tx_id: String,
    pub vout: u32,
    pub amount: String,
    pub used: bool,
}

// Response wrappers
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct GetAllTokensResponse {
    pub tokens: Vec<TSBToken>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct GetTokenResponse {
    pub token: Option<TSBToken>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct GetTokensByCreatorResponse {
    pub tokens: Vec<TSBToken>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct GetAllBalancesResponse {
    pub balances: Vec<TSBBalance>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct GetTokenBalanceResponse {
    pub balance: Option<TSBBalance>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct GetBalancesByOwnerResponse {
    pub balances: Vec<TSBBalance>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct GetTokenOperationsResponse {
    pub operations: Vec<TSBOperation>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct GetTokenOperationResponse {
    pub operation: Option<TSBOperation>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct GetPendingBitcoinSyncResponse {
    pub operations: Vec<TSBOperation>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct GetTokensForSyncResponse {
    pub token_ids: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct GetUTXOsResponse {
    pub utxos: Vec<TSBCTXO>,
}

// Contract messages
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    // This contract is read-only, no execute messages
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    // TSB read functions exposed to CosmWasm
    GetAllTokens {},
    GetToken { token_id: String },
    GetTokensByCreator { creator: String },
    GetAllBalances {},
    GetTokenBalance { token_id: String, owner: String },
    GetBalancesByOwner { owner: String },
    GetTokenOperations { token_id: String },
    GetTokenOperation { operation_id: String },
    GetPendingBitcoinSync {},
    GetTokensForSync {},
    GetUtxos { address: Option<String> },
    
    // Aggregated queries for convenience
    GetTokenSummary { token_id: String },
    GetUserPortfolio { owner: String },
    GetSyncStatus {},
}

// Aggregated response types
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TokenSummary {
    pub token: Option<TSBToken>,
    pub total_supply: String,
    pub holder_count: u32,
    pub operations_count: u32,
    pub pending_sync: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct UserPortfolio {
    pub owner: String,
    pub balances: Vec<TSBBalance>,
    pub total_tokens: u32,
    pub total_value: String, // Could be calculated if price oracle exists
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct SyncStatus {
    pub total_tokens: u32,
    pub synced_tokens: u32,
    pub pending_operations: u32,
    pub tokens_for_sync: Vec<String>,
}

#[entry_point]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    Ok(Response::new().add_attribute("method", "instantiate"))
}

#[entry_point]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: ExecuteMsg,
) -> StdResult<Response> {
    // This contract is read-only
    Err(StdError::generic_err("This contract is read-only"))
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        // Direct TSB queries
        QueryMsg::GetAllTokens {} => query_all_tokens(deps),
        QueryMsg::GetToken { token_id } => query_token(deps, token_id),
        QueryMsg::GetTokensByCreator { creator } => query_tokens_by_creator(deps, creator),
        QueryMsg::GetAllBalances {} => query_all_balances(deps),
        QueryMsg::GetTokenBalance { token_id, owner } => query_token_balance(deps, token_id, owner),
        QueryMsg::GetBalancesByOwner { owner } => query_balances_by_owner(deps, owner),
        QueryMsg::GetTokenOperations { token_id } => query_token_operations(deps, token_id),
        QueryMsg::GetTokenOperation { operation_id } => query_token_operation(deps, operation_id),
        QueryMsg::GetPendingBitcoinSync {} => query_pending_bitcoin_sync(deps),
        QueryMsg::GetTokensForSync {} => query_tokens_for_sync(deps),
        QueryMsg::GetUtxos { address } => query_utxos(deps, address),
        
        // Aggregated queries
        QueryMsg::GetTokenSummary { token_id } => query_token_summary(deps, token_id),
        QueryMsg::GetUserPortfolio { owner } => query_user_portfolio(deps, owner),
        QueryMsg::GetSyncStatus {} => query_sync_status(deps),
    }
}

// Helper function to make TSB queries
fn make_tsb_query<T: CustomQuery>(deps: Deps, query: T) -> StdResult<Binary> {
    let request: QueryRequest<Empty> = QueryRequest::Custom(query);
    deps.querier.query(&request)
}

// Direct TSB query implementations
fn query_all_tokens(deps: Deps) -> StdResult<Binary> {
    make_tsb_query(deps, TSBQuery::GetAllTokens {})
}

fn query_token(deps: Deps, token_id: String) -> StdResult<Binary> {
    make_tsb_query(deps, TSBQuery::GetToken { token_id })
}

fn query_tokens_by_creator(deps: Deps, creator: String) -> StdResult<Binary> {
    make_tsb_query(deps, TSBQuery::GetTokensByCreator { creator })
}

fn query_all_balances(deps: Deps) -> StdResult<Binary> {
    make_tsb_query(deps, TSBQuery::GetAllBalances {})
}

fn query_token_balance(deps: Deps, token_id: String, owner: String) -> StdResult<Binary> {
    make_tsb_query(deps, TSBQuery::GetTokenBalance { token_id, owner })
}

fn query_balances_by_owner(deps: Deps, owner: String) -> StdResult<Binary> {
    make_tsb_query(deps, TSBQuery::GetBalancesByOwner { owner })
}

fn query_token_operations(deps: Deps, token_id: String) -> StdResult<Binary> {
    make_tsb_query(deps, TSBQuery::GetTokenOperations { token_id })
}

fn query_token_operation(deps: Deps, operation_id: String) -> StdResult<Binary> {
    make_tsb_query(deps, TSBQuery::GetTokenOperation { operation_id })
}

fn query_pending_bitcoin_sync(deps: Deps) -> StdResult<Binary> {
    make_tsb_query(deps, TSBQuery::GetPendingBitcoinSync {})
}

fn query_tokens_for_sync(deps: Deps) -> StdResult<Binary> {
    make_tsb_query(deps, TSBQuery::GetTokensForSync {})
}

fn query_utxos(deps: Deps, address: Option<String>) -> StdResult<Binary> {
    make_tsb_query(deps, TSBQuery::GetUtxos { address })
}

// Aggregated query implementations
fn query_token_summary(deps: Deps, token_id: String) -> StdResult<Binary> {
    // Get token info
    let token_response: GetTokenResponse = deps.querier.query(&QueryRequest::Custom(TSBQuery::GetToken { token_id: token_id.clone() }))?;
    
    // Get operations count
    let operations_response: GetTokenOperationsResponse = deps.querier.query(&QueryRequest::Custom(TSBQuery::GetTokenOperations { token_id: token_id.clone() }))?;
    
    // Get all balances to calculate holder count
    let balances_response: GetAllBalancesResponse = deps.querier.query(&QueryRequest::Custom(TSBQuery::GetAllBalances {}))?;
    let holder_count = balances_response.balances.iter()
        .filter(|b| b.token_id == token_id && b.amount != "0")
        .count() as u32;
    
    // Check if pending sync
    let pending_response: GetPendingBitcoinSyncResponse = deps.querier.query(&QueryRequest::Custom(TSBQuery::GetPendingBitcoinSync {}))?;
    let pending_sync = pending_response.operations.iter()
        .any(|op| op.token_id == token_id);
    
    let total_supply = token_response.token.as_ref().map(|t| t.amount.clone()).unwrap_or_default();
    let token_clone = token_response.token.clone();
    
    let summary = TokenSummary {
        token: token_clone,
        total_supply,
        holder_count,
        operations_count: operations_response.operations.len() as u32,
        pending_sync,
    };
    
    to_binary(&summary)
}

fn query_user_portfolio(deps: Deps, owner: String) -> StdResult<Binary> {
    let balances_response: GetBalancesByOwnerResponse = deps.querier.query(&QueryRequest::Custom(TSBQuery::GetBalancesByOwner { owner: owner.clone() }))?;
    
    let portfolio = UserPortfolio {
        owner,
        total_tokens: balances_response.balances.len() as u32,
        balances: balances_response.balances,
        total_value: "0".to_string(), // Would need price oracle integration
    };
    
    to_binary(&portfolio)
}

fn query_sync_status(deps: Deps) -> StdResult<Binary> {
    // Get all tokens
    let tokens_response: GetAllTokensResponse = deps.querier.query(&QueryRequest::Custom(TSBQuery::GetAllTokens {}))?;
    
    // Get pending operations
    let pending_response: GetPendingBitcoinSyncResponse = deps.querier.query(&QueryRequest::Custom(TSBQuery::GetPendingBitcoinSync {}))?;
    
    // Get tokens for sync
    let sync_response: GetTokensForSyncResponse = deps.querier.query(&QueryRequest::Custom(TSBQuery::GetTokensForSync {}))?;
    
    let synced_count = tokens_response.tokens.iter()
        .filter(|t| t.synced_with_bitcoin)
        .count() as u32;
    
    let status = SyncStatus {
        total_tokens: tokens_response.tokens.len() as u32,
        synced_tokens: synced_count,
        pending_operations: pending_response.operations.len() as u32,
        tokens_for_sync: sync_response.token_ids,
    };
    
    to_binary(&status)
} 