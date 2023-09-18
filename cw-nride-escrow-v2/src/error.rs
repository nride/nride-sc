use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error(transparent)]
    Escrow(#[from] EscrowError),

    #[error("Escrow id already in use")]
    AlreadyInUse {},

    #[error("Escrow is closed")]
    Closed {},

}

#[derive(Error, Debug, PartialEq)]
pub enum EscrowError {
    #[error("Send some coins to create an escrow")]
    EmptyDeposit {},
    
    #[error("Match required deposit")]
    InvalidDeposit {},

    #[error("account lock is not set")]
    NoLock {},

    #[error("Invalid Secret")]
    InvalidSecret {}
}