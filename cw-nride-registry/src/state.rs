use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Order, StdResult, StdError, Storage};
use cw_storage_plus::Map;


#[cw_serde]
pub struct Record {
    pub reg_addr: Addr,
    pub nkn_addr: String,
    pub location: String,
}
     
pub const RECORDS: Map<Addr, Record> = Map::new("registry");

#[cfg(test)]
mod tests {
    use super::*;
  
    use cosmwasm_std::testing::MockStorage;

    #[test]
    fn add_records() {
        let mut storage = MockStorage::new();
        
        RECORDS.save(
            &mut storage, 
            Addr::unchecked("alice"), 
            &Record{
                reg_addr:Addr::unchecked("alice"), 
                nkn_addr:"nknalice".to_string(), 
                location: "world".to_string(),
            },
        ).unwrap();
        RECORDS.save(
            &mut storage, 
            Addr::unchecked("bob"), 
            &Record{
                reg_addr:Addr::unchecked("bob"), 
                nkn_addr:"nknbob".to_string(), 
                location: "world".to_string(),
            },
        ).unwrap();
        RECORDS.save(
            &mut storage, 
            Addr::unchecked("charlie"), 
            &Record{
                reg_addr:Addr::unchecked("charlie"), 
                nkn_addr:"nkncharlie".to_string(), 
                location: "world".to_string(),
            },
        ).unwrap();

        
        let ids = RECORDS
            .keys(&storage, None, None, Order::Ascending)
            .collect::<Result<Vec<Addr>,_>>()
            .unwrap();

        assert_eq!(3, ids.len());
    }
}
