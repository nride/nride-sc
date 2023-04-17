use cosmwasm_schema::{cw_serde};

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
     // Returns a list of records subscribed to a given location
    List {location: String},
   
    // Returns a list of records subscribed to one, or multiple locations
    ListMultiple {locations: Vec<String>},

    // Returns the record for a given registry address
    Details {address: String},
}

