use cosmwasm_std::{ Order, StdResult, Storage};
use cw_storage_plus::Map;

use crate::escrow::Escrow;


pub const ESCROWS: Map<&str, Escrow> = Map::new("escrow");

/// This returns the list of ids for all registered escrows
pub fn all_escrow_ids(storage: &dyn Storage) -> StdResult<Vec<String>> {
    ESCROWS
        .keys(storage, None, None, Order::Ascending)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{Addr, Uint128, Timestamp} ;    
    use cw20::{Balance, Cw20CoinVerified};
    use cosmwasm_std::testing::MockStorage;

    #[test]
    fn no_escrow_ids() {
        let storage = MockStorage::new();
        let ids = all_escrow_ids(&storage).unwrap();
        assert_eq!(0, ids.len());
    }

    fn dummy_escrow() -> Escrow {
        let coin = Cw20CoinVerified {
            address: Addr::unchecked("coin_address"),
            amount: Uint128::new(100),
        };
        let mut env = mock_env();
        env.block.time = Timestamp::from_seconds(1);
        let e = Escrow::create(
            &env,
            Addr::unchecked("user_a"),
            Addr::unchecked("user_b"),
            1000,
            2000,
            Balance::Cw20(coin),
        );
        return e.unwrap();
    }

    #[test]
    fn all_escrow_ids_in_order() {
        let mut storage = MockStorage::new();
        ESCROWS.save(&mut storage, "lazy", &dummy_escrow()).unwrap();
        ESCROWS.save(&mut storage, "assign", &dummy_escrow()).unwrap();
        ESCROWS.save(&mut storage, "zen", &dummy_escrow()).unwrap();

        let ids = all_escrow_ids(&storage).unwrap();
        assert_eq!(3, ids.len());
        assert_eq!(
            vec!["assign".to_string(), "lazy".to_string(), "zen".to_string()],
            ids
        )
    }
}
