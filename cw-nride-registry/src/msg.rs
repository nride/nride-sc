use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    Subscribe(SubscribeMsg),
}

#[cw_serde]
pub struct SubscribeMsg {
    pub nkn_addr: String,
    pub location: String, 
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {}
