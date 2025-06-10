use cosmwasm_std::{
    to_json_binary, Addr, Binary, CosmosMsg, CustomQuery, Deps, DepsMut, Decimal, Env, Event,
    MessageInfo, QueryRequest, Response, StdError, StdResult, WasmMsg,
};
use cw_storage_plus::Item;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::json;
use thiserror::Error;
use cosmwasm_schema::{cw_serde, QueryResponses};

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TorramQueryWrapper {
    pub method: String,
    pub args: serde_json::Value,
}

impl CustomQuery for TorramQueryWrapper {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Contact {
    pub address: String,
    pub contact: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ContactsResponse {
    pub contacts: Vec<Contact>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct PriceData {
    pub btc: Decimal,
    pub eth: Decimal,
    pub usdc: Decimal,
    pub usdt: Decimal,
    pub dai: Decimal,
}

pub const PRICES: Item<PriceData> = Item::new("prices");
pub const ADMIN: Item<Addr> = Item::new("admin");

#[cw_serde]
pub struct InstantiateMsg {
    pub admin: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    UpdatePrices {
        btc: Decimal,
        eth: Decimal,
        usdc: Decimal,
        usdt: Decimal,
        dai: Decimal,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(PriceData)]
    GetPrices {},
    #[returns(Binary)]
    FetchFromOracle {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct CwTemplateContract(pub Addr);

impl CwTemplateContract {
    pub fn addr(&self) -> Addr {
        self.0.clone()
    }

    pub fn call<T: Into<ExecuteMsg>>(&self, msg: T) -> StdResult<CosmosMsg> {
        let msg = to_json_binary(&msg.into())?;
        Ok(WasmMsg::Execute {
            contract_addr: self.addr().into(),
            msg,
            funds: vec![],
        }
        .into())
    }
}

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    deps.api.debug("reached out to here (instantiate)");
    let admin = deps.api.addr_validate(&msg.admin)?;
    ADMIN.save(deps.storage, &admin)?;
    PRICES.save(
        deps.storage,
        &PriceData {
            btc: Decimal::zero(),
            eth: Decimal::zero(),
            usdc: Decimal::zero(),
            usdt: Decimal::zero(),
            dai: Decimal::zero(),
        },
    )?;
    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("admin", msg.admin))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut<TorramQueryWrapper>,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, StdError> {
    match msg {
        ExecuteMsg::UpdatePrices {
            btc,
            eth,
            usdc,
            usdt,
            dai,
        } => try_update_prices(deps, info, btc, eth, usdc, usdt, dai),
    }
}

fn try_update_prices(
    deps: DepsMut<TorramQueryWrapper>,
    info: MessageInfo,
    btc: Decimal,
    eth: Decimal,
    usdc: Decimal,
    usdt: Decimal,
    dai: Decimal,
) -> Result<Response, StdError> {
    let admin = ADMIN.load(deps.storage)?;
    if info.sender != admin {
        return Err(StdError::generic_err("Unauthorized"));
    }
    let data = PriceData { btc, eth, usdc, usdt, dai };
    PRICES.save(deps.storage, &data)?;

    let avg = average_price(&data);

    Ok(Response::new()
        .add_attribute("action", "update_prices")
        .add_attribute("sender", info.sender)
        .add_event(
            Event::new("oracle_prices")
                .add_attribute("usdc", data.usdc.to_string())
                .add_attribute("usdt", data.usdt.to_string())
                .add_attribute("dai", data.dai.to_string())
                .add_attribute("average", avg.to_string()),
        ))
}

fn average_price(prices: &PriceData) -> Decimal {
    (prices.usdc + prices.usdt + prices.dai) / Decimal::from_atomics(3u128, 0).unwrap()
}

fn query_oracle_prices(deps: Deps<TorramQueryWrapper>) -> StdResult<Binary> {
    let request = QueryRequest::Custom(TorramQueryWrapper {
        method: "get_all_contacts".to_string(),
        args: json!({}),
    });
    deps.api.debug(&format!(
        "QueryRequest::Custom payload = {}",
        serde_json::to_string(&request).unwrap()
    ));
    let response: ContactsResponse = deps.querier.query(&request)?;
    deps.api.debug(&format!("got contacts = {:?}", response));
    if response.contacts.is_empty() {
        return Err(StdError::generic_err("No oracle contacts found"));
    }
    let contact_str = &response.contacts[0].contact;

    fn extract_price(s: &str, key: &str) -> Result<Decimal, StdError> {
        s.split(',')
            .find(|entry| entry.trim_start().starts_with(&format!("{key}:")))
            .ok_or_else(|| StdError::generic_err(format!("{} not found in contact data", key)))?
            .split(':')
            .nth(1)
            .ok_or_else(|| StdError::generic_err(format!("{} format invalid", key)))?
            .trim()
            .parse::<Decimal>()
            .map_err(|e| StdError::generic_err(format!("{} parse error: {}", key, e)))
    }

    let price_data = PriceData {
        btc: extract_price(contact_str, "BTC")?,
        eth: extract_price(contact_str, "ETH")?,
        usdc: extract_price(contact_str, "USDC")?,
        usdt: extract_price(contact_str, "USDT")?,
        dai: extract_price(contact_str, "DAI")?,
    };

    to_json_binary(&price_data)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps<TorramQueryWrapper>, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetPrices {} => to_json_binary(&PRICES.load(deps.storage)?),
        QueryMsg::FetchFromOracle {} => query_oracle_prices(deps),
    }
}
