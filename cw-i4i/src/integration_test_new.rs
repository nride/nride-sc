#![cfg(test)]

use cosmwasm_std::{coins, to_binary, Addr, Empty, Uint128, testing::mock_env, Timestamp};
use cw20::{Cw20Coin, Cw20Contract, Cw20ExecuteMsg, Balance, Cw20CoinVerified};
use cw_multi_test::{App, Contract, ContractWrapper, Executor};

use crate::new_msg::{CreateMsg, TopUpMsg, ApproveMsg, DetailsResponse, ExecuteMsg, InstantiateMsg, QueryMsg, ReceiveMsg};

pub fn contract_escrow() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(
        crate::new_contract::execute,
        crate::new_contract::instantiate,
        crate::new_contract::query,
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
// receive cw20 tokens and release upon approval
fn escrow_happy_path_cw20_tokens() {

    const T_ID: &str= "demo"; // escrow ID
    const ALICE: &str = "alice"; // alice Addr
    const BOB: &str  = "bob";   // bob Addr
    const ALICE_INIT_BAL: u128 = 5000; // alice initial balance in cw20 token
    const BOB_INIT_BAL: u128 = 5000; // bob initial balance in cw20 token
    
    // Escrow Params
    const T_T1_TIMEOUT: u64 = 1000000;
    const T_T2_TIMEOUT: u64 = 4000000;
    const ALICE_LOCK: &str = "04b4ac68eff3a82d86db5f0489d66f91707e99943bf796ae6a2dcb2205c9522fa7915428b5ac3d3b9291e62142e7246d85ad54504fabbdb2bae5795161f8ddf259";
    const ALICE_SECRET: &str  =  "3c9229289a6125f7fdf1885a77bb12c37a8d3b4962d936f7e3084dece32a3ca1";   
    const BOB_LOCK: &str = "042d5f7beb52d336163483804facb17c47033fb14dfc3f3c88235141bae1896fc8d99a685aafaf92d5f41d866fe387b988a998590326f1b549878b9d03eabed7e5";
    const BOB_SECRET: &str =  "cde73ee8f8584c54ac455c941f75990f4bff47a4340023e3fd236344e9a7d4ea";   
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
        t1_timeout: T_T1_TIMEOUT,
        t2_timeout: T_T2_TIMEOUT,
        account_b_lock: String::from(ALICE_LOCK),
    });
    // prepare the cw20 message, containing the escrow msg
    // the amount of tokens sent here to the cw20 will make it to the escrow contract
    // and will constitute alice's initial deposit in the escrow
    // bob will have to match that amount
    let send_msg = Cw20ExecuteMsg::Send {
        contract: escrow_addr.to_string(),
        amount: Uint128::new(T_DEPOSIT_AMOUNT),
        msg: to_binary(&create_msg).unwrap(),
    };
    // set block time to 500'000, which is below T1
    let mut block = mock_env().block;
    block.time = Timestamp::from_seconds(500000);
    router.set_block(block);
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
            account_a_state: "[FUNDED|NONE]".to_string(),
            account_a_lock: None,
            user_b: BOB.to_string(),
            account_b_state: "[INIT|NONE]".to_string(),
            account_b_lock: Some(ALICE_LOCK.to_string()),
            t1_timeout: T_T1_TIMEOUT.clone(),
            t2_timeout: T_T2_TIMEOUT.clone(),
            required_deposit: Balance::Cw20(
                Cw20CoinVerified{
                    address:Addr::unchecked(cash_addr.clone()),
                    amount: Uint128::new(T_DEPOSIT_AMOUNT),
                },
            ),
            payout: None,
            closed: false,
        }
    );

    /********************************************************* 
    bob tops up the escrow
    this requires sending a msg for the escrow contract through
    the cw20 contract from bob's address with some funds.
    *********************************************************/

    // prepare escrow topup msg
    let topup_msg = ReceiveMsg::TopUp(TopUpMsg {
        id: T_ID.to_string(),
        account_a_lock: String::from(BOB_LOCK),
    });
    // prepare the cw20 msg with funds matching the required deposit amount
    let send_msg = Cw20ExecuteMsg::Send {
        contract: escrow_addr.to_string(),
        amount: Uint128::new(T_DEPOSIT_AMOUNT),
        msg: to_binary(&topup_msg).unwrap(),
    };
    // set block time to 600'000, still before T1
    let mut block = mock_env().block;
    block.time = Timestamp::from_seconds(600000);
    router.set_block(block);
    // send the TX from bob's account
    let res = router
        .execute_contract(Addr::unchecked(BOB), cash_addr.clone(), &send_msg, &[])
        .unwrap();
    assert_eq!(4, res.events.len());

    // ensure balances updated
    let a_balance = cash.balance::<_, _, Empty>(&router, ALICE.to_string()).unwrap();
    assert_eq!(a_balance, Uint128::new(ALICE_INIT_BAL - T_DEPOSIT_AMOUNT));
    let b_balance = cash.balance::<_, _, Empty>(&router, BOB.clone()).unwrap();
    assert_eq!(b_balance, Uint128::new(BOB_INIT_BAL - T_DEPOSIT_AMOUNT));
    let escrow_balance = cash
        .balance::<_, _, Empty>(&router, escrow_addr.clone())
        .unwrap();
    assert_eq!(escrow_balance, Uint128::new(2*T_DEPOSIT_AMOUNT));

    // ensure escrow properly updated
    let details: DetailsResponse = router
    .wrap()
    .query_wasm_smart(&escrow_addr, &QueryMsg::Details { id: T_ID.to_string() })
    .unwrap();
    assert_eq!(
        details,
        DetailsResponse {
            id: T_ID.to_string(),
            user_a: ALICE.to_string(),
            account_a_state: "[FUNDED|NONE]".to_string(),
            account_a_lock: Some(BOB_LOCK.to_string()),
            user_b: BOB.to_string(),
            account_b_state: "[FUNDED|NONE]".to_string(),
            account_b_lock: Some(ALICE_LOCK.to_string()),
            t1_timeout: T_T1_TIMEOUT.clone(),
            t2_timeout: T_T2_TIMEOUT.clone(),
            required_deposit: Balance::Cw20(
                Cw20CoinVerified{
                    address:Addr::unchecked(cash_addr.clone()),
                    amount: Uint128::new(T_DEPOSIT_AMOUNT),
                },
            ),
            payout: None,
            closed: false,
        }
    );

    /*************************************************************************************** 
    alice approves using bob's secret (which is normally communicated offchain upon pickup)
    this tx can go directly to the escrow contract, because it doesnt require sending any 
    tokens
    ***************************************************************************************/

    // prepare approve msg
    let approve_msg = ExecuteMsg::Approve(ApproveMsg {
        id: T_ID.to_string(),
        secret: String::from(BOB_SECRET),
    });
    // set block time to 2'000'000, between T1 and T2
    let mut block = mock_env().block;
    block.time = Timestamp::from_seconds(2000000);
    router.set_block(block);
    // send the TX from alice's account
    _ = router
        .execute_contract(Addr::unchecked(ALICE), escrow_addr.clone(), &approve_msg, &[])
        .unwrap();
    
    // ensure balances haven't changed
    let a_balance = cash.balance::<_, _, Empty>(&router, ALICE.to_string()).unwrap();
    assert_eq!(a_balance, Uint128::new(ALICE_INIT_BAL - T_DEPOSIT_AMOUNT));
    let b_balance = cash.balance::<_, _, Empty>(&router, BOB.clone()).unwrap();
    assert_eq!(b_balance, Uint128::new(BOB_INIT_BAL - T_DEPOSIT_AMOUNT));
    let escrow_balance = cash
        .balance::<_, _, Empty>(&router, escrow_addr.clone())
        .unwrap();
    assert_eq!(escrow_balance, Uint128::new(2*T_DEPOSIT_AMOUNT));

    // ensure escrow properly updated
    let details: DetailsResponse = router
        .wrap()
        .query_wasm_smart(&escrow_addr, &QueryMsg::Details { id: T_ID.to_string() })
        .unwrap();
    assert_eq!(
        details,
        DetailsResponse {
            id: T_ID.to_string(),
            user_a: ALICE.to_string(),
            account_a_state: "[FUNDED|APPROVED]".to_string(),
            account_a_lock: Some(BOB_LOCK.to_string()),
            user_b: BOB.to_string(),
            account_b_state: "[FUNDED|NONE]".to_string(),
            account_b_lock: Some(ALICE_LOCK.to_string()),
            t1_timeout: T_T1_TIMEOUT.clone(),
            t2_timeout: T_T2_TIMEOUT.clone(),
            required_deposit: Balance::Cw20(
                Cw20CoinVerified{
                    address:Addr::unchecked(cash_addr.clone()),
                    amount: Uint128::new(T_DEPOSIT_AMOUNT),
                },
            ),
            payout: None,
            closed: false,
        }
    );

    /*************************************************************************************** 
    bob approves using alice's secret
    ***************************************************************************************/

    // prepare approve msg
    let approve_msg = ExecuteMsg::Approve(ApproveMsg {
        id: T_ID.to_string(),
        secret: String::from(ALICE_SECRET),
    });
    // set block time to 3'000'000, between T1 and T2
    let mut block = mock_env().block;
    block.time = Timestamp::from_seconds(3000000);
    router.set_block(block);
    // send the TX from alice's account
    _ = router
        .execute_contract(Addr::unchecked(BOB), escrow_addr.clone(), &approve_msg, &[])
        .unwrap();
    
    // ensure balances haven't changed
    let a_balance = cash.balance::<_, _, Empty>(&router, ALICE.to_string()).unwrap();
    assert_eq!(a_balance, Uint128::new(ALICE_INIT_BAL - T_DEPOSIT_AMOUNT));
    let b_balance = cash.balance::<_, _, Empty>(&router, BOB.clone()).unwrap();
    assert_eq!(b_balance, Uint128::new(BOB_INIT_BAL - T_DEPOSIT_AMOUNT));
    let escrow_balance = cash
        .balance::<_, _, Empty>(&router, escrow_addr.clone())
        .unwrap();
    assert_eq!(escrow_balance, Uint128::new(2*T_DEPOSIT_AMOUNT));

    // ensure escrow properly updated
    let details: DetailsResponse = router
        .wrap()
        .query_wasm_smart(&escrow_addr, &QueryMsg::Details { id: T_ID.to_string() })
        .unwrap();
    assert_eq!(
        details,
        DetailsResponse {
            id: T_ID.to_string(),
            user_a: ALICE.to_string(),
            account_a_state: "[FUNDED|APPROVED]".to_string(),
            account_a_lock: Some(BOB_LOCK.to_string()),
            user_b: BOB.to_string(),
            account_b_state: "[FUNDED|APPROVED]".to_string(),
            account_b_lock: Some(ALICE_LOCK.to_string()),
            t1_timeout: T_T1_TIMEOUT.clone(),
            t2_timeout: T_T2_TIMEOUT.clone(),
            required_deposit: Balance::Cw20(
                Cw20CoinVerified{
                    address:Addr::unchecked(cash_addr.clone()),
                    amount: Uint128::new(T_DEPOSIT_AMOUNT),
                },
            ),
            payout: None,
            closed: false,
        }
    );

    /************************************
     * alice calls the withdraw function
     ***********************************/

    // prepare withdraw msg
    let withdraw_msg = ExecuteMsg::Withdraw{id: T_ID.to_string()};
    // set block time to 5'000'000, after T2
    let mut block = mock_env().block;
    block.time = Timestamp::from_seconds(5000000);
    router.set_block(block);
    // send the TX from alice's account
    _ = router
        .execute_contract(Addr::unchecked(ALICE), escrow_addr.clone(), &withdraw_msg, &[])
        .unwrap();
    
    // ensure balances updated (they both get their money back)
    let a_balance = cash.balance::<_, _, Empty>(&router, ALICE.to_string()).unwrap();
    assert_eq!(a_balance, Uint128::new(ALICE_INIT_BAL));
    let b_balance = cash.balance::<_, _, Empty>(&router, BOB.clone()).unwrap();
    assert_eq!(b_balance, Uint128::new(BOB_INIT_BAL));
    let escrow_balance = cash
        .balance::<_, _, Empty>(&router, escrow_addr.clone())
        .unwrap();
    assert_eq!(escrow_balance, Uint128::new(0));

    // ensure escrow properly updated
    let details: DetailsResponse = router
        .wrap()
        .query_wasm_smart(&escrow_addr, &QueryMsg::Details { id: T_ID.to_string() })
        .unwrap();
    assert_eq!(
        details,
        DetailsResponse {
            id: T_ID.to_string(),
            user_a: ALICE.to_string(),
            account_a_state: "[FUNDED|APPROVED]".to_string(),
            account_a_lock: Some(BOB_LOCK.to_string()),
            user_b: BOB.to_string(),
            account_b_state: "[FUNDED|APPROVED]".to_string(),
            account_b_lock: Some(ALICE_LOCK.to_string()),
            t1_timeout: T_T1_TIMEOUT.clone(),
            t2_timeout: T_T2_TIMEOUT.clone(),
            required_deposit: Balance::Cw20(
                Cw20CoinVerified{
                    address:Addr::unchecked(cash_addr.clone()),
                    amount: Uint128::new(T_DEPOSIT_AMOUNT),
                },
            ),
            payout: Some("(100,100)".to_string()),
            closed: true,
        }
    );
}
