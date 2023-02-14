#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Addr};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, SubscribeMsg};
use crate::state::{Record, RECORDS};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cw-nride-registry";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(_deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    // no setup
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match _msg {
        ExecuteMsg::Subscribe(msg) => {
            execute_subscribe(_deps, _env, msg, &_info.sender)
        }
    }
}

pub fn execute_subscribe(
    deps:DepsMut,
    _env: Env,
    msg: SubscribeMsg,
    sender: &Addr,
) -> Result<Response, ContractError> {
    
    let record = Record{
        reg_addr: sender.clone(),
        nkn_addr: msg.nkn_addr,
        location: msg.location,
    };

    RECORDS.save(
            deps.storage,
            sender.clone(),
            &record,    
    )?;

    let res = Response::new().add_attributes(vec![
        ("action", "subscribe"),
        ("reg_addr", record.reg_addr.as_str()),
        ("nkn_addr", record.nkn_addr.as_str()),
        ("location", record.location.as_str())]);

    Ok(res)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{ Uint128, Timestamp};

    use super::*;

    fn get_instantiate_msg() -> (MessageInfo, InstantiateMsg) {
        let instantiate_msg = InstantiateMsg {};
        let info = mock_info(&String::from("anyone"), &[]);
        return (info, instantiate_msg);
    }

    #[test]
    fn happy_path() {
        let mut deps = mock_dependencies();

        // instantiate an empty contract
        let (info, instantiate_msg) = get_instantiate_msg();
        let res = instantiate(deps.as_mut(), mock_env(), info, instantiate_msg).unwrap();
        assert_eq!(0, res.messages.len());

        let subscribe_msg = SubscribeMsg{
            nkn_addr: "caput mundi".to_string(),
            location: "roma".to_string(),
        };

        let execute_msg = ExecuteMsg::Subscribe(subscribe_msg);

        let res = execute(
            deps.as_mut(),
             mock_env(),
             mock_info("alice",  &[]),
            execute_msg,
        ).unwrap();
        assert_eq!(0, res.messages.len());
        assert_eq!(("action", "subscribe"), res.attributes[0]);
        assert_eq!(("reg_addr", "alice"), res.attributes[1]);
        assert_eq!(("nkn_addr", "caput mundi"), res.attributes[2]);
        assert_eq!(("location", "roma"), res.attributes[3]);

        let subscribe_msg_2 = SubscribeMsg{
            nkn_addr: "ville de l'amour".to_string(),
            location: "paris".to_string(),
        };

        let execute_msg_2 = ExecuteMsg::Subscribe(subscribe_msg_2);

        let res = execute(
            deps.as_mut(),
             mock_env(),
             mock_info("alice",  &[]),
            execute_msg_2,
        ).unwrap();
        assert_eq!(0, res.messages.len());
        assert_eq!(("action", "subscribe"), res.attributes[0]);
        assert_eq!(("reg_addr", "alice"), res.attributes[1]);
        assert_eq!(("nkn_addr", "ville de l'amour"), res.attributes[2]);
        assert_eq!(("location", "paris"), res.attributes[3]);
    }

}
