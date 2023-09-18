use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use k256::{
    ecdsa::SigningKey,              
    elliptic_curve::sec1::ToEncodedPoint,
};

use cosmwasm_std::Addr;
use cw20::Balance;

use crate::error::EscrowError;

#[derive(Serialize, Deserialize, JsonSchema, Clone, PartialEq, Debug)]
pub struct Escrow {
    /// user_a creates the escrow
    pub user_a: Addr,
    /// user_b is the counterparty
    pub user_b: Addr,
    /// deposit are the funds locked in escrow
    pub deposit: Balance,
    /// lock is the public key that guards the deposit. 
    /// the corresponding private key is necessary to withdraw.
    pub lock: String,
    /// close indicates whether the escrow is closed and already settled
    /// if this value is true, it is assumed that all payouts have already
    /// been settled
    pub closed: bool,
}

impl Escrow {
    pub fn create(
        user_a: Addr,
        user_b: Addr,
        deposit: Balance,
        lock: &str,
    ) -> Result<Self,EscrowError> {
        
        if deposit.is_empty() {
            return Err(EscrowError::EmptyDeposit {});
        }

        Ok(Escrow{
            user_a,
            user_b,
            deposit,
            lock: lock.to_string(),
            closed: false,
        })
    }

    /// check secret against lock
    /// Returns an EscrowError:InvalidSecret if the secret is invalid
    pub fn unlock(&mut self, secret:&str) -> Result<(), EscrowError> {        
        let private_key = hex::decode(secret);
        if private_key.is_err() {
            return Err(EscrowError::InvalidSecret {  });
        }

        let recomputed_public_key = SigningKey::from_bytes(&private_key.unwrap())
        .unwrap()
        .verifying_key()
        .to_encoded_point(true)
        .as_bytes()
        .to_vec();

        let recomputed_public_key_str = hex::encode(recomputed_public_key);

        if recomputed_public_key_str == self.lock {
            return Ok(());
        }

        return Err(EscrowError::InvalidSecret { });
    }

    /// close sets the closed flag to true 
    /// we can only close if the payout has already been computed
    /// which indireclty ensures that the escrow is in a closeable 
    /// state
    pub fn close(&mut self) {
        self.closed = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use cosmwasm_std::Uint128;
    use cw20::Cw20CoinVerified;

    const  DUMMY_LOCK: &str = "0330347c5cb0f1627bdd2e7b082504a443b2bf50ad2e3efbb4e754ebd687c78c24";
    const  DUMMY_SECRET: &str =  "27874aa2b70ce7281c94413c36d44fac6fa6a1198f2c529188c4dd4f7a4e1870";   
    static DUMMY_SECRET_INCORRECT: &str = "27874aa2b70ce7281c94413c36d44fac6fa6a1198f2c529188c4dd4f7a4exxxx";
    
    #[test]
    fn escrow_create_happy() {     
        let coin = Cw20CoinVerified {
            address: Addr::unchecked("coin_address"),
            amount: Uint128::new(100),
        };
        let deposit = Balance::Cw20(coin);

        let e = Escrow::create(
            Addr::unchecked("user_a"),
            Addr::unchecked("user_b"),
            deposit.clone(),
            DUMMY_LOCK,
        ).unwrap();
        
    
        assert_eq!(e.deposit, deposit.clone());
        assert_eq!(e.lock, DUMMY_LOCK.to_string());
    }

    #[test]
    fn escrow_create_empty_deposit() {
        let empty_coin = Cw20CoinVerified {
            address: Addr::unchecked("coin_address"),
            amount: Uint128::new(0),
        };
        let empty_deposit = Balance::Cw20(empty_coin);

        let err = Escrow::create(
            Addr::unchecked("user_a"),
            Addr::unchecked("user_b"),
            empty_deposit,
            DUMMY_LOCK,
        ).unwrap_err();
        assert!(matches!(err, EscrowError::EmptyDeposit{}));
    }

    #[test]
    fn escrow_unlock() {     
        let coin = Cw20CoinVerified {
            address: Addr::unchecked("coin_address"),
            amount: Uint128::new(100),
        };
        let deposit = Balance::Cw20(coin);

        let mut e = Escrow::create(
            Addr::unchecked("user_a"),
            Addr::unchecked("user_b"),
            deposit.clone(),
            DUMMY_LOCK,
        ).unwrap();
        
        // correct secret
        let _ = e.unlock(DUMMY_SECRET).unwrap();

        // invalid secret
        let err = e.unlock(DUMMY_SECRET_INCORRECT).unwrap_err();
        assert!(matches!(err, EscrowError::InvalidSecret{}));
    }
}