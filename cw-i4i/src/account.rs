use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::error::AccountError;

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub enum AccountStatus {
    Init,
    Funded,
    Closed,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub enum UserAction {
    None,
    Approved,
    Cancelled,
    T2,
}
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct Account {
    pub status: AccountStatus,
    pub action: UserAction,
}

impl Account {
    /// This creates a new Account in the [INIT, NONE] state
    pub fn new() -> Self {
        Account{
            status: AccountStatus::Init,
            action: UserAction::None,
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
                return Err(AccountError::InvalidState{});
            },
        }
    }

    /// This triggers a state transition from [FUNDED, NONE] to [FUNDED, APPROVED]
    /// if the account is in any state other than [FUNDED, NONE], the function returns 
    /// an AccountError::InvalidState
    pub fn approve(&mut self) -> Result<(),AccountError> {
        match (&self.status, &self.action) {
            (AccountStatus::Funded, UserAction::None) => {
                self.action = UserAction::Approved;
                return Ok(());
            },
            _other => {
                return Err(AccountError::InvalidState{});
            },
        }
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
                return Err(AccountError::InvalidState{});
            },
        }
    }

    /// This triggers a state transition from [INIT|FUNDED, NONE]  to [INIT|FUNDED, T2]
    /// if the account is in any state other than [INIT|FUNDED, NONE], the function returns 
    /// an AccountError::InvalidState
    pub fn t2(&mut self) -> Result<(),AccountError> {
        match (&self.status, &self.action) {
            (AccountStatus::Init, UserAction::None) => {
                self.action = UserAction::T2;
                return Ok(());
            }
            (AccountStatus::Funded, UserAction::None) => {
                self.action = UserAction::T2;
                return Ok(());
            },
            _other => {
                return Err(AccountError::InvalidState{});
            },
        }
    }

    /// This triggers a state transition from [FUNDED, APPROVED|CANCELLED|T2]  to [CLOSED, APPROVED|CANCELLED|T2]
    /// if the account is in any state other than [FUNDED, APPROVED|CANCELLED|T2], the function returns 
    /// an AccountError::InvalidState
    pub fn close(&mut self) -> Result<(),AccountError> {
        match &self.status {
            AccountStatus::Funded => {
                match &self.action {
                    UserAction::None => {
                        return Err(AccountError::InvalidState {  });
                    },
                    _other => {
                        self.status = AccountStatus::Closed;
                        return Ok(());
                    },
                };
            },
            _other => {
                return Err(AccountError::InvalidState{});
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

     fn all_states() -> HashMap<String, Account> {
        return HashMap::from([
            ("[INIT,NONE]".to_string(), Account{status: AccountStatus::Init, action: UserAction::None}),
            ("[INIT,APPROVED]".to_string(), Account{status: AccountStatus::Init, action: UserAction::Approved}),
            ("[INIT,CANCELLED]".to_string(), Account{status: AccountStatus::Init, action: UserAction::Cancelled}),
            ("[INIT,T2]".to_string(), Account{status: AccountStatus::Init, action: UserAction::T2}),
            ("[FUNDED,NONE]".to_string(), Account{status: AccountStatus::Funded, action: UserAction::None}),
            ("[FUNDED,APPROVED]".to_string(), Account{status: AccountStatus::Funded, action: UserAction::Approved}),
            ("[FUNDED,CANCELLED]".to_string(), Account{status: AccountStatus::Funded, action: UserAction::Cancelled}),
            ("[FUNDED,T2]".to_string(), Account{status: AccountStatus::Funded, action: UserAction::T2}),
            ("[CLOSED,NONE]".to_string(), Account{status: AccountStatus::Closed, action: UserAction::None}),
            ("[CLOSED,APPROVED]".to_string(), Account{status: AccountStatus::Closed, action: UserAction::Approved}),
            ("[CLOSED,CANCELLED]".to_string(), Account{status: AccountStatus::Closed, action: UserAction::Cancelled}),
            ("[CLOSED,T2]".to_string(), Account{status: AccountStatus::Closed, action: UserAction::T2}),
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
            let _ = tc.fund().unwrap();
            assert_eq!(tc.status, AccountStatus::Funded);
            assert_eq!(tc.action, UserAction::None);
            continue;
           }

            // calling fund() on an account that is in any state other than [INIT, NONE] should
            // return a AccountError:InvalidState, and the account should remain in the same
            // state
            let copy = tc.clone();
            let err = tc.fund().unwrap_err();
            assert!(matches!(err, AccountError::InvalidState{}));
            assert_eq!(tc.status, copy.status);
            assert_eq!(tc.action, copy.action);
        }
    }

    #[test]
    fn account_approve() {
        let mut test_cases = all_states();

        for (key,tc) in test_cases.iter_mut() {
            // happy case
            // calling ok() on an account that is in the [FUNDED, NONE] state should succeed
            // and the account should end up in the [FUNDED, APPROVED] state
           if key == "[FUNDED,NONE]" {
            let _ = tc.approve().unwrap();
            assert_eq!(tc.status, AccountStatus::Funded);
            assert_eq!(tc.action, UserAction::Approved);
            continue;
           }

            // calling fund() on an account that is in any state other than [FUNDED, NONE] should
            // return a AccountError:InvalidState, and the account should remain in the same
            // state
            let copy = tc.clone();
            let err = tc.approve().unwrap_err();
            assert!(matches!(err, AccountError::InvalidState{}));
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
            assert!(matches!(err, AccountError::InvalidState{}));
            assert_eq!(tc.status, copy.status);
            assert_eq!(tc.action, copy.action);
        }
    }

    #[test]
    fn account_t2() {
        let mut test_cases = all_states();

        for (key,tc) in test_cases.iter_mut() {
            // happy case
            // calling t2() on an account that is in the [INIT|FUNDED, NONE] state should succeed
            // and the account should end up in the [INIT|FUNDED, T2] state
           if key == "[INIT,NONE]" || key == "[FUNDED,NONE]" {
            let copy = tc.clone();
            let _ = tc.t2().unwrap();
            assert_eq!(tc.status, copy.status);
            assert_eq!(tc.action, UserAction::T2);
            continue;
           }

            // calling t2() on an account that is in any state other than [INIT|FUNDED, NONE] should
            // return a AccountError:InvalidState, and the account should remain in the same
            // state
            let copy = tc.clone();
            let err = tc.t2().unwrap_err();
            assert!(matches!(err, AccountError::InvalidState{}));
            assert_eq!(tc.status, copy.status);
            assert_eq!(tc.action, copy.action);
        }
    }

    #[test]
    fn account_close() {
        let mut test_cases = all_states();

        for (key,tc) in test_cases.iter_mut() {
            // happy case
            // calling t2() on an account that is in the [FUNDED, APPROVED|CANCELLED|T2] state should succeed
            // and the account should end up in the [CLOSED, OK|CANCELLED|T2] state
           if key == "[FUNDED,APPROVED]" ||
           key == "[FUNDED,CANCELLED]" ||
           key == "[FUNDED,T2]" {
            let copy = tc.clone();
            let _ = tc.close().unwrap();
            assert_eq!(tc.status, AccountStatus::Closed);
            assert_eq!(tc.action, copy.action);
            continue;
           }

            // calling close() on an account that is in any state other than [FUNDED, APPROVED|CANCELLED|T2] should
            // return a AccountError:InvalidState, and the account should remain in the same
            // state
            let copy = tc.clone();
            let err = tc.close().unwrap_err();
            assert!(matches!(err, AccountError::InvalidState{}));
            assert_eq!(tc.status, copy.status);
            assert_eq!(tc.action, copy.action);
        }
    }
}