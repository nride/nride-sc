#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Deps, DepsMut, Env, MessageInfo, Addr};
use cosmwasm_std::{Response, StdResult };
use cosmwasm_std::{Binary, to_binary};
use cosmwasm_std::Order;
use cw2::{set_contract_version, get_contract_version};
use semver::Version;

use crate::error::ContractError;
use crate::msg::{ InstantiateMsg, MigrateMsg, ExecuteMsg, SubscribeMsg, QueryMsg };
use crate::state::{Record, records};

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
pub fn migrate(
    deps: DepsMut,
    _env: Env,
    _msg: MigrateMsg,
) -> Result<Response, ContractError> {
    let version: Version = CONTRACT_VERSION.parse()?;
    let storage_version: Version = get_contract_version(deps.storage)?.version.parse()?;
    if storage_version < version {
        set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
        // If state structure changed in any contract version in the way migration is needed, it
        // should occur here
    }
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

    records().save(
            deps.storage,
            sender,
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
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::List { location } => to_binary(&query_list(deps, vec![location])?),
        QueryMsg::ListMultiple {locations} => to_binary(&query_list(deps, locations)?), 
        QueryMsg::Details { address } =>to_binary(&query_details(deps, address)?),
    }
}

fn query_details(deps: Deps, address: String) -> StdResult<Record> {
    let addr = deps.api.addr_validate(&address)?;
    let record = records().load(deps.storage, &addr)?;
    Ok(record)
}

fn query_list(deps: Deps, locations: Vec<String>) -> StdResult<Vec<Record>> {
    let mut res = Vec::new();
    for loc in locations.iter() {
        let loc_items = records()
            .idx
            .location
            .prefix(loc.clone())
            .range(deps.storage, None, None, Order::Ascending)
            .map(|r| r.map(|(_,v)| v))
            .collect::<StdResult<Vec<Record>>>()?;
        res.extend(loc_items);
    }
    Ok(res)
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};

    use super::*;

    fn get_instantiate_msg() -> (MessageInfo, InstantiateMsg) {
        let instantiate_msg = InstantiateMsg {};
        let info = mock_info(&String::from("anyone"), &[]);
        return (info, instantiate_msg);
    }

    #[test]
    fn add_and_update() {
        let mut deps = mock_dependencies();

        // instantiate an empty contract
        let (info, instantiate_msg) = get_instantiate_msg();
        let res = instantiate(deps.as_mut(), mock_env(), info, instantiate_msg).unwrap();
        assert_eq!(0, res.messages.len());

        let subscribe_msg = SubscribeMsg{
            nkn_addr: "colosseo".to_string(),
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
        assert_eq!(("nkn_addr", "colosseo"), res.attributes[2]);
        assert_eq!(("location", "roma"), res.attributes[3]);

        let details = query_details(
            deps.as_ref(),
            "alice".to_string(),
        ).unwrap();
        assert_eq!(
            details,
            Record{
                reg_addr: Addr::unchecked("alice"),
                nkn_addr: "colosseo".to_string(),
                location: "roma".to_string(),
            },
        );

        let subscribe_msg_2 = SubscribeMsg{
            nkn_addr: "piccadilly".to_string(),
            location: "london".to_string(),
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
        assert_eq!(("nkn_addr", "piccadilly"), res.attributes[2]);
        assert_eq!(("location", "london"), res.attributes[3]);

        let details = query_details(
            deps.as_ref(),
            "alice".to_string(),
        ).unwrap();
        assert_eq!(
            details,
            Record{
                reg_addr: Addr::unchecked("alice"),
                nkn_addr: "piccadilly".to_string(),
                location: "london".to_string(),
            },
        );
    }

    #[test]
    fn by_location() {
        let mut deps = mock_dependencies();

        // instantiate an empty contract
        let (info, instantiate_msg) = get_instantiate_msg();
        let res = instantiate(deps.as_mut(), mock_env(), info, instantiate_msg).unwrap();
        assert_eq!(0, res.messages.len());

        execute(
            deps.as_mut(),
             mock_env(),
             mock_info("alice",  &[]),
             ExecuteMsg::Subscribe(SubscribeMsg{
                nkn_addr: "colosseo".to_string(),
                location: "roma".to_string(),
            }),
        ).unwrap();
        execute(
            deps.as_mut(),
             mock_env(),
             mock_info("bob",  &[]),
             ExecuteMsg::Subscribe(SubscribeMsg{
                nkn_addr: "trastevere".to_string(),
                location: "roma".to_string(),
            }),
        ).unwrap();
        execute(
            deps.as_mut(),
             mock_env(),
             mock_info("charlie",  &[]),
             ExecuteMsg::Subscribe(SubscribeMsg{
                nkn_addr: "piccadilly".to_string(),
                location: "london".to_string(),
            }),
        ).unwrap();
        execute(
            deps.as_mut(),
             mock_env(),
             mock_info("dennis",  &[]),
             ExecuteMsg::Subscribe(SubscribeMsg{
                nkn_addr: "bastille".to_string(),
                location: "paris".to_string(),
            }),
        ).unwrap();

        let records = query_list(
            deps.as_ref(),
            vec!["roma".to_string(), "paris".to_string()],
        ).unwrap();
        assert_eq!(records.len(), 3);
        assert_eq!(
            records,
            vec![
                Record{
                    reg_addr: Addr::unchecked("alice"),
                    nkn_addr: "colosseo".to_string(),
                    location: "roma".to_string(),
                },
                Record{
                    reg_addr: Addr::unchecked("bob"),
                    nkn_addr: "trastevere".to_string(),
                    location: "roma".to_string(),
                },
                Record{
                    reg_addr: Addr::unchecked("dennis"),
                    nkn_addr: "bastille".to_string(),
                    location: "paris".to_string(),
                },
            ],
        ); 
    }
}
