use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cw20::{ Cw20ReceiveMsg, Balance};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Create(CreateMsg),
    TopUp(TopUpMsg),
    /// This accepts a properly-encoded ReceiveMsg from a cw20 contract
    Receive(Cw20ReceiveMsg),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ReceiveMsg {
    Create(CreateMsg),
    TopUp(TopUpMsg),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CreateMsg {
    /// id is a human-readable name for the escrow to use later
    /// 3-20 bytes of utf-8 text
    pub id: String,
    pub user_b: String,
    pub t1_timeout: u64,
    pub t2_timeout: u64,
    pub account_b_lock: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TopUpMsg {
    pub id: String,
    pub account_a_lock: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    /// Show all open escrows. Return type is ListResponse.
    List {},
    /// Returns the details of the named escrow, error if not created
    /// Return type: DetailsResponse.
    Details { id: String },
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct ListResponse {
    /// list all registered ids
    pub escrows: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct DetailsResponse {
    /// id of this escrow
    pub id: String,
    pub user_a: String,
    pub account_a_state: String,
    pub account_a_lock: Option<String>,
    pub user_b: String,
    pub account_b_state: String,
    pub account_b_lock: Option<String>,
    pub t1_timeout: u64,
    pub t2_timeout: u64,
    pub required_deposit: Balance,
}