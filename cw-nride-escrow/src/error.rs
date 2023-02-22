use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error(transparent)]
    Escrow(#[from] EscrowError),

    #[error(transparent)]
    Account(#[from] AccountError),

    #[error("Escrow id already in use")]
    AlreadyInUse {},

    #[error("Escrow is closed")]
    Closed {},

}

#[derive(Error, Debug, PartialEq)]
pub enum EscrowError {
    #[error(transparent)]
    Account(#[from] AccountError),

    #[error("Send some coins to create an escrow")]
    EmptyDeposit {},
    
    #[error("Match required deposit")]
    InvalidDeposit {},

    #[error("Invalid timeouts")]
    InvalidTimeouts,

    #[error("Sender is not user_a or user_b")]
    UnknownUser,
    
    #[error("The escrow is not in a withdrawable state {}", msg)]
    InvalidWithdrawState{msg: String}
}


#[derive(Error, Debug, PartialEq)]
pub enum AccountError {
    #[error("cannot call {} in state {}", action, state)]
    InvalidState {action: String, state: String},

    #[error("account lock is not set")]
    NoLock {},

    #[error("Invalid Secret")]
    InvalidSecret {}
}