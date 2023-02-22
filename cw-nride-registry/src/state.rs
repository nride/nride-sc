use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr};
use cw_storage_plus::{MultiIndex, Index, IndexList, IndexedMap};


#[cw_serde]
pub struct Record {
    pub reg_addr: Addr,
    pub nkn_addr: String,
    pub location: String,
}
     
pub struct RecordIndexes<'a> {
    pub location: MultiIndex<'a, String, Record, Addr>,
}

impl<'a> IndexList<Record> for RecordIndexes<'a> {
    fn get_indexes(&'_ self) -> Box<dyn Iterator<Item = &'_ dyn Index<Record>> + '_> {
        let v: Vec<&dyn Index<Record>> = vec![&self.location];
        Box::new(v.into_iter())
    }
}

pub fn records<'a>() -> IndexedMap<'a, &'a Addr, Record, RecordIndexes<'a>> {
    let indexes = RecordIndexes {
        location: MultiIndex::new(
            |_key, r| r.location.clone(),
            "records",
            "records__location",
        ),
    };
    IndexedMap::new("records", indexes)
}

#[cfg(test)]
mod tests {
    use super::*;
  
    use cosmwasm_std::testing::MockStorage;
    use cosmwasm_std::Order;
    
    #[test]
    fn add_records() {
        let mut storage = MockStorage::new();
        
        records().save(
            &mut storage, 
            &Addr::unchecked("alice"), 
            &Record{
                reg_addr:Addr::unchecked("alice"), 
                nkn_addr:"nknalice".to_string(), 
                location: "rome".to_string(),
            },
        ).unwrap();
        records().save(
            &mut storage, 
            &Addr::unchecked("bob"), 
            &Record{
                reg_addr:Addr::unchecked("bob"), 
                nkn_addr:"nknbob".to_string(), 
                location: "paris".to_string(),
            },
        ).unwrap();
        records().save(
            &mut storage, 
            &Addr::unchecked("charlie"), 
            &Record{
                reg_addr:Addr::unchecked("charlie"), 
                nkn_addr:"nkncharlie".to_string(), 
                location: "london".to_string(),
            },
        ).unwrap();
        records().save(
            &mut storage, 
            &Addr::unchecked("dennis"), 
            &Record{
                reg_addr:Addr::unchecked("dennis"), 
                nkn_addr:"nkndennis".to_string(), 
                location: "london".to_string(),
            },
        ).unwrap();
        

        let ids = records()
            .keys(&storage, None, None, Order::Ascending)
            .collect::<Result<Vec<Addr>,_>>()
            .unwrap();

        assert_eq!(4, ids.len());

        let res = records()
            .idx
            .location
            .prefix("london".to_string())
            .range(&storage, None, None, Order::Descending)
            .collect::<Vec<_>>();
        assert_eq!(2, res.len());

    }
}
