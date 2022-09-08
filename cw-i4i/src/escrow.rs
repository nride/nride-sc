use cosmwasm_std::{Addr, StdError };
use cw20::{Balance};

use crate::{account::{Account, UserAction, AccountStatus}, error::AccountError};

#[derive(Clone, PartialEq,Debug)]
pub struct WithdrawResult {
    user_a_basis_points: u8,
    user_b_basis_points: u8,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Escrow {
    /// user_a creates the escrow
    pub user_a: Addr,
    /// user_b is the counterparty
    pub user_b: Addr,
    /// t2_timeout (in seconds since epoch 00:00:00 UTC on 1 January 1970).
    ///  when the block time exceeds this value, the escrow is expired.
    pub t2_timeout: u64,
    /// token amount that each side (user_a and user_b) is expected to put in
    pub amount: Balance,
    /// account_a is the account state-machine for user_a
    pub account_a: Account,
    /// account_a is the account state-machine for user_a
    pub account_b: Account,
}

impl Escrow {
    pub fn create(
        user_a: Addr,
        user_b: Addr,
        t2_timeout: u64,
        amount: Balance,
    ) -> Self {
        // TODO: check timeout is in the future
        // TODO: Check amount is positive
        // TODO: handle transfer? (fund account a)???
        Escrow{
            user_a,
            user_b,
            t2_timeout,
            amount,
            account_a: Account::new(),
            account_b: Account::new(),
        }
    }

    /// Set the sender's account AccountStatus to Funded
    /// Returns an AccountError if this is an invalid state transition
    /// Also sets the lock on the counterparty's account
    pub fn fund(&mut self, sender: Addr, lock: &str) -> Result<(), AccountError> {
        if sender == self.user_a {
            let res = self.account_a.fund();
            if res.is_ok() {
                self.account_b.set_lock(lock);
            }
            return res;

        }
        if sender == self.user_b {
            let res = self.account_b.fund();
            if res.is_ok() {
            self.account_a.set_lock(lock);
            }
            return res;
        }
        Err(AccountError::Std(StdError::NotFound {kind:"sender is not user_a or user_b".to_string()}))
    }

    /// Set the sender's UserAction to Approved
    /// Returns an AccountError if this is an invalid state transition
    /// GUARDED by the counterparty's secret such that
    /// a user can only approve its own Account if it is in posession of the counterparty's 
    /// secret
    pub fn approve(&mut self, sender: Addr, secret: &str) -> Result<(), AccountError> {
        if sender == self.user_a {
            return self.account_a.approve(secret);
        }
        if sender == self.user_b {
            return self.account_b.approve(secret);
        }
        Err(AccountError::Std(StdError::NotFound {kind:"sender is not user_a or user_b".to_string()}))
    }

    /// Set the sender's account UserAction to Cancelled
    /// Returns an AccountError if this is an invalid state transition
    pub fn cancel(&mut self, sender: Addr) -> Result<(), AccountError> {
        if sender == self.user_a {
            return self.account_a.cancel();
        }
        if sender == self.user_b {
            return self.account_b.cancel();
        }
        Err(AccountError::Std(StdError::NotFound {kind:"sender is not user_a or user_b".to_string()}))
    }

    /// Set account's UserAction to T1
    /// Returns an AccountError if this is an invalid state transition
    pub fn t1(&mut self, account: Addr) -> Result<(), AccountError> {
        if account == self.user_a {
            return self.account_a.t1();
        }
        if account == self.user_b {
            return self.account_b.t1();
        }
        Err(AccountError::Std(StdError::NotFound {kind:"sender is not user_a or user_b".to_string()}))   
    }

    /// Set account's UserAction to T2
    /// Returns an AccountError if this is an invalid state transition
    pub fn t2(&mut self, account: Addr) -> Result<(), AccountError> {
        if account == self.user_a {
            return self.account_a.t2();
        }
        if account == self.user_b {
            return self.account_b.t2();
        }
        Err(AccountError::Std(StdError::NotFound {kind:"sender is not user_a or user_b".to_string()}))   
    }

    /// Withdraw calculates the withdrawal coefficients for both accounts based on
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
    /// Returns an StdError if the escrow is not in a withdrawable state
    pub fn withdraw(&mut self) -> Result<WithdrawResult, StdError> {
       // ATTENTION: Add T1 State where one of the accounts is not funded before T1 timeout 

        match (&self.account_a.action, &self.account_b.action) {
            (UserAction::Approved, UserAction::Approved) => {
                Ok(WithdrawResult{user_a_basis_points:100, user_b_basis_points:100})
            },
            (UserAction::Approved, UserAction::Cancelled) => {
                Ok(WithdrawResult{user_a_basis_points:130, user_b_basis_points:70})
            },
            (UserAction::Approved, UserAction::T2) => {
                Ok(WithdrawResult{user_a_basis_points:150, user_b_basis_points:50})
            },
            (UserAction::Cancelled, UserAction::Approved) => {
                Ok(WithdrawResult{user_a_basis_points:70, user_b_basis_points:130})
            },
            (UserAction::Cancelled, UserAction::Cancelled) => {
                Ok(WithdrawResult{user_a_basis_points:50, user_b_basis_points:50})
            },
            (UserAction::Cancelled, UserAction::T1) => {
                Ok(WithdrawResult{user_a_basis_points:100, user_b_basis_points:0})
            },
            (UserAction::Cancelled, UserAction::T2) => {
                Ok(WithdrawResult{user_a_basis_points:50, user_b_basis_points:0})
            },
            (UserAction::T1, UserAction::Cancelled) => {
                Ok(WithdrawResult{user_a_basis_points:0, user_b_basis_points:100})
            },
            (UserAction::T1, UserAction::T1) => {
                Ok(WithdrawResult{user_a_basis_points:0, user_b_basis_points:0})
            },
            (UserAction::T1, UserAction::T2) => {
                Ok(WithdrawResult{user_a_basis_points:0, user_b_basis_points:100})
            },
            (UserAction::T2, UserAction::Approved) => {
                Ok(WithdrawResult{user_a_basis_points:50, user_b_basis_points:150})
            },
            (UserAction::T2, UserAction::Cancelled) => {
                Ok(WithdrawResult{user_a_basis_points:0, user_b_basis_points:50})
            },
            (UserAction::T2, UserAction::T1) => {
                Ok(WithdrawResult{user_a_basis_points:100, user_b_basis_points:0})
            },
            (UserAction::T2, UserAction::T2) => {
                Ok(WithdrawResult{user_a_basis_points:0, user_b_basis_points:0})
            },
            _ => Err(StdError::GenericErr{msg:"invalid state".to_string()})
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::account::{AccountStatus, UserAction};
    use cw20::{Cw20CoinVerified};
    
    use cosmwasm_std::{Uint128};

    static DUMMY_LOCK_A: &str = "04b4ac68eff3a82d86db5f0489d66f91707e99943bf796ae6a2dcb2205c9522fa7915428b5ac3d3b9291e62142e7246d85ad54504fabbdb2bae5795161f8ddf259";
    static DUMMY_SECRET_A: &str =  "3c9229289a6125f7fdf1885a77bb12c37a8d3b4962d936f7e3084dece32a3ca1";   

    static DUMMY_LOCK_B: &str = "042d5f7beb52d336163483804facb17c47033fb14dfc3f3c88235141bae1896fc8d99a685aafaf92d5f41d866fe387b988a998590326f1b549878b9d03eabed7e5";
    static DUMMY_SECRET_B: &str =  "cde73ee8f8584c54ac455c941f75990f4bff47a4340023e3fd236344e9a7d4ea";   

    fn dummy_escrow() -> Escrow {
        let coin = Cw20CoinVerified {
            address: Addr::unchecked("coin_address"),
            amount: Uint128::new(100),
        };
        let e = Escrow::create(
            Addr::unchecked("user_a"),
            Addr::unchecked("user_b"),
            123,
            Balance::Cw20(coin),
        );
        return e;
    }

    #[test]
    fn escrow_create() {
        let e = dummy_escrow();
        assert_eq!(e.account_a.status, AccountStatus::Init);
        assert_eq!(e.account_a.action, UserAction::None);
        assert_eq!(e.account_b.status, AccountStatus::Init);
        assert_eq!(e.account_b.action, UserAction::None);
    }

    #[test]
    fn escrow_fund() {
        let mut e = dummy_escrow();

        let err = e.fund(Addr::unchecked("bad"), DUMMY_LOCK_A).unwrap_err();
        assert!(matches!(err, AccountError::Std(_)));

        let _ = e.fund(Addr::unchecked("user_a"), DUMMY_LOCK_A).unwrap();
        assert_eq!(e.account_a.status, AccountStatus::Funded);
        assert_eq!(e.account_a.action, UserAction::None);
        assert_eq!(e.account_b.status, AccountStatus::Init);
        assert_eq!(e.account_b.action, UserAction::None);
        assert_eq!(e.account_b.lock, Some(DUMMY_LOCK_A.to_string()));

        let _ = e.fund(Addr::unchecked("user_b"), DUMMY_LOCK_B).unwrap();
        assert_eq!(e.account_a.status, AccountStatus::Funded);
        assert_eq!(e.account_a.action, UserAction::None);
        assert_eq!(e.account_b.status, AccountStatus::Funded);
        assert_eq!(e.account_b.action, UserAction::None);
        assert_eq!(e.account_a.lock, Some(DUMMY_LOCK_B.to_string()));


        let err = e.fund(Addr::unchecked("user_a"), "hacker lock").unwrap_err();
        assert!(matches!(err, AccountError::InvalidState {  }));
        assert_eq!(e.account_a.status, AccountStatus::Funded);
        assert_eq!(e.account_a.action, UserAction::None);
        assert_eq!(e.account_b.status, AccountStatus::Funded);
        assert_eq!(e.account_b.action, UserAction::None);
        assert_eq!(e.account_b.lock, Some(DUMMY_LOCK_A.to_string()));

        let err = e.fund(Addr::unchecked("user_b"), "hacker lock").unwrap_err();
        assert!(matches!(err, AccountError::InvalidState {  }));
        assert_eq!(e.account_a.status, AccountStatus::Funded);
        assert_eq!(e.account_a.action, UserAction::None);
        assert_eq!(e.account_b.status, AccountStatus::Funded);
        assert_eq!(e.account_b.action, UserAction::None);
        assert_eq!(e.account_a.lock, Some(DUMMY_LOCK_B.to_string()));
    }

    #[test]
    fn escrow_approve() {
        let mut e = dummy_escrow();

        // fund both accounts before attempting to approve
        let _ = e.fund(Addr::unchecked("user_a"), DUMMY_LOCK_A).unwrap();
        let _ = e.fund(Addr::unchecked("user_b"), DUMMY_LOCK_B).unwrap();

        // test that only user_a and user_b can approve
        let err = e.approve(Addr::unchecked("bad"), DUMMY_SECRET_A).unwrap_err();
        assert!(matches!(err, AccountError::Std(_)));

        // test that user_a can approve its own account using b's secret
        let _ = e.approve(Addr::unchecked("user_a"), DUMMY_SECRET_B).unwrap();
        assert_eq!(e.account_a.status, AccountStatus::Funded);
        assert_eq!(e.account_a.action, UserAction::Approved);
        assert_eq!(e.account_b.status, AccountStatus::Funded);
        assert_eq!(e.account_b.action, UserAction::None);

        // test that user_b can approve its own account using a's secret
        let _ = e.approve(Addr::unchecked("user_b"), DUMMY_SECRET_A).unwrap();
        assert_eq!(e.account_a.status, AccountStatus::Funded);
        assert_eq!(e.account_a.action, UserAction::Approved);
        assert_eq!(e.account_b.status, AccountStatus::Funded);
        assert_eq!(e.account_b.action, UserAction::Approved);

    }

    #[test]
    fn escrow_cancel() {
        let mut e = dummy_escrow();

        // test unknown sender
        let err = e.cancel(Addr::unchecked("bad")).unwrap_err();
        assert!(matches!(err, AccountError::Std(_)));

        // fund accounts before cancelling
        let _ = e.fund(Addr::unchecked("user_a"), DUMMY_LOCK_A).unwrap();
        let _ = e.fund(Addr::unchecked("user_b"), DUMMY_LOCK_B).unwrap();

        // test user_a cancel
        let _ = e.cancel(Addr::unchecked("user_a")).unwrap();
        assert_eq!(e.account_a.status, AccountStatus::Funded);
        assert_eq!(e.account_a.action, UserAction::Cancelled);
        assert_eq!(e.account_b.status, AccountStatus::Funded);
        assert_eq!(e.account_b.action, UserAction::None);

          // test user_b cancel
          let _ = e.cancel(Addr::unchecked("user_b")).unwrap();
          assert_eq!(e.account_a.status, AccountStatus::Funded);
          assert_eq!(e.account_a.action, UserAction::Cancelled);
          assert_eq!(e.account_b.status, AccountStatus::Funded);
          assert_eq!(e.account_b.action, UserAction::Cancelled);
    }

    #[test]
    fn escrow_t2() {
        let mut e = dummy_escrow();

        // fund accounts before timing out
        let _ = e.fund(Addr::unchecked("user_a"), DUMMY_LOCK_A).unwrap();
        let _ = e.fund(Addr::unchecked("user_b"), DUMMY_LOCK_B).unwrap();

        // account a
        let _ = e.t2( Addr::unchecked("user_a")).unwrap();
        assert_eq!(e.account_a.status, AccountStatus::Funded);
        assert_eq!(e.account_a.action, UserAction::T2);
        assert_eq!(e.account_b.status, AccountStatus::Funded);
        assert_eq!(e.account_b.action, UserAction::None);
        
        // account b
        let _ = e.t2( Addr::unchecked("user_b")).unwrap();
        assert_eq!(e.account_a.status, AccountStatus::Funded);
        assert_eq!(e.account_a.action, UserAction::T2);
        assert_eq!(e.account_b.status, AccountStatus::Funded);
        assert_eq!(e.account_b.action, UserAction::T2);
    }

    struct TestCase {
        name: String,
        expected: Result<WithdrawResult, StdError>,
        init:  fn() -> Escrow,
    }

    fn get_happy_cases() -> Vec<TestCase> {
        let res = vec![
            TestCase{
                name: "(APPROVED, APPROVED)".to_string(),
                expected: Ok(WithdrawResult{user_a_basis_points:100, user_b_basis_points:100}),
                init:  || {
                    let mut escrow = dummy_escrow();
                    let _ = escrow.fund(Addr::unchecked("user_a"), DUMMY_LOCK_A).unwrap();
                    let _ = escrow.fund(Addr::unchecked("user_b"), DUMMY_LOCK_B).unwrap();
                    let _ = escrow.approve(Addr::unchecked("user_a"), DUMMY_SECRET_B).unwrap();
                    let _ = escrow.approve(Addr::unchecked("user_b"), DUMMY_SECRET_A).unwrap();
                    return escrow;
                },
            },
            TestCase{
                name: "(APPROVED, CANCELLED)".to_string(),
                expected: Ok(WithdrawResult{user_a_basis_points:130, user_b_basis_points:70}),
                init:  || {
                    let mut escrow = dummy_escrow();
                    let _ = escrow.fund(Addr::unchecked("user_a"), DUMMY_LOCK_A).unwrap();
                    let _ = escrow.fund(Addr::unchecked("user_b"), DUMMY_LOCK_B).unwrap();
                    let _ = escrow.approve(Addr::unchecked("user_a"), DUMMY_SECRET_B).unwrap();
                    let _ = escrow.cancel(Addr::unchecked("user_b")).unwrap();
                    return escrow;
                },
            },
            TestCase{
                name: "(APPROVED, T2)".to_string(),
                expected: Ok(WithdrawResult{user_a_basis_points:150, user_b_basis_points:50}),
                init:  || {
                    let mut escrow = dummy_escrow();
                    let _ = escrow.fund(Addr::unchecked("user_a"), DUMMY_LOCK_A).unwrap();
                    let _ = escrow.fund(Addr::unchecked("user_b"), DUMMY_LOCK_B).unwrap();
                    let _ = escrow.approve(Addr::unchecked("user_a"), DUMMY_SECRET_B).unwrap();
                    let _ = escrow.t2(Addr::unchecked("user_b")).unwrap();
                    return escrow;
                },
            },
            TestCase{
                name: "(CANCELLED, APPROVED)".to_string(),
                expected: Ok(WithdrawResult{user_a_basis_points:70, user_b_basis_points:130}),
                init:  || {
                    let mut escrow = dummy_escrow();
                    let _ = escrow.fund(Addr::unchecked("user_a"), DUMMY_LOCK_A).unwrap();
                    let _ = escrow.fund(Addr::unchecked("user_b"), DUMMY_LOCK_B).unwrap();
                    let _ = escrow.approve(Addr::unchecked("user_b"), DUMMY_SECRET_A).unwrap();
                    let _ = escrow.cancel(Addr::unchecked("user_a")).unwrap();
                    return escrow;
                },
            },
            TestCase{
                name: "(CANCELLED, CANCELLED)".to_string(),
                expected: Ok(WithdrawResult{user_a_basis_points:50, user_b_basis_points:50}),
                init:  || {
                    let mut escrow = dummy_escrow();
                    let _ = escrow.fund(Addr::unchecked("user_a"), DUMMY_LOCK_A).unwrap();
                    let _ = escrow.fund(Addr::unchecked("user_b"), DUMMY_LOCK_B).unwrap();
                    let _ = escrow.cancel(Addr::unchecked("user_a")).unwrap();
                    let _ = escrow.cancel(Addr::unchecked("user_b")).unwrap();
                    return escrow;
                },
            },
            TestCase{
                name: "(CANCELLED, T1)".to_string(),
                expected: Ok(WithdrawResult{user_a_basis_points:100, user_b_basis_points:0}),
                init:  || {
                    let mut escrow = dummy_escrow();
                    let _ = escrow.fund(Addr::unchecked("user_a"), DUMMY_LOCK_A).unwrap();
                    let _ = escrow.cancel(Addr::unchecked("user_a")).unwrap();
                    let _ = escrow.t1(Addr::unchecked("user_b")).unwrap();
                    return escrow;
                },
            },
            TestCase{
                name: "(CANCELLED, T2)".to_string(),
                expected: Ok(WithdrawResult{user_a_basis_points:50, user_b_basis_points:0}),
                init:  || {
                    let mut escrow = dummy_escrow();
                    let _ = escrow.fund(Addr::unchecked("user_a"), DUMMY_LOCK_A).unwrap();
                    let _ = escrow.fund(Addr::unchecked("user_b"), DUMMY_LOCK_B).unwrap();
                    let _ = escrow.cancel(Addr::unchecked("user_a")).unwrap();
                    let _ = escrow.t2(Addr::unchecked("user_b")).unwrap();
                    return escrow;
                },
            },
            TestCase{
                name: "(T1, CANCELLED)".to_string(),
                expected: Ok(WithdrawResult{user_a_basis_points:0, user_b_basis_points:100}),
                init:  || {
                    let mut escrow = dummy_escrow();
                    let _ = escrow.fund(Addr::unchecked("user_b"), DUMMY_LOCK_A).unwrap();
                    let _ = escrow.t1(Addr::unchecked("user_a")).unwrap();
                    let _ = escrow.cancel(Addr::unchecked("user_b")).unwrap();
                    return escrow;
                },
            },
            TestCase{
                name: "(T1, T1)".to_string(),
                expected: Ok(WithdrawResult{user_a_basis_points:0, user_b_basis_points:0}),
                init:  || {
                    let mut escrow = dummy_escrow();
                    let _ = escrow.t1(Addr::unchecked("user_a")).unwrap();
                    let _ = escrow.t1(Addr::unchecked("user_b")).unwrap();
                    return escrow;
                },
            },
            TestCase{
                name: "(T1, T2)".to_string(),
                expected: Ok(WithdrawResult{user_a_basis_points:0, user_b_basis_points:100}),
                init:  || {
                    let mut escrow = dummy_escrow();
                    let _ = escrow.fund(Addr::unchecked("user_b"), DUMMY_LOCK_B).unwrap();
                    let _ = escrow.t1(Addr::unchecked("user_a")).unwrap();
                    let _ = escrow.t2(Addr::unchecked("user_b")).unwrap();
                    return escrow;
                },
            },
            TestCase{
                name: "(T2, APPROVED)".to_string(),
                expected: Ok(WithdrawResult{user_a_basis_points:50, user_b_basis_points:150}),
                init:  || {
                    let mut escrow = dummy_escrow();
                    let _ = escrow.fund(Addr::unchecked("user_a"), DUMMY_LOCK_A).unwrap();
                    let _ = escrow.fund(Addr::unchecked("user_b"), DUMMY_LOCK_A).unwrap();
                    let _ = escrow.approve(Addr::unchecked("user_b"), DUMMY_SECRET_A).unwrap();
                    let _ = escrow.t2(Addr::unchecked("user_a")).unwrap();
                    return escrow;
                },
            },
            TestCase{
                name: "(T2, CANCELLED)".to_string(),
                expected: Ok(WithdrawResult{user_a_basis_points:0, user_b_basis_points:50}),
                init:  || {
                    let mut escrow = dummy_escrow();
                    let _ = escrow.fund(Addr::unchecked("user_a"), DUMMY_LOCK_A).unwrap();
                    let _ = escrow.fund(Addr::unchecked("user_b"), DUMMY_LOCK_B).unwrap();
                    let _ = escrow.t2(Addr::unchecked("user_a")).unwrap();
                    let _ = escrow.cancel(Addr::unchecked("user_b")).unwrap();
                    return escrow;
                },
            },
            TestCase{
                name: "(T2, T1)".to_string(),
                expected: Ok(WithdrawResult{user_a_basis_points:100, user_b_basis_points:0}),
                init:  || {
                    let mut escrow = dummy_escrow();
                    let _ = escrow.fund(Addr::unchecked("user_a"), DUMMY_LOCK_A).unwrap();
                    let _ = escrow.t2(Addr::unchecked("user_a")).unwrap();
                    let _ = escrow.t1(Addr::unchecked("user_b")).unwrap();
                    return escrow;
                },
            },
            TestCase{
                name: "(T2, T2)".to_string(),
                expected: Ok(WithdrawResult{user_a_basis_points:0, user_b_basis_points:0}),
                init:  || {
                    let mut escrow = dummy_escrow();
                    let _ = escrow.fund(Addr::unchecked("user_a"), DUMMY_LOCK_A).unwrap();
                    let _ = escrow.fund(Addr::unchecked("user_b"), DUMMY_LOCK_B).unwrap();
                    let _ = escrow.t2(Addr::unchecked("user_a")).unwrap();
                    let _ = escrow.t2(Addr::unchecked("user_b")).unwrap();
                    return escrow;
                },
            },
        ];
        return res;
    }

    #[test]
    fn escrow_withdraw_happy() {
        let mut test_cases = get_happy_cases();

        for tc in test_cases.iter_mut() {
            let mut e = (tc.init)();
            let res = e.withdraw();
            assert_eq!(tc.expected, res, "{}", tc.name);
        }
    }

    fn get_unhappy_cases() -> Vec<TestCase> {
        let res = vec![
            TestCase{
                name: "(NONE, NONE)".to_string(),
                expected: Err(StdError::GenericErr{msg:"invalid state".to_string()}),
                init:  || {
                    let escrow = dummy_escrow();
                    return escrow;
                },
            },
            TestCase{
                name: "(CANCEL, NONE)".to_string(),
                expected: Err(StdError::GenericErr{msg:"invalid state".to_string()}),
                init:  || {
                    let mut escrow = dummy_escrow();
                    let _ = escrow.fund(Addr::unchecked("user_a"), DUMMY_LOCK_A).unwrap();
                    let _ = escrow.cancel(Addr::unchecked("user_a")).unwrap();
                    return escrow;
                },
            },
            TestCase{
                name: "(T1, NONE)".to_string(),
                expected: Err(StdError::GenericErr{msg:"invalid state".to_string()}),
                init:  || {
                    let mut escrow = dummy_escrow();
                    let _ = escrow.t1(Addr::unchecked("user_a")).unwrap();
                    return escrow;
                },
            },
            TestCase{
                name: "(T2, NONE)".to_string(),
                expected: Err(StdError::GenericErr{msg:"invalid state".to_string()}),
                init:  || {
                    let mut escrow = dummy_escrow();
                    let _ = escrow.fund(Addr::unchecked("user_a"), DUMMY_LOCK_A).unwrap();
                    let _ = escrow.t2(Addr::unchecked("user_a")).unwrap();
                    return escrow;
                },
            },
            TestCase{
                name: "(NONE, CANCEL)".to_string(),
                expected: Err(StdError::GenericErr{msg:"invalid state".to_string()}),
                init:  || {
                    let mut escrow = dummy_escrow();
                    let _ = escrow.fund(Addr::unchecked("user_b"), DUMMY_LOCK_B).unwrap();
                    let _ = escrow.cancel(Addr::unchecked("user_b")).unwrap();
                    return escrow;
                },
            },
            TestCase{
                name: "(NONE, T1)".to_string(),
                expected: Err(StdError::GenericErr{msg:"invalid state".to_string()}),
                init:  || {
                    let mut escrow = dummy_escrow();
                    let _ = escrow.t1(Addr::unchecked("user_b")).unwrap();
                    return escrow;
                },
            },
            TestCase{
                name: "(NONE, T2)".to_string(),
                expected: Err(StdError::GenericErr{msg:"invalid state".to_string()}),
                init:  || {
                    let mut escrow = dummy_escrow();
                    let _ = escrow.fund(Addr::unchecked("user_b"), DUMMY_LOCK_B).unwrap();
                    let _ = escrow.t2(Addr::unchecked("user_b")).unwrap();
                    return escrow;
                },
            },
        ];
        return res;
    }

    #[test]
    fn escrow_withdraw_unhappy() {
        let mut test_cases = get_unhappy_cases();

        for tc in test_cases.iter_mut() {
            let mut e = (tc.init)();
            let res = e.withdraw();
            assert_eq!(tc.expected, res,  "{}", tc.name);
        }
    }
}