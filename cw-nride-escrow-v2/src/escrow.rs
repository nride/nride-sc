use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Env, Timestamp};
use cw20::Balance;

use crate::{account::{Account, UserAction}, error::EscrowError};

#[derive(Serialize, Deserialize, JsonSchema, Clone, Copy, PartialEq, Debug)]
pub struct Payout {
    pub user_a_basis_points: u8,
    pub user_b_basis_points: u8,
}

#[derive(Serialize, Deserialize, JsonSchema, Clone, PartialEq, Debug)]
pub struct Escrow {
    /// user_a creates the escrow
    pub user_a: Addr,
    /// user_b is the counterparty
    pub user_b: Addr,
    /// t1_timeout (in seconds since epoch 00:00:00 UTC on 1 January 1970).
    /// when the block time exceed this value, user_b canno't fund the escrow anymore,
    /// and user_a gets to withdraw the entirety of their deposit
    pub t1_timeout: u64,
    /// t2_timeout (in seconds since epoch 00:00:00 UTC on 1 January 1970).
    /// when the block time exceeds this value, the escrow is expired, neither user can
    /// approve or cancel anymore.
    pub t2_timeout: u64,
    /// the required deposit for each side of the escrow
    pub required_deposit: Balance,
    /// account_a is the account state-machine for user_a
    pub account_a: Account,
    /// account_a is the account state-machine for user_a
    pub account_b: Account,
    /// close indicates whether the escrow is closed and already settled
    /// if this value is true, it is assumed that all payouts have already
    /// been settled
    pub closed: bool,
    /// payout holds the result of compute_payout or none if the escrow is in a 
    /// state that doesn't allow withdrawals
    pub payout: Option<Payout>,
}

impl Escrow {
    pub fn create(
        env: &Env,
        user_a: Addr,
        user_b: Addr,
        t1_timeout: u64,
        t2_timeout: u64,
        deposit: Balance,
    ) -> Result<Self,EscrowError> {
        
        if deposit.is_empty() {
            return Err(EscrowError::EmptyDeposit {});
        }

        if t1_timeout >= t2_timeout  ||
        env.block.time >= Timestamp::from_seconds(t1_timeout) {
            return Err(EscrowError::InvalidTimeouts{});
        }
    
        // TODO: Check balance is positive
        
        Ok(Escrow{
            user_a,
            user_b,
            t1_timeout,
            t2_timeout,
            required_deposit: deposit,
            account_a: Account::new(),
            account_b: Account::new(),
            closed: false,
            payout: None,
        })
    }

    /// Check timeouts and trigger appropriate state transitions.
    fn check_timeouts(&mut self, env: Env) {
        if env.block.time >= Timestamp::from_seconds(self.t1_timeout) {
            _ =self.account_a.t1();
            _= self.account_b.t1();
        }
        if env.block.time >= Timestamp::from_seconds(self.t2_timeout) {
            _ =self.account_a.t2();
            _= self.account_b.t2();
        }
    }

    /// Set the sender's account AccountStatus to Funded
    /// Returns an AccountError if this is an invalid state transition
    /// Also sets the lock on the counterparty's account
    pub fn fund(&mut self, env: Env, sender: Addr, lock: &str) -> Result<(), EscrowError> {
        self.check_timeouts(env);
        if sender == self.user_a {
            self.account_a.fund()?;
            self.account_b.set_lock(lock);
        } else if sender == self.user_b {
            self.account_b.fund()?;
            self.account_a.set_lock(lock);
        } else {
            return Err(EscrowError::UnknownUser)
        } 
        return Ok({});
    }

    /// Set the sender's UserAction to Approved
    /// Returns an AccountError if this is an invalid state transition
    /// GUARDED by the counterparty's secret such that
    /// a user can only approve its own Account if it is in posession of the counterparty's 
    /// secret
    pub fn approve(&mut self, env: Env, sender: Addr, secret: &str) -> Result<(), EscrowError> {
        self.check_timeouts(env);
        if sender == self.user_a {
            self.account_a.approve(secret)?;
        } else if sender == self.user_b {
            self.account_b.approve(secret)?;
        } else {
            return Err(EscrowError::UnknownUser)
        }
        return Ok({});
    }

    /// Set the sender's account UserAction to Cancelled
    /// Returns an AccountError if this is an invalid state transition
    pub fn cancel(&mut self, env: Env, sender: Addr) -> Result<(), EscrowError> {
        self.check_timeouts(env);
        if sender == self.user_a {
            self.account_a.cancel()?;
        } else if sender == self.user_b {
            self.account_b.cancel()?;
        } else {
            return Err(EscrowError::UnknownUser)
        }
        return Ok({});
    }

    /// compute_payout calculates the withdrawal coefficients for both accounts based on
    /// the internal state of the escrow. The result is expressed in basis points.
    /// 
    ///       N            A         C         T1        T2
    /// N     X            X         X         X         X
    /// A     X        (100,100)  (130,70)     X      (150,50)  
    /// C     X        (70,130)   (50,50)   (100,0)   (50, 0)   
    /// T1    X            X      (0,100)    (0,0)    (0,100) 
    /// T2    X        (50,150)   (0,50)    (100,0)    (0,0) 
    /// 
    /// Ex: (100, 100) -> return 100% for both accounts
    ///     (130,  70) -> return 130% to user_a and 70% to user_b
    /// 
    /// Returns an EscrowError::InvalidWithdrawState if the escrow is not in a 
    /// withdrawable state
    pub fn compute_payout(&mut self, env: Env) -> Result<Payout, EscrowError> {
        if let Some(res) = self.payout {
            return Ok(res);
        }

        self.check_timeouts(env);
        
        let res; 
        match (&self.account_a.action, &self.account_b.action) {
            (UserAction::Approved, UserAction::Approved) => {
                res = Payout{user_a_basis_points:100, user_b_basis_points:100};
            },
            (UserAction::Approved, UserAction::Cancelled) => {
                res = Payout{user_a_basis_points:130, user_b_basis_points:70};
            },
            (UserAction::Approved, UserAction::T2) => {
                res =Payout{user_a_basis_points:150, user_b_basis_points:50};
            },
            (UserAction::Cancelled, UserAction::Approved) => {
                res =Payout{user_a_basis_points:70, user_b_basis_points:130};
            },
            (UserAction::Cancelled, UserAction::Cancelled) => {
                res =Payout{user_a_basis_points:50, user_b_basis_points:50};
            },
            (UserAction::Cancelled, UserAction::T1) => {
                res =Payout{user_a_basis_points:100, user_b_basis_points:0};
            },
            (UserAction::Cancelled, UserAction::T2) => {
                res =Payout{user_a_basis_points:50, user_b_basis_points:0};
            },
            (UserAction::T1, UserAction::Cancelled) => {
                res =Payout{user_a_basis_points:0, user_b_basis_points:100};
            },
            (UserAction::T1, UserAction::T1) => {
                res =Payout{user_a_basis_points:0, user_b_basis_points:0};
            },
            (UserAction::T1, UserAction::T2) => {
                res =Payout{user_a_basis_points:0, user_b_basis_points:100};
            },
            (UserAction::T2, UserAction::Approved) => {
                res =Payout{user_a_basis_points:50, user_b_basis_points:150};
            },
            (UserAction::T2, UserAction::Cancelled) => {
                res =Payout{user_a_basis_points:0, user_b_basis_points:50};
            },
            (UserAction::T2, UserAction::T1) => {
                res =Payout{user_a_basis_points:100, user_b_basis_points:0};
            },
            (UserAction::T2, UserAction::T2) => {
                res =Payout{user_a_basis_points:0, user_b_basis_points:0};
            },
            _ => return Err(EscrowError::InvalidWithdrawState { msg: format!("[{},{}]", self.account_a.action, self.account_b.action) })
        }

        self.payout = Some(res);

        return Ok(res);
    }

    /// close sets the closed flag to true 
    /// we can only close if the payout has already been computed
    /// which indireclty ensures that the escrow is in a closeable 
    /// state
    pub fn close(&mut self) {
        if let Some(_) = self.payout {
            self.closed = true;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use cosmwasm_std::testing::mock_env;
    use cosmwasm_std::{Uint128};
    use cw20::{Cw20CoinVerified};

    use crate::account::{AccountStatus, UserAction};
    use crate::error::AccountError;
  
    const  DUMMY_LOCK_A: &str = "0330347c5cb0f1627bdd2e7b082504a443b2bf50ad2e3efbb4e754ebd687c78c24";
    const  DUMMY_SECRET_A: &str =  "27874aa2b70ce7281c94413c36d44fac6fa6a1198f2c529188c4dd4f7a4e1870";   

    const DUMMY_LOCK_B: &str = "032d5f7beb52d336163483804facb17c47033fb14dfc3f3c88235141bae1896fc8";
    const DUMMY_SECRET_B: &str =  "cde73ee8f8584c54ac455c941f75990f4bff47a4340023e3fd236344e9a7d4ea";   

    const DUMMY_T1: u64 = 1000;
    const DUMMY_T2: u64 = 4000;

    fn dummy_env(current_timestamp: u64) -> Env {
        let mut env = mock_env();
        env.block.time = Timestamp::from_seconds(current_timestamp);
        return env;
    }

    fn dummy_escrow(current_timestamp: u64) -> Result<Escrow, EscrowError> {
        let coin = Cw20CoinVerified {
            address: Addr::unchecked("coin_address"),
            amount: Uint128::new(100),
        };

        return Escrow::create(
            &dummy_env(current_timestamp),
            Addr::unchecked("user_a"),
            Addr::unchecked("user_b"),
            DUMMY_T1,
            DUMMY_T2,
            Balance::Cw20(coin),
        );
    }

    #[test]
    fn escrow_create() {
        // t < t1 < t2
        let e = dummy_escrow(1).unwrap();
        assert_eq!(e.account_a.status, AccountStatus::Init);
        assert_eq!(e.account_a.action, UserAction::None);
        assert_eq!(e.account_b.status, AccountStatus::Init);
        assert_eq!(e.account_b.action, UserAction::None);

        // t1 < t < t2
        let err = dummy_escrow(2000).unwrap_err();
        assert!(matches!(err, EscrowError::InvalidTimeouts{}));

        // t1 < t2 < t
        let err = dummy_escrow(5000).unwrap_err();
        assert!(matches!(err, EscrowError::InvalidTimeouts{}));
    }

    #[test]
    fn escrow_fund() {
        let mut e = dummy_escrow(1).unwrap();

        // not user_a or user_b
        let err = e.fund(
            dummy_env(10),
            Addr::unchecked("bad"),
            DUMMY_LOCK_A,
        ).unwrap_err();
        assert!(matches!(err, EscrowError::UnknownUser));

        // fund account_a before T1, check that user_b's lock is set.
        let _ = e.fund(
            dummy_env(10),
            Addr::unchecked("user_a"),
             DUMMY_LOCK_A,
        ).unwrap();
        assert_eq!(e.account_a.status, AccountStatus::Funded);
        assert_eq!(e.account_a.action, UserAction::None);
        assert_eq!(e.account_b.status, AccountStatus::Init);
        assert_eq!(e.account_b.action, UserAction::None);
        assert_eq!(e.account_b.lock, Some(DUMMY_LOCK_A.to_string()));

        // fund account_b before T1, check that user_b's lock is set.
        let _ = e.fund(
            dummy_env(20),
             Addr::unchecked("user_b"),
              DUMMY_LOCK_B,
        ).unwrap();
        assert_eq!(e.account_a.status, AccountStatus::Funded);
        assert_eq!(e.account_a.action, UserAction::None);
        assert_eq!(e.account_b.status, AccountStatus::Funded);
        assert_eq!(e.account_b.action, UserAction::None);
        assert_eq!(e.account_a.lock, Some(DUMMY_LOCK_B.to_string()));

        // can't call twice, can't override the lock
        let err = e.fund(
            dummy_env(30),
            Addr::unchecked("user_a"),
            "hacker lock",
        ).unwrap_err();
        assert!(matches!(err, EscrowError::Account(AccountError::InvalidState { action:_, state:_ })));
        assert_eq!(e.account_a.status, AccountStatus::Funded);
        assert_eq!(e.account_a.action, UserAction::None);
        assert_eq!(e.account_b.status, AccountStatus::Funded);
        assert_eq!(e.account_b.action, UserAction::None);
        assert_eq!(e.account_b.lock, Some(DUMMY_LOCK_A.to_string()));

        let err = e.fund(
            dummy_env(40),
            Addr::unchecked("user_b"),
             "hacker lock",
        ).unwrap_err();
        assert!(matches!(err, EscrowError::Account(AccountError::InvalidState { action:_, state:_ })));
        assert_eq!(e.account_a.status, AccountStatus::Funded);
        assert_eq!(e.account_a.action, UserAction::None);
        assert_eq!(e.account_b.status, AccountStatus::Funded);
        assert_eq!(e.account_b.action, UserAction::None);
        assert_eq!(e.account_a.lock, Some(DUMMY_LOCK_B.to_string()));
    }

    #[test]
    fn escrow_approve() {
        let mut e = dummy_escrow(1).unwrap();

        // fund both accounts before attempting to approve
        let _ = e.fund(dummy_env(10), Addr::unchecked("user_a"), DUMMY_LOCK_A).unwrap();
        let _ = e.fund(dummy_env(10), Addr::unchecked("user_b"), DUMMY_LOCK_B).unwrap();

        // test that only user_a and user_b can approve
        let err = e.approve(
            dummy_env(20),
            Addr::unchecked("bad"),
            DUMMY_SECRET_A,
        ).unwrap_err();
        assert!(matches!(err, EscrowError::UnknownUser));

        // test that user_a can approve its own account using b's secret
        let _ = e.approve(
            dummy_env(20),
            Addr::unchecked("user_a"),
            DUMMY_SECRET_B,
        ).unwrap();
        assert_eq!(e.account_a.status, AccountStatus::Funded);
        assert_eq!(e.account_a.action, UserAction::Approved);
        assert_eq!(e.account_b.status, AccountStatus::Funded);
        assert_eq!(e.account_b.action, UserAction::None);

        // test that user_b can approve its own account using a's secret
        let _ = e.approve(
            dummy_env(20),
            Addr::unchecked("user_b"),
             DUMMY_SECRET_A,
        ).unwrap();
        assert_eq!(e.account_a.status, AccountStatus::Funded);
        assert_eq!(e.account_a.action, UserAction::Approved);
        assert_eq!(e.account_b.status, AccountStatus::Funded);
        assert_eq!(e.account_b.action, UserAction::Approved);

    }

    #[test]
    fn escrow_cancel() {
        let mut e = dummy_escrow(1).unwrap();

        // test unknown sender
        let err = e.cancel(
            dummy_env(10),
            Addr::unchecked("bad"),
        ).unwrap_err();
        assert!(matches!(err, EscrowError::UnknownUser));

        // fund accounts before cancelling
        let _ = e.fund(dummy_env(10), Addr::unchecked("user_a"), DUMMY_LOCK_A).unwrap();
        let _ = e.fund(dummy_env(10), Addr::unchecked("user_b"), DUMMY_LOCK_B).unwrap();

        // test user_a cancel
        let _ = e.cancel(
            dummy_env(20),
            Addr::unchecked("user_a"),
        ).unwrap();
        assert_eq!(e.account_a.status, AccountStatus::Funded);
        assert_eq!(e.account_a.action, UserAction::Cancelled);
        assert_eq!(e.account_b.status, AccountStatus::Funded);
        assert_eq!(e.account_b.action, UserAction::None);

          // test user_b cancel
        let _ = e.cancel(
            dummy_env(20),
            Addr::unchecked("user_b"),
        ).unwrap();
        assert_eq!(e.account_a.status, AccountStatus::Funded);
        assert_eq!(e.account_a.action, UserAction::Cancelled);
        assert_eq!(e.account_b.status, AccountStatus::Funded);
        assert_eq!(e.account_b.action, UserAction::Cancelled);
    }

    struct TestCase {
        name: String,
        expected: Payout,
        init:  fn() -> Escrow,
        withdraw_timestamp: u64,
    }

    fn get_cases() -> Vec<TestCase> {
        let res = vec![
            TestCase{
                // both users fund the account before T1
                // both users approve other's account before T2
                // call withdraw AFTER these events (it is irrelevant whether it is before or after the timeouts, so long as it
                // is after the approvals)
                name: "(APPROVED, APPROVED)".to_string(),
                expected: Payout{user_a_basis_points:100, user_b_basis_points:100},
                init:  || {
                    let mut escrow = dummy_escrow(1).unwrap();
                    let _ = escrow.fund(dummy_env(10), Addr::unchecked("user_a"), DUMMY_LOCK_A).unwrap();
                    let _ = escrow.fund(dummy_env(10), Addr::unchecked("user_b"), DUMMY_LOCK_B).unwrap();
                    let _ = escrow.approve(dummy_env(20), Addr::unchecked("user_a"), DUMMY_SECRET_B).unwrap();
                    let _ = escrow.approve(dummy_env(20), Addr::unchecked("user_b"), DUMMY_SECRET_A).unwrap();
                    return escrow;
                },
                withdraw_timestamp: DUMMY_T2 -10,
            },
            TestCase{
                // both users fund their accounts before T1
                // user_a approves before T2
                // user_b cancels before T2
                // withdraw after user actions
                name: "(APPROVED, CANCELLED)".to_string(),
                expected: Payout{user_a_basis_points:130, user_b_basis_points:70},
                init:  || {
                    let mut escrow = dummy_escrow(1).unwrap();
                    let _ = escrow.fund(dummy_env(10), Addr::unchecked("user_a"), DUMMY_LOCK_A).unwrap();
                    let _ = escrow.fund(dummy_env(10), Addr::unchecked("user_b"), DUMMY_LOCK_B).unwrap();
                    let _ = escrow.approve(dummy_env(20), Addr::unchecked("user_a"), DUMMY_SECRET_B).unwrap();
                    let _ = escrow.cancel(dummy_env(20), Addr::unchecked("user_b")).unwrap();
                    return escrow;
                },
                withdraw_timestamp: DUMMY_T2 - 10,
            },
            TestCase{
                // both users fund before T1
                // user_a approves before T2
                // user_b does nothing
                // call withdraw after T2 -> account_b should transition to T2
                name: "(APPROVED, T2)".to_string(),
                expected: Payout{user_a_basis_points:150, user_b_basis_points:50},
                init:  || {
                    let mut escrow = dummy_escrow(1).unwrap();
                    let _ = escrow.fund(dummy_env(10), Addr::unchecked("user_a"), DUMMY_LOCK_A).unwrap();
                    let _ = escrow.fund(dummy_env(10), Addr::unchecked("user_b"), DUMMY_LOCK_B).unwrap();
                    let _ = escrow.approve(dummy_env(20), Addr::unchecked("user_a"), DUMMY_SECRET_B).unwrap();
                    return escrow;
                },
                withdraw_timestamp: DUMMY_T2 + 10,
            },
            TestCase{
                // both users fund before T1
                // user_a cancels before T2
                // user_b approves before T2
                name: "(CANCELLED, APPROVED)".to_string(),
                expected: Payout{user_a_basis_points:70, user_b_basis_points:130},
                init:  || {
                    let mut escrow = dummy_escrow(1).unwrap();
                    let _ = escrow.fund(dummy_env(10), Addr::unchecked("user_a"), DUMMY_LOCK_A).unwrap();
                    let _ = escrow.fund(dummy_env(10), Addr::unchecked("user_b"), DUMMY_LOCK_B).unwrap();
                    let _ = escrow.cancel(dummy_env(20),Addr::unchecked("user_a")).unwrap();
                    let _ = escrow.approve(dummy_env(20), Addr::unchecked("user_b"), DUMMY_SECRET_A).unwrap();
                    return escrow;
                },
                withdraw_timestamp: DUMMY_T2 - 10,
            },
            TestCase{
                // both users fund before T1
                // both users cancel before T2
                name: "(CANCELLED, CANCELLED)".to_string(),
                expected: Payout{user_a_basis_points:50, user_b_basis_points:50},
                init:  || {
                    let mut escrow = dummy_escrow(1).unwrap();
                    let _ = escrow.fund(dummy_env(10), Addr::unchecked("user_a"), DUMMY_LOCK_A).unwrap();
                    let _ = escrow.fund(dummy_env(10), Addr::unchecked("user_b"), DUMMY_LOCK_B).unwrap();
                    let _ = escrow.cancel(dummy_env(20), Addr::unchecked("user_a")).unwrap();
                    let _ = escrow.cancel(dummy_env(20), Addr::unchecked("user_b")).unwrap();
                    return escrow;
                },
                withdraw_timestamp: DUMMY_T2 - 10,
            },
            TestCase{
                // user_a funds before T1
                // user_b tries to fund after T1
                // user_a cancels before T2
                name: "(CANCELLED, T1)".to_string(),
                expected: Payout{user_a_basis_points:100, user_b_basis_points:0},
                init:  || {
                    let mut escrow = dummy_escrow(1).unwrap();
                    let _ = escrow.fund(dummy_env(10), Addr::unchecked("user_a"), DUMMY_LOCK_A).unwrap();
                    let _ = escrow.fund(dummy_env(DUMMY_T1 + 10), Addr::unchecked("user_b"), DUMMY_LOCK_B).unwrap_err();
                    let _ = escrow.cancel(dummy_env(20), Addr::unchecked("user_a")).unwrap();
                    return escrow;
                },
                withdraw_timestamp: DUMMY_T2,
            },
            TestCase{
                // both users fund before T1
                // user_a cancels before T2
                // user_b does nothing
                // call withdraw after T2 -> account_b should transition to T2
                name: "(CANCELLED, T2)".to_string(),
                expected: Payout{user_a_basis_points:50, user_b_basis_points:0},
                init:  || {
                    let mut escrow = dummy_escrow(1).unwrap();
                    let _ = escrow.fund(dummy_env(10), Addr::unchecked("user_a"), DUMMY_LOCK_A).unwrap();
                    let _ = escrow.fund(dummy_env(10), Addr::unchecked("user_b"), DUMMY_LOCK_B).unwrap();
                    let _ = escrow.cancel(dummy_env(20), Addr::unchecked("user_a")).unwrap();
                    return escrow;
                },
                withdraw_timestamp: DUMMY_T2 + 10,
            },
            TestCase{
                // user_b funds before T1
                // user_a tries to fund after T1
                // user_b cancels before T2
                name: "(T1, CANCELLED)".to_string(),
                expected: Payout{user_a_basis_points:0, user_b_basis_points:100},
                init:  || {
                    let mut escrow = dummy_escrow(1).unwrap();
                    let _ = escrow.fund(dummy_env(10), Addr::unchecked("user_b"), DUMMY_LOCK_B);
                    let _ = escrow.fund(dummy_env(DUMMY_T1 +10), Addr::unchecked("user_a"), DUMMY_LOCK_A).unwrap_err();
                    let _ = escrow.cancel(dummy_env(20), Addr::unchecked("user_b")).unwrap();
                    return escrow;
                },
                withdraw_timestamp: DUMMY_T2,
            },
            TestCase{
                // both users do not fund before T1
                // withdrawing after T1 causes both accounts to transition to T1
                name: "(T1, T1)".to_string(),
                expected: Payout{user_a_basis_points:0, user_b_basis_points:0},
                init:  || {
                    let escrow = dummy_escrow(1).unwrap();
                    return escrow;
                },
                withdraw_timestamp: DUMMY_T1 + 10,
            },
            TestCase{
                // user_b funds before T1
                // user_a tries to fund after T1, causing it to transition to T1
                // neither user does anything before T2
                // calling withdraw after T2 causes account_b to transition to T2
                name: "(T1, T2)".to_string(),
                expected: Payout{user_a_basis_points:0, user_b_basis_points:100},
                init:  || {
                    let mut escrow = dummy_escrow(1).unwrap();
                    let _ = escrow.fund(dummy_env(10),Addr::unchecked("user_b"), DUMMY_LOCK_B).unwrap();
                    let _ = escrow.fund(dummy_env(DUMMY_T1+10),Addr::unchecked("user_a"), DUMMY_LOCK_A).unwrap_err();
                    return escrow;
                },
                withdraw_timestamp: DUMMY_T2 + 10,
            },
            TestCase{
                // both users fund before T1
                // user_b approves before T2
                // user_a does nothing before T2
                // calling withdraw after T2 causes account_a to transition to T2
                name: "(T2, APPROVED)".to_string(),
                expected: Payout{user_a_basis_points:50, user_b_basis_points:150},
                init:  || {
                    let mut escrow = dummy_escrow(1).unwrap();
                    let _ = escrow.fund(dummy_env(10),Addr::unchecked("user_a"), DUMMY_LOCK_A).unwrap();
                    let _ = escrow.fund(dummy_env(10), Addr::unchecked("user_b"), DUMMY_LOCK_A).unwrap();
                    let _ = escrow.approve(dummy_env(20), Addr::unchecked("user_b"), DUMMY_SECRET_A).unwrap();
                    return escrow;
                },
                withdraw_timestamp: DUMMY_T2 + 10,
            },
            TestCase{
                // both users fund before T1
                // user_b cancels before T2
                // user_a does nothing before T2
                // calling withdraw after T2 causes account_a to transition to T2
                name: "(T2, CANCELLED)".to_string(),
                expected: Payout{user_a_basis_points:0, user_b_basis_points:50},
                init:  || {
                    let mut escrow = dummy_escrow(1).unwrap();
                    let _ = escrow.fund(dummy_env(10), Addr::unchecked("user_a"), DUMMY_LOCK_A).unwrap();
                    let _ = escrow.fund(dummy_env(10), Addr::unchecked("user_b"), DUMMY_LOCK_B).unwrap();
                    let _ = escrow.cancel(dummy_env(20), Addr::unchecked("user_b")).unwrap();
                    return escrow;
                },
                withdraw_timestamp: DUMMY_T2 + 10,
            },
            TestCase{
                // user_a funds before T1
                // user_a does nothing before T2
                // user_b does nothing at all
                // calling withdraw after T2 causes account_a to transition to T2, and account_b to T1
                name: "(T2, T1)".to_string(),
                expected: Payout{user_a_basis_points:100, user_b_basis_points:0},
                init:  || {
                    let mut escrow = dummy_escrow(1).unwrap();
                    let _ = escrow.fund(dummy_env(10),Addr::unchecked("user_a"), DUMMY_LOCK_A).unwrap();
                    return escrow;
                },
                withdraw_timestamp: DUMMY_T2 + 10,
            },
            TestCase{
                // both users fund before T1
                // neither user does anything before T2
                // calling withdraw after T2 causes both accounts to transition to T2
                name: "(T2, T2)".to_string(),
                expected: Payout{user_a_basis_points:0, user_b_basis_points:0},
                init:  || {
                    let mut escrow = dummy_escrow(1).unwrap();
                    let _ = escrow.fund(dummy_env(10),Addr::unchecked("user_a"), DUMMY_LOCK_A).unwrap();
                    let _ = escrow.fund(dummy_env(10),Addr::unchecked("user_b"), DUMMY_LOCK_B).unwrap();
                    return escrow;
                },
                withdraw_timestamp: DUMMY_T2 + 10,
            },
        ];
        return res;
    }

    #[test]
    fn escrow_withdraw() {
        let mut test_cases = get_cases();

        for tc in test_cases.iter_mut() {
            let mut e = (tc.init)();
            let res = e.compute_payout(dummy_env(tc.withdraw_timestamp));
            assert_eq!(tc.expected, res.unwrap(), "{}", tc.name);
            assert_eq!(tc.expected, e.payout.unwrap(), "{}", tc.name);
        }
    }
}