#[cfg(not(feature = "library"))]

use cosmwasm_std::entry_point;
use cosmwasm_std::{
    from_binary, to_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response,
    StdResult, SubMsg, WasmMsg, StdError,
};

use cw2::set_contract_version;
use cw20::{Balance, Cw20CoinVerified, Cw20ExecuteMsg, Cw20ReceiveMsg};

use crate::error::ContractError;
use crate::msg::{
    CreateMsg, ExecuteMsg, InstantiateMsg, ReceiveMsg, QueryMsg, ListResponse, DetailsResponse, TopUpMsg, ApproveMsg,
};

use crate::escrow::{Escrow, Payout};
use crate::state::{all_escrow_ids, ESCROWS};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cw20-i4i-escrow";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

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
        ExecuteMsg::Approve(msg) => {
            execute_approve(deps, env, msg, &info.sender)
        }
        ExecuteMsg::Cancel{id} => {
            execute_cancel(deps, env, id, &info.sender)
        }
        ExecuteMsg::Withdraw { id } => {
            execute_withdraw(deps, env, id)
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
        },
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

pub fn execute_approve(
    deps: DepsMut,
    env: Env,
    msg: ApproveMsg,
    sender: &Addr,
) -> Result<Response, ContractError> {
    // this fails is no escrow there
    let mut escrow = ESCROWS.load(deps.storage, &msg.id)?;

    let approve_res = escrow.approve(env, sender.clone(), &msg.secret);
    if approve_res.is_err() {
        return Err(ContractError::InvalidState(approve_res.unwrap_err()));
    }

    // and save
    ESCROWS.save(deps.storage, &msg.id, &escrow)?;

    let res = Response::new().add_attributes(vec![("action", "approve"), ("id", msg.id.as_str())]);
    Ok(res)
}

pub fn execute_cancel(
    deps: DepsMut,
    env: Env,
    id: String,
    sender: &Addr,
) -> Result<Response, ContractError> {
    // this fails is no escrow there
    let mut escrow = ESCROWS.load(deps.storage, &id)?;

    let cancel_res = escrow.cancel(env, sender.clone());
    if cancel_res.is_err() {
        return Err(ContractError::InvalidState(cancel_res.unwrap_err()));
    }

    // and save
    ESCROWS.save(deps.storage, &id, &escrow)?;

    let res = Response::new().add_attributes(vec![("action", "cancel"), ("id", id.as_str())]);
    Ok(res)
}

pub fn execute_withdraw(
    deps: DepsMut,
    env: Env,
    id: String,
) -> Result<Response, ContractError> {
    // this fails is no escrow there
    let mut escrow = ESCROWS.load(deps.storage, &id)?;

    if escrow.closed {
        return Err(ContractError::Std(StdError::GenericErr { msg: "escrow is already closed".to_string() }));
    }
    
    // compute payout returns an error if the escrow is not in a withdrawable state
    // so the function will panic here if this is the case
    let payout = escrow.compute_payout(env).unwrap();

    escrow.close();
    
    ESCROWS.save(deps.storage, &id, &escrow)?;

    let payments = create_payment_submsgs(&payout, escrow).unwrap();
    
    let res = Response::new().add_attributes(vec![
        ("action", "withdraw"),
            ("id", id.as_str()),
            ("res", format!("({},{})", &payout.user_a_basis_points, payout.user_b_basis_points).as_str()), 
            ])
        .add_submessages(payments);
        
    Ok(res)
}

pub fn create_payment_submsgs(coeffs: &Payout, escrow: Escrow) -> StdResult<Vec<SubMsg>> {
    let mut msgs: Vec<SubMsg> = vec![];
    if let Balance::Cw20(token) = escrow.required_deposit {
        let user_a_payoff_msg = Cw20ExecuteMsg::Transfer {
            recipient: escrow.user_a.to_string(),
            amount:  token.amount.multiply_ratio(
                coeffs.user_a_basis_points as u128,
                100 as u128,
            ),
        };
        let user_a_exec = SubMsg::new(WasmMsg::Execute {
            contract_addr: token.address.to_string(),
            msg: to_binary(&user_a_payoff_msg)?,
            funds: vec![],
        });
        msgs.push(user_a_exec);

        let user_b_payoff_msg = Cw20ExecuteMsg::Transfer {
            recipient: escrow.user_b.to_string(),
            amount:  token.amount.multiply_ratio(
                coeffs.user_b_basis_points as u128,
                100 as u128,
            ),
        };
        let user_b_exec = SubMsg::new(WasmMsg::Execute {
            contract_addr: token.address.to_string(),
            msg: to_binary(&user_b_payoff_msg)?,
            funds: vec![],
        });
        msgs.push(user_b_exec);
    } else {
        return Err(StdError::GenericErr { msg: "native tokens not supported".to_string() });
    }
    Ok(msgs)
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

    let payout_val = match escrow.payout {
        None => None,
        Some(payout) => Some(format!("({},{})", payout.user_a_basis_points, payout.user_b_basis_points)),
    };

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
        payout: payout_val,
        closed: escrow.closed,
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
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{ Uint128, Timestamp};

    use super::*;

    const ESCROW_ID: &str ="foobar";
    const USER_A_ADDR: &str= "user_a";
    const  LOCK_A: &str = "04b4ac68eff3a82d86db5f0489d66f91707e99943bf796ae6a2dcb2205c9522fa7915428b5ac3d3b9291e62142e7246d85ad54504fabbdb2bae5795161f8ddf259";
    const  SECRET_A: &str =  "3c9229289a6125f7fdf1885a77bb12c37a8d3b4962d936f7e3084dece32a3ca1";   
    const USER_B_ADDR : &str = "user_b";
    const LOCK_B: &str = "042d5f7beb52d336163483804facb17c47033fb14dfc3f3c88235141bae1896fc8d99a685aafaf92d5f41d866fe387b988a998590326f1b549878b9d03eabed7e5";
    const SECRET_B: &str =  "cde73ee8f8584c54ac455c941f75990f4bff47a4340023e3fd236344e9a7d4ea";  
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
            msg: to_binary(&ReceiveMsg::Create(create.clone())).unwrap(),
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
            msg: to_binary(&ReceiveMsg::TopUp(topup.clone())).unwrap(),
        };
        let info = mock_info(&deposit_token_addr, &[]);
        let msg = ExecuteMsg::Receive(receive.clone());
        return (info, msg);
    }

    fn get_approve_msg(
        sender_addr: String,
        escrow_id: String,
        secret: String,
    ) -> (MessageInfo, ExecuteMsg) {
        
        let approve = ApproveMsg{
            id: escrow_id,
            secret: secret,
        };
        let info = mock_info(&sender_addr,  &[]);
        let msg = ExecuteMsg::Approve(approve.clone());
            return (info, msg);
    }

    fn get_withdraw_msg(
        sender_addr: String,
        escrow_id: String,
    ) -> (MessageInfo, ExecuteMsg) {
        let info = mock_info(&sender_addr,  &[]);
        let msg = ExecuteMsg::Withdraw{id:escrow_id};
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
                payout: None,
                closed: false,
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
                payout: None,
                closed: false,
            }
        );

        // user_a approves
        let (info, approve_msg) = get_approve_msg(
            USER_A_ADDR.to_string(),
            ESCROW_ID.to_string(),
            SECRET_B.to_string(),
        );  
        let res = execute(deps.as_mut(), get_mock_env(3), info, approve_msg).unwrap();
        assert_eq!(0, res.messages.len());
        assert_eq!(("action", "approve"), res.attributes[0]);

        // ensure the escrow is what we expect
        let details = query_details(deps.as_ref(), ESCROW_ID.to_string()).unwrap();
        assert_eq!(
            details,
            DetailsResponse {
                id: ESCROW_ID.to_string(),
                user_a: USER_A_ADDR.to_string(),
                account_a_state: "[FUNDED|APPROVED]".to_string(),
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
                payout: None,
                closed: false,
            }
        );

        // user_b approves
        let (info, approve_msg) = get_approve_msg(
            USER_B_ADDR.to_string(),
            ESCROW_ID.to_string(),
            SECRET_A.to_string(),
        );  
        let res = execute(deps.as_mut(), get_mock_env(4), info, approve_msg).unwrap();
        assert_eq!(0, res.messages.len());
        assert_eq!(("action", "approve"), res.attributes[0]);

        // ensure the escrow is what we expect
        let details = query_details(deps.as_ref(), ESCROW_ID.to_string()).unwrap();
        assert_eq!(
            details,
            DetailsResponse {
                id: ESCROW_ID.to_string(),
                user_a: USER_A_ADDR.to_string(),
                account_a_state: "[FUNDED|APPROVED]".to_string(),
                account_a_lock: Some(LOCK_B.to_string()),
                user_b: USER_B_ADDR.to_string(),
                account_b_state: "[FUNDED|APPROVED]".to_string(),
                account_b_lock: Some(LOCK_A.to_string()),
                t1_timeout: T1_TIMEOUT,
                t2_timeout: T2_TIMEOUT,
                required_deposit: Balance::Cw20(
                    Cw20CoinVerified{
                        address:Addr::unchecked(REQUIRED_TOKEN_ADDR),
                        amount: Uint128::new(REQUIRED_TOKEN_AMOUNT),
                    },
                ),
                payout: None,
                closed: false,
            }
        );

        let (info, withdraw_msg) = get_withdraw_msg(
            USER_A_ADDR.to_string(),
            ESCROW_ID.to_string(),
        );  
        let res = execute(deps.as_mut(), get_mock_env(4), info, withdraw_msg).unwrap();
        assert_eq!(2, res.messages.len());
        assert_eq!(("action", "withdraw"), res.attributes[0]);
        assert_eq!(("res", "(100,100)"), res.attributes[2]);

        // ensure the escrow is what we expect
        let details = query_details(deps.as_ref(), ESCROW_ID.to_string()).unwrap();
        assert_eq!(
            details,
            DetailsResponse {
                id: ESCROW_ID.to_string(),
                user_a: USER_A_ADDR.to_string(),
                account_a_state: "[FUNDED|APPROVED]".to_string(),
                account_a_lock: Some(LOCK_B.to_string()),
                user_b: USER_B_ADDR.to_string(),
                account_b_state: "[FUNDED|APPROVED]".to_string(),
                account_b_lock: Some(LOCK_A.to_string()),
                t1_timeout: T1_TIMEOUT,
                t2_timeout: T2_TIMEOUT,
                required_deposit: Balance::Cw20(
                    Cw20CoinVerified{
                        address:Addr::unchecked(REQUIRED_TOKEN_ADDR),
                        amount: Uint128::new(REQUIRED_TOKEN_AMOUNT),
                    },
                ),
                payout: Some("(100,100)".to_string()),
                closed: true,
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