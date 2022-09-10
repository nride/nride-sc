#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    from_binary, to_binary, Addr, BankMsg, Binary, Deps, DepsMut, Env, MessageInfo, Response,
    StdResult, SubMsg, WasmMsg, StdError,
};

use cw2::set_contract_version;
use cw20::{Balance, Cw20Coin, Cw20CoinVerified, Cw20ExecuteMsg, Cw20ReceiveMsg};

use crate::error::ContractError;
use crate::new_msg::{
    CreateMsg, ExecuteMsg, InstantiateMsg, ReceiveMsg, QueryMsg, ListResponse, DetailsResponse, TopUpMsg,
};

use crate::escrow::Escrow;
use crate::new_state::{all_escrow_ids, ESCROWS};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cw20-i4i";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
const CW20_TOKEN_ADDR: &str = "xxxxxxxxxx";

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    // no setup
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Create(msg) => {
            execute_create(deps, env,  msg, Balance::from(info.funds), &info.sender)
        }
        ExecuteMsg::TopUp(msg) => {
            execute_topup(deps, env, msg, Balance::from(info.funds), &info.sender)
        }
        ExecuteMsg::Receive(msg) => execute_receive(deps, env, info, msg),
    }
}

pub fn execute_receive(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    wrapper: Cw20ReceiveMsg,
) -> Result<Response, ContractError> {
    let msg: ReceiveMsg = from_binary(&wrapper.msg)?;
    let balance = Balance::Cw20(Cw20CoinVerified {
        address: info.sender,
        amount: wrapper.amount,
    });
    let api = deps.api;
    match msg {
        ReceiveMsg::Create(msg) => {
            execute_create(deps, env,  msg, balance, &api.addr_validate(&wrapper.sender)?)
        },
        ReceiveMsg::TopUp(msg) => {
            execute_topup(deps, env, msg, balance, &api.addr_validate(&wrapper.sender)?)
        }
    }
}

pub fn execute_create(
    deps: DepsMut,
    env: Env,
    msg: CreateMsg,
    balance: Balance,
    sender: &Addr,
) -> Result<Response, ContractError> {
    if balance.is_empty() {
        return Err(ContractError::EmptyBalance {});
    }

    // TODO check balance matches whitelisted NRIDE token
    // transfer token (or is that automatic)

    let user_b_addr = deps.api.addr_validate(&msg.user_b);

    let escrow_res = Escrow::create(
        &env,
        sender.clone(),
        user_b_addr.unwrap(),
        msg.t1_timeout,
        msg.t2_timeout,
        balance,
    );
    if escrow_res.is_err() {
        return Err(ContractError::Expired {  }); // XXX proper error message
    }
    let mut escrow = escrow_res.unwrap();

    let fund_res = escrow.fund(env, sender.clone(), &msg.account_b_lock);
    if fund_res.is_err() {
        return Err(ContractError::InvalidState(fund_res.unwrap_err()));
    }

    // try to store it, fail if the id was already in use
    ESCROWS.update(deps.storage, &msg.id, |existing| match existing {
        None => Ok(escrow),
        Some(_) => Err(ContractError::AlreadyInUse {}),
    })?;

    let res = Response::new().add_attributes(vec![("action", "create"), ("id", msg.id.as_str())]);
    Ok(res)
}

pub fn execute_topup(
    deps: DepsMut,
    env: Env,
    msg: TopUpMsg,
    balance: Balance,
    sender: &Addr,
) -> Result<Response, ContractError> {
    if balance.is_empty() {
        return Err(ContractError::EmptyBalance {});
    }
    // this fails is no escrow there
    let mut escrow = ESCROWS.load(deps.storage, &msg.id)?;

    // check token
    if balance != escrow.required_deposit {
        return Err(ContractError::InvalidDeposit{  }); 
    }

    let fund_res = escrow.fund(env, sender.clone(), &msg.account_a_lock);
    if fund_res.is_err() {
        return Err(ContractError::InvalidState(fund_res.unwrap_err()));
    }

    // and save
    ESCROWS.save(deps.storage, &msg.id, &escrow)?;

    let res = Response::new().add_attributes(vec![("action", "top_up"), ("id", msg.id.as_str())]);
    Ok(res)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::List {} => to_binary(&query_list(deps)?),
        QueryMsg::Details { id } => to_binary(&query_details(deps, id)?),
    }
}

fn query_details(deps: Deps, id: String) -> StdResult<DetailsResponse> {
    let escrow = ESCROWS.load(deps.storage, &id)?;

    let details = DetailsResponse {
        id,
        user_a: escrow.user_a.to_string(),
        account_a_state: escrow.account_a.to_string(),
        account_a_lock:escrow.account_a.lock,
        user_b: escrow.user_b.to_string(),
        account_b_state: escrow.account_b.to_string(),
        account_b_lock:escrow.account_b.lock,
        t1_timeout: escrow.t1_timeout,
        t2_timeout: escrow.t2_timeout,
        required_deposit: escrow.required_deposit,
    };

    Ok(details)
}

fn query_list(deps: Deps) -> StdResult<ListResponse> {
    Ok(ListResponse {
        escrows: all_escrow_ids(deps.storage)?,
    })
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info, MockApi, MockQuerier, MockStorage};
    use cosmwasm_std::{attr, coin, coins, CosmosMsg, StdError, Uint128, OwnedDeps, Empty, Timestamp};

    use super::*;

     const ESCROW_ID: &str ="foobar";
     const USER_A_ADDR: &str= "user_a";
     const LOCK_A: &str = "xxxxxx";
     const USER_B_ADDR : &str = "user_b";
     const LOCK_B : &str= "yyyyyy";
     const T1_TIMEOUT: u64 = 1000000;
     const T2_TIMEOUT: u64 = 4000000;
     const REQUIRED_TOKEN_ADDR: &str = "the_cw20_token";
     const REQUIRED_TOKEN_AMOUNT: u128 =  100;

    fn get_mock_env(timestamp: u64) -> Env {
        let mut env = mock_env();
        env.block.time = Timestamp::from_seconds(timestamp);
        return env;
    }

    fn get_instantiate_msg() -> (MessageInfo, InstantiateMsg) {
        let instantiate_msg = InstantiateMsg {};
        let info = mock_info(&String::from("anyone"), &[]);
        return (info, instantiate_msg);
    }

    fn get_create_msg(
        sender_addr: String,
        escrow_id: String,
        user_b_addr: String,
        account_b_lock: String,
        t1_timeout: u64,
        t2_timeout: u64,
        required_token_addr: String,
        required_token_amount: u128 ) -> (MessageInfo, ExecuteMsg){

        let create = CreateMsg {
            id: escrow_id,
            user_b: user_b_addr,
            account_b_lock: account_b_lock,
            t1_timeout,
            t2_timeout,
        };
        let receive = Cw20ReceiveMsg {
            sender: sender_addr,
            amount: Uint128::new(required_token_amount),
            msg: to_binary(&ExecuteMsg::Create(create.clone())).unwrap(),
        };
        let info = mock_info(&required_token_addr, &[]);
        let msg = ExecuteMsg::Receive(receive.clone());
        return (info, msg);
    }

    fn get_topup_msg(
        sender_addr: String,
        escrow_id: String,
        account_a_lock: String,
        deposit_token_addr: String,
        deposit_token_amount: u128,
    ) -> (MessageInfo, ExecuteMsg) {
        
        let topup = TopUpMsg {
            id: escrow_id,
            account_a_lock: account_a_lock,
        };
        let receive = Cw20ReceiveMsg {
            sender: sender_addr,
            amount: Uint128::new(deposit_token_amount),
            msg: to_binary(&ExecuteMsg::TopUp(topup.clone())).unwrap(),
        };
        let info = mock_info(&deposit_token_addr, &[]);
        let msg = ExecuteMsg::Receive(receive.clone());
        return (info, msg);
    }

    #[test]
    fn happy_path() {
        let mut deps = mock_dependencies();

        // instantiate an empty contract
        let (info, instantiate_msg) = get_instantiate_msg();
        let res = instantiate(deps.as_mut(), mock_env(), info, instantiate_msg).unwrap();
        assert_eq!(0, res.messages.len());

        // create an escrow
        let (info, create_msg) = get_create_msg(
            USER_A_ADDR.to_string(),
            ESCROW_ID.to_string(),
            USER_B_ADDR.to_string(),
            LOCK_A.to_string(),
            T1_TIMEOUT,
            T2_TIMEOUT,
            REQUIRED_TOKEN_ADDR.to_string(),
            REQUIRED_TOKEN_AMOUNT,
        );

        let res = execute(deps.as_mut(), get_mock_env(1), info, create_msg).unwrap();
        assert_eq!(0, res.messages.len());
        assert_eq!(("action", "create"), res.attributes[0]);

        // ensure the escrow is in the expected state
        let details = query_details(deps.as_ref(), ESCROW_ID.to_string()).unwrap();
        assert_eq!(
            details,
            DetailsResponse {
                id: ESCROW_ID.to_string(),
                user_a: USER_A_ADDR.to_string(),
                account_a_state: "[FUNDED|NONE]".to_string(),
                account_a_lock: None,
                user_b: USER_B_ADDR.to_string(),
                account_b_state: "[INIT|NONE]".to_string(),
                account_b_lock: Some(LOCK_A.to_string()),
                t1_timeout: T1_TIMEOUT,
                t2_timeout: T2_TIMEOUT,
                required_deposit: Balance::Cw20(
                    Cw20CoinVerified{
                        address:Addr::unchecked(REQUIRED_TOKEN_ADDR),
                        amount: Uint128::new(REQUIRED_TOKEN_AMOUNT),
                    },
                ),
            }
        );

        // topup an escrow
        let (info, topup_msg) = get_topup_msg(
            USER_B_ADDR.to_string(),
            ESCROW_ID.to_string(),
            LOCK_B.to_string(),
            REQUIRED_TOKEN_ADDR.to_string(),
            REQUIRED_TOKEN_AMOUNT,
        );  
        let res = execute(deps.as_mut(), get_mock_env(2), info, topup_msg).unwrap();
        assert_eq!(0, res.messages.len());
        assert_eq!(("action", "top_up"), res.attributes[0]);

        // ensure the escrow is what we expect
        let details = query_details(deps.as_ref(), ESCROW_ID.to_string()).unwrap();
        assert_eq!(
            details,
            DetailsResponse {
                id: ESCROW_ID.to_string(),
                user_a: USER_A_ADDR.to_string(),
                account_a_state: "[FUNDED|NONE]".to_string(),
                account_a_lock: Some(LOCK_B.to_string()),
                user_b: USER_B_ADDR.to_string(),
                account_b_state: "[FUNDED|NONE]".to_string(),
                account_b_lock: Some(LOCK_A.to_string()),
                t1_timeout: T1_TIMEOUT,
                t2_timeout: T2_TIMEOUT,
                required_deposit: Balance::Cw20(
                    Cw20CoinVerified{
                        address:Addr::unchecked(REQUIRED_TOKEN_ADDR),
                        amount: Uint128::new(REQUIRED_TOKEN_AMOUNT),
                    },
                ),
            }
        );
    }

    #[test]
    fn top_up_wrong_deposit() {
        let mut deps = mock_dependencies();

        // instantiate an empty contract
        let (info, instantiate_msg) = get_instantiate_msg();
        let res = instantiate(deps.as_mut(), mock_env(), info, instantiate_msg).unwrap();
        assert_eq!(0, res.messages.len());

        // create an escrow
        let (info, create_msg) = get_create_msg(
            USER_A_ADDR.to_string(),
            ESCROW_ID.to_string(),
            USER_B_ADDR.to_string(),
            LOCK_A.to_string(),
            T1_TIMEOUT,
            T2_TIMEOUT,
            REQUIRED_TOKEN_ADDR.to_string(),
            REQUIRED_TOKEN_AMOUNT,
        );
        let res = execute(deps.as_mut(), get_mock_env(1), info, create_msg).unwrap();
        assert_eq!(0, res.messages.len());
        assert_eq!(("action", "create"), res.attributes[0]);

          // topup an escrow with different depost token
        let (info, topup_msg) = get_topup_msg(
            USER_B_ADDR.to_string(),
            ESCROW_ID.to_string(),
            LOCK_B.to_string(),
            "bad_token_addr".to_string(),
            REQUIRED_TOKEN_AMOUNT,
        );  
        let err = execute(deps.as_mut(), get_mock_env(2), info, topup_msg).unwrap_err();
        assert!(matches!(err, ContractError::InvalidDeposit {  }));

        // topup an escrow with different deposit amount
        let (info, topup_msg) = get_topup_msg(
            USER_B_ADDR.to_string(),
            ESCROW_ID.to_string(),
            LOCK_B.to_string(),
            REQUIRED_TOKEN_ADDR.to_string(),
            666,
        );  
        let err = execute(deps.as_mut(), get_mock_env(2), info, topup_msg).unwrap_err();
        assert!(matches!(err, ContractError::InvalidDeposit {  }));

    }
}