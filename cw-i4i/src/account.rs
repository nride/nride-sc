use std;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use k256::{
    ecdsa::SigningKey,              
    elliptic_curve::sec1::ToEncodedPoint,
};

use crate::error::AccountError;

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub enum AccountStatus {
    Init,
    Funded,
}

impl std::fmt::Display for AccountStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            AccountStatus::Init => write!(f, "INIT"),
            AccountStatus::Funded => write!(f, "FUNDED"),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub enum UserAction {
    None,
    Approved,
    Cancelled,
    T1,
    T2,
}

impl std::fmt::Display for UserAction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            UserAction::None => write!(f, "NONE"),
            UserAction::Approved => write!(f, "APPROVED"),
            UserAction::Cancelled => write!(f, "CANCELLED"),
            UserAction::T1 => write!(f, "T1"),
            UserAction::T2 => write!(f, "T2"),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct Account {
    pub status: AccountStatus,
    pub action: UserAction,
    pub lock: Option<String>,
}

impl Account {
    /// This creates a new Account in the [INIT, NONE] state
    pub fn new() -> Self {
        Account{
            status: AccountStatus::Init,
            action: UserAction::None,
            lock: None,
        }
    }

    /// This triggers a state transition from [INIT, NONE] to [FUNDED, NONE]
    /// if the account is in any state other than [INIT, NONE], the function returns 
    /// an AccountError::InvalidState
    pub fn fund(&mut self) -> Result<(),AccountError> {
        match (&self.status, &self.action) {
            (AccountStatus::Init, UserAction::None )=> {
                self.status = AccountStatus::Funded;
                return Ok(());
            },
            _other => {
                return Err(AccountError::InvalidState{
                    action: "fund".to_string(),
                    state: self.to_string(),
                });
            },
        }
    }

    pub fn set_lock(&mut self, lock: &str) {
        self.lock = Some(lock.to_string());
    }

    /// This triggers a state transition from [FUNDED, NONE] to [FUNDED, APPROVED]
    /// if the account is in any state other than [FUNDED, NONE], the function returns 
    /// an AccountError::InvalidState
    /// The operation is guarded by the account lock, so the method accepts a secret
    /// with which we attempt to unlock the account before approving it
    pub fn approve(&mut self, secret: &str) -> Result<(),AccountError> {
        match (&self.status, &self.action) {
            (AccountStatus::Funded, UserAction::None) => {
                let unlock_res = self.unlock(secret);
                if unlock_res.is_err() {
                    return unlock_res;
                }
                self.action = UserAction::Approved;
                return Ok(());
            },
            _other => {
                return Err(AccountError::InvalidState{
                    action: "approve".to_string(),
                    state: self.to_string(),
                });
            },
        }
    }

    fn unlock(&mut self, secret:&str) -> Result<(), AccountError> {
        if self.lock.is_none() {
            return Err(AccountError::NoLock { });
        }
        
        let private_key = hex::decode(secret);
        if private_key.is_err() {
            return Err(AccountError::InvalidSecret {  });
        }

        let recomputed_public_key = SigningKey::from_bytes(&private_key.unwrap())
        .unwrap()
        .verifying_key()
        .to_encoded_point(false)
        .as_bytes()
        .to_vec();

        let recomputed_public_key_str = hex::encode(recomputed_public_key);

        let lock = self.lock.clone().unwrap();

        if recomputed_public_key_str == lock {
            return Ok(());
        }

        return Err(AccountError::InvalidSecret { });
    }

    /// This triggers a state transition from [FUNDED, NONE] to [FUNDED, CANCEL]
    /// if the account is in any state other than [FUNDED, NONE], the function returns 
    /// an AccountError::InvalidState
    pub fn cancel(&mut self) -> Result<(),AccountError> {
        match (&self.status, &self.action) {
            (AccountStatus::Funded, UserAction::None) => {
                self.action = UserAction::Cancelled;
                return Ok(());
            },
            _other => {
                return Err(AccountError::InvalidState{
                    action: "cancel".to_string(),
                    state: self.to_string(),
                });
            },
        }
    }

    /// This triggers a state transition from [INIT, NONE]  to [INIT, T1]
    /// if the account is in any state other than [INIT, NONE], the function returns 
    /// an AccountError::InvalidState
    pub fn t1(&mut self) -> Result<(),AccountError> {
        match (&self.status, &self.action) {
            (AccountStatus::Init, UserAction::None) => {
                self.action = UserAction::T1;
                return Ok(());
            }
            _other => {
                return Err(AccountError::InvalidState{
                    action: "t1".to_string(),
                    state: self.to_string(),
                });
            },
        }
    }

    /// This triggers a state transition from [FUNDED, NONE]  to [FUNDED, T2]
    /// if the account is in any state other than [FUNDED, NONE], the function returns 
    /// an AccountError::InvalidState
    pub fn t2(&mut self) -> Result<(),AccountError> {
        match (&self.status, &self.action) {
            (AccountStatus::Funded, UserAction::None) => {
                self.action = UserAction::T2;
                return Ok(());
            },
            _other => {
                return Err(AccountError::InvalidState{
                    action: "t2".to_string(),
                    state: self.to_string(),
                });
            },
        }
    }
}

impl std::fmt::Display for Account {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[{},{}]", self.status, self.action)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{collections::HashMap};

    static DUMMY_LOCK: &str = "04b4ac68eff3a82d86db5f0489d66f91707e99943bf796ae6a2dcb2205c9522fa7915428b5ac3d3b9291e62142e7246d85ad54504fabbdb2bae5795161f8ddf259";
    static DUMMY_SECRET_CORRECT: &str = "3c9229289a6125f7fdf1885a77bb12c37a8d3b4962d936f7e3084dece32a3ca1";
    static DUMMY_SECRET_INCORRECT: &str = "3c9229289a6125f7fdf1885a77bb12c37a8d3b4962d936f7e3084dece32xxxxx";

     fn all_states() -> HashMap<String, Account> {
        return HashMap::from([
            ("[INIT,NONE]".to_string(), Account{status: AccountStatus::Init, action: UserAction::None, lock: None}),
            ("[INIT,APPROVED]".to_string(), Account{status: AccountStatus::Init, action: UserAction::Approved, lock: None}),
            ("[INIT,CANCELLED]".to_string(), Account{status: AccountStatus::Init, action: UserAction::Cancelled, lock: None}),
            ("[INIT,T1]".to_string(), Account{status: AccountStatus::Init, action: UserAction::T1, lock: None}),
            ("[INIT,T2]".to_string(), Account{status: AccountStatus::Init, action: UserAction::T2, lock: None}),
            ("[FUNDED,NONE]".to_string(), Account{status: AccountStatus::Funded, action: UserAction::None, lock: None}),
            ("[FUNDED,APPROVED]".to_string(), Account{status: AccountStatus::Funded, action: UserAction::Approved, lock: None}),
            ("[FUNDED,CANCELLED]".to_string(), Account{status: AccountStatus::Funded, action: UserAction::Cancelled, lock: None}),
            ("[FUNDED,T1]".to_string(), Account{status: AccountStatus::Funded, action: UserAction::T1, lock: None}),
            ("[FUNDED,T2]".to_string(), Account{status: AccountStatus::Funded, action: UserAction::T2, lock: None}),
        ]); 
    }

    #[test]
    fn account_initialization() {
        let a = Account::new();
        assert_eq!(a.status, AccountStatus::Init);
        assert_eq!(a.action, UserAction::None);
    }

    #[test]
    fn account_fund() {
        let mut test_cases = all_states();

        for (key,tc) in test_cases.iter_mut() {
            // happy case
            // calling fund() on a new Account should succeed and the account should end up 
            // in the [FUNDED, NONE] state
           if key == "[INIT,NONE]" {
            _ = tc.fund().unwrap();
            assert_eq!(tc.status, AccountStatus::Funded);
            assert_eq!(tc.action, UserAction::None);
            continue;
           }

            // calling fund() on an account that is in any state other than [INIT, NONE] should
            // return a AccountError:InvalidState, and the account should remain in the same
            // state
            let copy = tc.clone();
            let err = tc.fund().unwrap_err();
            assert_eq!(err, AccountError::InvalidState{
                action: "fund".to_string(),
                state: key.to_string(),
            });
            assert_eq!(tc.status, copy.status);
            assert_eq!(tc.action, copy.action);
            assert_eq!(tc.lock, None);
        }
    }

    #[test]
    fn account_fund_setlock_approve_sequence() {
        // happy case
        let mut a = Account::new();
        let _ = a.fund().unwrap();
        let _ = a.set_lock(DUMMY_LOCK);
        let _ = a.approve(DUMMY_SECRET_CORRECT);

        // bad secret
        let mut a = Account::new();
        let _ = a.fund().unwrap();
        let _ = a.set_lock(DUMMY_LOCK);
        let err = a.approve(DUMMY_SECRET_INCORRECT).unwrap_err();
        assert!(matches!(err, AccountError::InvalidSecret{}));
    }

    #[test]
    fn account_approve_nolock() {
        // try to approve when lock not set
        let mut a = Account::new();
        let _ = a.fund().unwrap();
        // don't set lock
        let err = a.approve(DUMMY_SECRET_INCORRECT).unwrap_err();
        assert!(matches!(err, AccountError::NoLock{}));
    }

    #[test]
    fn account_approve_states() {
        let mut test_cases = all_states();

        for (key,tc) in test_cases.iter_mut() {
            // prepare the test case with lock
            tc.set_lock(DUMMY_LOCK);
            
            // happy case
            // calling approve, with the correct secret, on an account that is
            // in the [FUNDED, NONE] state should succeed and the account should
            // end up in the [FUNDED, APPROVED] state
           if key == "[FUNDED,NONE]" {
            let _ = tc.approve(DUMMY_SECRET_CORRECT).unwrap();
            assert_eq!(tc.status, AccountStatus::Funded);
            assert_eq!(tc.action, UserAction::Approved);
            continue;
           }

            // calling fund() on an account that is in any state other than [FUNDED, NONE] should
            // return a AccountError:InvalidState, and the account should remain in the same
            // state
            let copy = tc.clone();
            let err = tc.approve(DUMMY_SECRET_CORRECT).unwrap_err();
            assert_eq!(err, AccountError::InvalidState{
                action: "approve".to_string(),
                state: key.to_string(),
            });
            assert_eq!(tc.status, copy.status);
            assert_eq!(tc.action, copy.action);
        }
    }
    
    #[test]
    fn account_cancel() {
        let mut test_cases = all_states();

        for (key,tc) in test_cases.iter_mut() {
            // happy case
            // calling cancel() on an account that is in the [FUNDED, NONE] state should succeed
            // and the account should end up in the [FUNDED, CANCELLED] state
           if key == "[FUNDED,NONE]" {
            let _ = tc.cancel().unwrap();
            assert_eq!(tc.status, AccountStatus::Funded);
            assert_eq!(tc.action, UserAction::Cancelled);
            continue;
           }

            // calling cancel() on an account that is in any state other than [FUNDED, NONE] should
            // return a AccountError:InvalidState, and the account should remain in the same
            // state
            let copy = tc.clone();
            let err = tc.cancel().unwrap_err();
            assert_eq!(err, AccountError::InvalidState{
                action: "cancel".to_string(),
                state: key.to_string(),
            });
            assert_eq!(tc.status, copy.status);
            assert_eq!(tc.action, copy.action);
        }
    }

    #[test]
    fn account_t1() {
        let mut test_cases = all_states();

        for (key,tc) in test_cases.iter_mut() {
            // happy case
            // calling t1() on an account that is in the [INIT, NONE] state should succeed
            // and the account should end up in the [INIT, T1] state
           if key == "[INIT,NONE]" {
            let copy = tc.clone();
            let _ = tc.t1().unwrap();
            assert_eq!(tc.status, copy.status);
            assert_eq!(tc.action, UserAction::T1);
            continue;
           }

            // calling t1() on an account that is in any state other than [INIT, NONE] should
            // return a AccountError:InvalidState, and the account should remain in the same
            // state
            let copy = tc.clone();
            let err = tc.t1().unwrap_err();
            assert_eq!(err, AccountError::InvalidState{
                action: "t1".to_string(),
                state: key.to_string(),
            });
            assert_eq!(tc.status, copy.status);
            assert_eq!(tc.action, copy.action);
        }
    }

    #[test]
    fn account_t2() {
        let mut test_cases = all_states();

        for (key,tc) in test_cases.iter_mut() {
            // happy case
            // calling t2() on an account that is in the [FUNDED, NONE] state should succeed
            // and the account should end up in the [FUNDED, T2] state
           if key == "[FUNDED,NONE]" {
            let copy = tc.clone();
            let _ = tc.t2().unwrap();
            assert_eq!(tc.status, copy.status);
            assert_eq!(tc.action, UserAction::T2);
            continue;
           }

            // calling t2() on an account that is in any state other than [FUNDED, NONE] should
            // return a AccountError:InvalidState, and the account should remain in the same
            // state
            let copy = tc.clone();
            let err = tc.t2().unwrap_err();
            assert_eq!(err, AccountError::InvalidState{
                action: "t2".to_string(),
                state: key.to_string(),
            });
            assert_eq!(tc.status, copy.status);
            assert_eq!(tc.action, copy.action);
        }
    }

    #[test]
    fn gen_key() {

        let secret =    "cde73ee8f8584c54ac455c941f75990f4bff47a4340023e3fd236344e9a7d4ea";
        let private_key = hex::decode(secret).unwrap();

        let recomputed_public_key = SigningKey::from_bytes(&private_key)
        .unwrap()
        .verifying_key()
        .to_encoded_point(false)
        .as_bytes()
        .to_vec();

        let recomputed_public_key_str = hex::encode(recomputed_public_key);

        println!("{}", recomputed_public_key_str);

        // 042d5f7beb52d336163483804facb17c47033fb14dfc3f3c88235141bae1896fc8d99a685aafaf92d5f41d866fe387b988a998590326f1b549878b9d03eabed7e5
    }
}