#[cfg(not(feature = "library"))]

use cosmwasm_std::entry_point;
use cosmwasm_std::{
    from_binary, to_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response,
    StdResult, SubMsg, WasmMsg, BankMsg, StdError,
};

use cw2::set_contract_version;
use cw20::{Balance, Cw20CoinVerified, Cw20ExecuteMsg, Cw20ReceiveMsg};

use crate::error::ContractError;
use crate::msg::{
    InstantiateMsg,
    CreateMsg, 
    ExecuteMsg,
    WithdrawMsg,
    ReceiveMsg,
    QueryMsg,
    ListResponse,
    DetailsResponse
};

use crate::escrow::Escrow;
use crate::state::{all_escrow_ids, ESCROWS};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cw-nride-escrow";
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
        ExecuteMsg::Create(msg)=>execute_create(deps, env, msg, Balance::from(info.funds), &info.sender),
        ExecuteMsg::Withdraw(msg)  => execute_withdraw(deps, env, msg),
        ExecuteMsg::Cancel{id} => execute_cancel(deps, env, id, &info.sender),
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
    }
}

pub fn execute_create(
    deps: DepsMut,
    _env: Env,
    msg: CreateMsg,
    balance: Balance,
    sender: &Addr,
) -> Result<Response, ContractError> {
  
    let user_b_addr = deps.api.addr_validate(&msg.user_b)?;

    let escrow = Escrow::create(
        sender.clone(),
        user_b_addr,
        balance,
        &msg.lock,
    )?;

    // try to store it, fail if the id was already in use
    ESCROWS.update(deps.storage, &msg.id, |existing| match existing {
        None => Ok(escrow),
        Some(_) => Err(ContractError::AlreadyInUse {}),
    })?;

    let res = Response::new().add_attributes(vec![("action", "create"), ("id", msg.id.as_str())]);
    Ok(res)
}


pub fn execute_withdraw(
    deps: DepsMut,
    _env: Env,
    msg: WithdrawMsg,
) -> Result<Response, ContractError> {
    // this fails if no escrow there
    let mut escrow = ESCROWS.load(deps.storage, &msg.id)?;

    if escrow.closed {
        return Err(ContractError::Closed {  });
    }
        
    escrow.unlock(&msg.secret)?;

    escrow.close();
    
    ESCROWS.save(deps.storage, &msg.id, &escrow)?;

    let payments = create_payment_submsgs(escrow.deposit, escrow.user_b).unwrap();
    
    let res = Response::new().add_attributes(vec![
        ("action", "withdraw"),
        ("id", &msg.id.as_str()),
    ]).add_submessages(payments);
        
    Ok(res)
}

pub fn execute_cancel(
    deps: DepsMut,
    _env: Env,
    id: String,
    sender: &Addr,
) -> Result<Response, ContractError> {
    // this fails is no escrow there
    let mut escrow = ESCROWS.load(deps.storage, &id)?;

    if escrow.closed {
        return Err(ContractError::Closed {  });
    }

    if sender != escrow.user_a {
        return Err(ContractError::InvalidUser { });
    }

    escrow.close();
    
    ESCROWS.save(deps.storage, &id, &escrow)?;

    let payments = create_payment_submsgs(escrow.deposit, escrow.user_a).unwrap();
    
    let res = Response::new().add_attributes(vec![
        ("action", "cancel"),
        ("id", &id.as_str()),
    ]).add_submessages(payments);
        
    Ok(res)
}

pub fn create_payment_submsgs(deposit: Balance, recipient: Addr) -> StdResult<Vec<SubMsg>> {
    let mut msgs: Vec<SubMsg> = vec![];
    
    match deposit {
        Balance::Cw20(token) => {
            let payoff_msg = Cw20ExecuteMsg::Transfer {
                recipient: recipient.to_string(),
                amount:  token.amount,
            };
            let payoff_exec = SubMsg::new(WasmMsg::Execute {
                contract_addr: token.address.to_string(),
                msg: to_binary(&payoff_msg)?,
                funds: vec![],
            });
            msgs.push(payoff_exec);   
        }
        Balance::Native(native_balance) => {
            let payoff_msg = SubMsg::new(BankMsg::Send {
                to_address: recipient.to_string(),
                amount: native_balance.into_vec(),
            });
            msgs.push(payoff_msg);
        }
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

    let details = DetailsResponse {
        id,
        user_a: escrow.user_a.to_string(),
        user_b: escrow.user_b.to_string(),
        deposit: escrow.deposit,
        lock: escrow.lock,
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
    use cosmwasm_std::Uint128;

    use super::*;

    const ESCROW_ID: &str ="foobar";
    const USER_A_ADDR: &str= "user_a";  
    const USER_B_ADDR : &str = "user_b";
    const REQUIRED_TOKEN_ADDR: &str = "the_cw20_token";
    const REQUIRED_TOKEN_AMOUNT: u128 =  100;
    const LOCK_A: &str = "0330347c5cb0f1627bdd2e7b082504a443b2bf50ad2e3efbb4e754ebd687c78c24";
    const SECRET_A: &str =  "27874aa2b70ce7281c94413c36d44fac6fa6a1198f2c529188c4dd4f7a4e1870"; 

    fn get_instantiate_msg() -> (MessageInfo, InstantiateMsg) {
        let instantiate_msg = InstantiateMsg {};
        let info = mock_info(&String::from("anyone"), &[]);
        return (info, instantiate_msg);
    }

    fn get_create_msg(
        sender_addr: String,
        escrow_id: String,
        user_b_addr: String,
        lock: String,
        required_token_addr: String,
        required_token_amount: u128 ) -> (MessageInfo, ExecuteMsg){

        let create = CreateMsg {
            id: escrow_id,
            user_b: user_b_addr,
            lock: lock,
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

    fn get_withdraw_msg(
        sender_addr: String,
        escrow_id: String,
        secret: String,
    ) -> (MessageInfo, ExecuteMsg) {
        let withdraw = WithdrawMsg {
            id: escrow_id,
            secret: secret,
        };
        let info = mock_info(&sender_addr,  &[]);
        let msg = ExecuteMsg::Withdraw(withdraw.clone());
        return (info, msg);
    }

    fn get_cancel_msg(
        sender_addr: String,
        escrow_id: String,
    ) -> (MessageInfo, ExecuteMsg) {
        let info = mock_info(&sender_addr, &[]);
        let msg = ExecuteMsg::Cancel{id: escrow_id};
        return (info, msg);
    }

    #[test]
    fn withdraw_happy() {
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
            REQUIRED_TOKEN_ADDR.to_string(),
            REQUIRED_TOKEN_AMOUNT,
        );

        let res = execute(deps.as_mut(), mock_env(), info, create_msg).unwrap();
        assert_eq!(0, res.messages.len());
        assert_eq!(("action", "create"), res.attributes[0]);

        // ensure the escrow is in the expected state
        let details = query_details(deps.as_ref(), ESCROW_ID.to_string()).unwrap();
        assert_eq!(
            details,
            DetailsResponse {
                id: ESCROW_ID.to_string(),
                user_a: USER_A_ADDR.to_string(),
                user_b: USER_B_ADDR.to_string(),
                deposit: Balance::Cw20(
                    Cw20CoinVerified{
                        address:Addr::unchecked(REQUIRED_TOKEN_ADDR),
                        amount: Uint128::new(REQUIRED_TOKEN_AMOUNT),
                    },
                ),
                lock: LOCK_A.to_string(),
                closed: false,
            }
        );

        // withdraw
        let (info, withdraw_msg) = get_withdraw_msg(
            USER_B_ADDR.to_string(),
            ESCROW_ID.to_string(),
            SECRET_A.to_string(),
        );  
        let res = execute(deps.as_mut(), mock_env(), info.clone(), withdraw_msg.clone()).unwrap();
        assert_eq!(1, res.messages.len());
        assert_eq!(("action", "withdraw"), res.attributes[0]);
        assert_eq!(("id", ESCROW_ID.to_string()), res.attributes[1]);
    
        // ensure the escrow is closed
        let details = query_details(deps.as_ref(), ESCROW_ID.to_string()).unwrap();
        assert_eq!(
            details,
            DetailsResponse {
                id: ESCROW_ID.to_string(),
                user_a: USER_A_ADDR.to_string(),
                user_b: USER_B_ADDR.to_string(),
                deposit: Balance::Cw20(
                    Cw20CoinVerified{
                        address:Addr::unchecked(REQUIRED_TOKEN_ADDR),
                        amount: Uint128::new(REQUIRED_TOKEN_AMOUNT),
                    },
                ),
                lock: LOCK_A.to_string(),
                closed: true,
            }
        );

        // withdraw when escrow closed
        let err = execute(deps.as_mut(), mock_env(), info.clone(), withdraw_msg.clone()).unwrap_err();
        assert!(matches!(err, ContractError::Closed{}));
    }

    #[test]
    fn cancel_happy() {
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
            REQUIRED_TOKEN_ADDR.to_string(),
            REQUIRED_TOKEN_AMOUNT,
        );
        let _ = execute(deps.as_mut(), mock_env(), info, create_msg).unwrap();
     
        // cancel
        let (info, cancel_msg) = get_cancel_msg(
            USER_A_ADDR.to_string(),
            ESCROW_ID.to_string(),
        );  
        let res = execute(deps.as_mut(), mock_env(), info.clone(), cancel_msg.clone()).unwrap();
        assert_eq!(1, res.messages.len());
        assert_eq!(("action", "cancel"), res.attributes[0]);
        assert_eq!(("id", ESCROW_ID.to_string()), res.attributes[1]);
    
        // ensure the escrow is closed
        let details = query_details(deps.as_ref(), ESCROW_ID.to_string()).unwrap();
        assert_eq!(
            details,
            DetailsResponse {
                id: ESCROW_ID.to_string(),
                user_a: USER_A_ADDR.to_string(),
                user_b: USER_B_ADDR.to_string(),
                deposit: Balance::Cw20(
                    Cw20CoinVerified{
                        address:Addr::unchecked(REQUIRED_TOKEN_ADDR),
                        amount: Uint128::new(REQUIRED_TOKEN_AMOUNT),
                    },
                ),
                lock: LOCK_A.to_string(),
                closed: true,
            }
        );

         // cancel when escrow closed
         let err = execute(deps.as_mut(), mock_env(), info.clone(), cancel_msg.clone()).unwrap_err();
         assert!(matches!(err, ContractError::Closed{}));
    }

    #[test]
    fn cancel_wrong_user() {
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
            REQUIRED_TOKEN_ADDR.to_string(),
            REQUIRED_TOKEN_AMOUNT,
        );
        let _ = execute(deps.as_mut(), mock_env(), info, create_msg).unwrap();
     
        // cancel
        let (info, cancel_msg) = get_cancel_msg(
            USER_B_ADDR.to_string(),
            ESCROW_ID.to_string(),
        );  
        let err = execute(deps.as_mut(), mock_env(), info.clone(), cancel_msg.clone()).unwrap_err();
         assert!(matches!(err, ContractError::InvalidUser{}));
        
        // ensure the escrow is still open
        let details = query_details(deps.as_ref(), ESCROW_ID.to_string()).unwrap();
        assert_eq!(
            details,
            DetailsResponse {
                id: ESCROW_ID.to_string(),
                user_a: USER_A_ADDR.to_string(),
                user_b: USER_B_ADDR.to_string(),
                deposit: Balance::Cw20(
                    Cw20CoinVerified{
                        address:Addr::unchecked(REQUIRED_TOKEN_ADDR),
                        amount: Uint128::new(REQUIRED_TOKEN_AMOUNT),
                    },
                ),
                lock: LOCK_A.to_string(),
                closed: false,
            }
        );
    }

}