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
pub enum QueryMsg {
    // Returns a list of nkn addresses subscribed to a given location
    List {location: String},
    // Returns the record for a given registry address
    Details {address: String},
}

