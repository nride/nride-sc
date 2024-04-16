#![cfg(test)]

use cw_utils::NativeBalance;

use cosmwasm_std::{coins, to_binary, Addr, Empty, Uint128};
use cw20::{Cw20Coin, Cw20Contract, Cw20ExecuteMsg, Balance, Cw20CoinVerified};
use cw_multi_test::{App, Contract, ContractWrapper, Executor};

use crate::msg::{CreateMsg, WithdrawMsg, DetailsResponse, ExecuteMsg, InstantiateMsg, QueryMsg, ReceiveMsg};

pub fn contract_escrow() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(
        crate::contract::execute,
        crate::contract::instantiate,
        crate::contract::query,
    );
    Box::new(contract)
}

pub fn contract_cw20() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(
        cw20_base::contract::execute,
        cw20_base::contract::instantiate,
        cw20_base::contract::query,
    );
    Box::new(contract)
}

#[test]
// receive native tokens and release upon approval
fn escrow_happy_path_native_tokens() {

    const T_ID: &str= "demo"; // escrow ID
    const ALICE: &str = "alice"; // alice Addr
    const BOB: &str  = "bob";   // bob Addr
    const ALICE_INIT_BAL: u128 = 5000; // alice initial balance in cw20 token
    const BOB_INIT_BAL: u128 = 5000; // bob initial balance in cw20 token
    
    // Escrow Params
    const ALICE_LOCK: &str = "0330347c5cb0f1627bdd2e7b082504a443b2bf50ad2e3efbb4e754ebd687c78c24";
    const ALICE_SECRET: &str  =  "27874aa2b70ce7281c94413c36d44fac6fa6a1198f2c529188c4dd4f7a4e1870";   
    const T_DEPOSIT_AMOUNT: u128 = 1200;

    /*********************************************************************************** 
    Initialize the context and deploy the Escrow contract from the "owner" account
    ************************************************************************************/

    // create the owner account to store and deploy the token and escrow contracts
    let owner = Addr::unchecked("owner");
    let init_funds = coins(2000, "btc");

    let mut router = App::new(|router, _, storage| {
        router
            .bank
            .init_balance(storage, &owner, init_funds.clone())
            .unwrap();
        router
            .bank
            .init_balance(storage, &Addr::unchecked(ALICE), init_funds.clone())
            .unwrap();
        router
            .bank
            .init_balance(storage, &Addr::unchecked(BOB), init_funds.clone())
            .unwrap();
    });

    assert_eq!(
        router.wrap()
            .query_balance(ALICE, "btc")
            .unwrap()
            .amount
            .u128(),
        2000,
    );

    assert_eq!(
        router.wrap()
            .query_balance(BOB, "btc")
            .unwrap()
            .amount
            .u128(),
        2000,
    );

    // set up escrow contract
    let escrow_id = router.store_code(contract_escrow());
    let escrow_addr = router
        .instantiate_contract(
            escrow_id,
            owner.clone(),
            &InstantiateMsg {},
            &[],
            "Escrow",
            None,
        )
        .unwrap();

    // create escrow with native token
    // prepare the inner escrow msg    
    let create_msg = ExecuteMsg::Create(CreateMsg {
        id: T_ID.to_string(),
        user_b: BOB.to_string(),
        lock: String::from(ALICE_LOCK),
    });

    // send the TX from alice's account    
    let res = router.execute_contract(
        Addr::unchecked(ALICE),
        escrow_addr.clone(),
        &create_msg,
        &coins(10, "btc"),
    )
    .unwrap();
    assert_eq!(2, res.events.len());

    assert_eq!(
        router.wrap()
            .query_balance(ALICE, "btc")
            .unwrap()
            .amount
            .u128(),
        1990,
    );

    assert_eq!(
        router.wrap()
            .query_balance(BOB, "btc")
            .unwrap()
            .amount
            .u128(),
        2000,
    );

    assert_eq!(
        router.wrap()
            .query_balance(escrow_addr.clone(), "btc")
            .unwrap()
            .amount
            .u128(),
        10,
    );

    // ensure escrow properly created
    let details: DetailsResponse = router
        .wrap()
        .query_wasm_smart(escrow_addr.clone(), &QueryMsg::Details { id: T_ID.to_string() })
        .unwrap();
    assert_eq!(
        details,
        DetailsResponse {
            id: T_ID.to_string(),
            user_a: ALICE.to_string(),
            user_b: BOB.to_string(),
            deposit:Balance::Native(NativeBalance(coins(10, "btc"))),
            lock: ALICE_LOCK.to_string(),
            closed: false,
        }
    );

    /************************************
     * bob calls the withdraw function
     ***********************************/

    // prepare withdraw msg
    let withdraw_msg = ExecuteMsg::Withdraw(WithdrawMsg {
        id: T_ID.to_string(),
        secret: ALICE_SECRET.to_string(),
    });
    // send the TX from bob's account
    _ = router
        .execute_contract(Addr::unchecked(BOB), escrow_addr.clone(), &withdraw_msg, &[])
        .unwrap();

    assert_eq!(
        router.wrap()
            .query_balance(ALICE, "btc")
            .unwrap()
            .amount
            .u128(),
        1990,
    );

    assert_eq!(
        router.wrap()
            .query_balance(BOB, "btc")
            .unwrap()
            .amount
            .u128(),
        2010,
    );

    assert_eq!(
        router.wrap()
            .query_balance(escrow_addr.clone(), "btc")
            .unwrap()
            .amount
            .u128(),
        0,
    );

    let details: DetailsResponse = router
        .wrap()
        .query_wasm_smart(escrow_addr.clone(), &QueryMsg::Details { id: T_ID.to_string() })
        .unwrap();
    assert_eq!(
        details,
        DetailsResponse {
            id: T_ID.to_string(),
            user_a: ALICE.to_string(),
            user_b: BOB.to_string(),
            deposit:Balance::Native(NativeBalance(coins(10, "btc"))),
            lock: ALICE_LOCK.to_string(),
            closed: true,
        }
    );    
}

#[test]
// receive cw20 tokens and release upon approval
fn escrow_happy_path_cw20_tokens() {

    const T_ID: &str= "demo"; // escrow ID
    const ALICE: &str = "alice"; // alice Addr
    const BOB: &str  = "bob";   // bob Addr
    const ALICE_INIT_BAL: u128 = 5000; // alice initial balance in cw20 token
    const BOB_INIT_BAL: u128 = 5000; // bob initial balance in cw20 token
    
    // Escrow Params
    const ALICE_LOCK: &str = "0330347c5cb0f1627bdd2e7b082504a443b2bf50ad2e3efbb4e754ebd687c78c24";
    const ALICE_SECRET: &str  =  "27874aa2b70ce7281c94413c36d44fac6fa6a1198f2c529188c4dd4f7a4e1870";   
    const T_DEPOSIT_AMOUNT: u128 = 1200;


    /*********************************************************************************** 
    Initialize the context and deploy CW20 and Escrow contracts from the "owner" account
    ************************************************************************************/

    // create the owner account to store and deploy the token and escrow contracts
    let owner = Addr::unchecked("owner");
    let init_funds = coins(2000, "btc");

    let mut router = App::new(|router, _, storage| {
        router
            .bank
            .init_balance(storage, &owner, init_funds)
            .unwrap();
    });

    // set up cw20 contract with some tokens for user_a and user_b
    let cw20_id = router.store_code(contract_cw20());
    let msg = cw20_base::msg::InstantiateMsg {
        name: "Cash Money".to_string(),
        symbol: "CASH".to_string(),
        decimals: 2,
        initial_balances: vec![
            Cw20Coin {
                address: ALICE.to_string(),
                amount: Uint128::new(ALICE_INIT_BAL),
            },
            Cw20Coin {
                address: BOB.to_string(),
                amount: Uint128::new(BOB_INIT_BAL),
            },
        ],
        mint: None,
        marketing: None,
    };
    let cash_addr = router
        .instantiate_contract(cw20_id, owner.clone(), &msg, &[], "CASH", None)
        .unwrap();

    // set up escrow contract
    let escrow_id = router.store_code(contract_escrow());
    let escrow_addr = router
        .instantiate_contract(
            escrow_id,
            owner.clone(),
            &InstantiateMsg {},
            &[],
            "Escrow",
            None,
        )
        .unwrap();

    // they are different
    assert_ne!(cash_addr, escrow_addr);

    // set up cw20 helpers
    let cash = Cw20Contract(cash_addr.clone());

    // ensure our balances
    let a_balance = cash.balance::<_, _, Empty>(&router, ALICE.to_string()).unwrap();
    assert_eq!(a_balance, Uint128::new(ALICE_INIT_BAL));
    let b_balance = cash.balance::<_, _, Empty>(&router, BOB.clone()).unwrap();
    assert_eq!(b_balance, Uint128::new(BOB_INIT_BAL));
    let escrow_balance = cash
        .balance::<_, _, Empty>(&router, escrow_addr.clone())
        .unwrap();
    assert_eq!(escrow_balance, Uint128::zero());

    /********************************************************* 
    alice creates escrow with bob
    this requires sending a msg for the escrow contract through
    the cw20 contract from alice's address with some funds.
    *********************************************************/

    // prepare the inner escrow msg    
    let create_msg = ReceiveMsg::Create(CreateMsg {
        id: T_ID.to_string(),
        user_b: BOB.to_string(),
        lock: String::from(ALICE_LOCK),
    });
    // prepare the cw20 message, containing the escrow msg
    // the amount of tokens sent here to the cw20 will make it to the escrow contract
    // and will constitute alice's deposit in the escrow
    let send_msg = Cw20ExecuteMsg::Send {
        contract: escrow_addr.to_string(),
        amount: Uint128::new(T_DEPOSIT_AMOUNT),
        msg: to_binary(&create_msg).unwrap(),
    };

    // send the TX from alice's account    
    let res = router
        .execute_contract(Addr::unchecked(ALICE), cash_addr.clone(), &send_msg, &[])
        .unwrap();
    assert_eq!(4, res.events.len());

    assert_eq!(res.events[0].ty.as_str(), "execute");
    let cw20_attr = res.custom_attrs(1);
    assert_eq!(4, cw20_attr.len());

    assert_eq!(res.events[2].ty.as_str(), "execute");
    let escrow_attr = res.custom_attrs(3);
    assert_eq!(2, escrow_attr.len());

    // ensure balances updated
    let a_balance = cash.balance::<_, _, Empty>(&router, ALICE.to_string()).unwrap();
    assert_eq!(a_balance, Uint128::new(ALICE_INIT_BAL - T_DEPOSIT_AMOUNT));
    let b_balance = cash.balance::<_, _, Empty>(&router, BOB.clone()).unwrap();
    assert_eq!(b_balance, Uint128::new(BOB_INIT_BAL));
    let escrow_balance = cash
        .balance::<_, _, Empty>(&router, escrow_addr.clone())
        .unwrap();
    assert_eq!(escrow_balance, Uint128::new(T_DEPOSIT_AMOUNT));

    // ensure escrow properly created
    let details: DetailsResponse = router
        .wrap()
        .query_wasm_smart(&escrow_addr, &QueryMsg::Details { id: T_ID.to_string() })
        .unwrap();
    assert_eq!(
        details,
        DetailsResponse {
            id: T_ID.to_string(),
            user_a: ALICE.to_string(),
            user_b: BOB.to_string(),
            deposit: Balance::Cw20(
                Cw20CoinVerified{
                    address:Addr::unchecked(cash_addr.clone()),
                    amount: Uint128::new(T_DEPOSIT_AMOUNT),
                },
            ),
            lock: ALICE_LOCK.to_string(),
            closed: false,
        }
    );


    /************************************
     * bob calls the withdraw function
     ***********************************/

    // prepare withdraw msg
    let withdraw_msg = ExecuteMsg::Withdraw(WithdrawMsg {
        id: T_ID.to_string(),
        secret: ALICE_SECRET.to_string(),
    });
    // send the TX from bob's account
    _ = router
        .execute_contract(Addr::unchecked(BOB), escrow_addr.clone(), &withdraw_msg, &[])
        .unwrap();
    
    // ensure balances updated
    let a_balance = cash.balance::<_, _, Empty>(&router, ALICE.to_string()).unwrap();
    assert_eq!(a_balance, Uint128::new(ALICE_INIT_BAL - T_DEPOSIT_AMOUNT));
    let b_balance = cash.balance::<_, _, Empty>(&router, BOB.clone()).unwrap();
    assert_eq!(b_balance, Uint128::new(BOB_INIT_BAL + T_DEPOSIT_AMOUNT));
    let escrow_balance = cash
        .balance::<_, _, Empty>(&router, escrow_addr.clone())
        .unwrap();
    assert_eq!(escrow_balance, Uint128::new(0));

    // ensure escrow properly closed
    let details: DetailsResponse = router
        .wrap()
        .query_wasm_smart(&escrow_addr, &QueryMsg::Details { id: T_ID.to_string() })
        .unwrap();
    assert_eq!(
        details,
        DetailsResponse {
            id: T_ID.to_string(),
            user_a: ALICE.to_string(),
            user_b: BOB.to_string(),
            deposit: Balance::Cw20(
                Cw20CoinVerified{
                    address:Addr::unchecked(cash_addr.clone()),
                    amount: Uint128::new(T_DEPOSIT_AMOUNT),
                },
            ),
            lock: ALICE_LOCK.to_string(),
            closed: true,
        }
    );
}