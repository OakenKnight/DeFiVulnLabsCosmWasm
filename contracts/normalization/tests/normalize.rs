// use cosmwasm_std::{Addr, QueryRequest, coin, BankQuery, Querier, to_binary, Coin, from_binary, BalanceResponse};
// use cw_receive::contract::{execute, instantiate, query};
// use cw_receive::msg::{InstantiateMsg, ExecuteMsg, QueryMsg, BlacklistResponse};
// use cw_multi_test::{App, ContractWrapper, Executor};


// #[test]
//     fn lack_of_address_normalization_test() {
//         let mut app = App::default();
//         let code = ContractWrapper::new(execute, instantiate, query);
//         let code_id = app.store_code(Box::new(code));
//         // The contract has no real life functionality
//         let contract_addr = app
//             .instantiate_contract(
//                 code_id,
//                 Addr::unchecked("owner"),
//                 &InstantiateMsg { owner: Addr::unchecked("owner").to_string(), black_list: None },
//                 &[],
//                 "Contract",
//                 None,
//             )
//             .unwrap();

//             mint_native(&mut app, contract_addr.to_string(), "ATOM".to_string(), 100);
//             let query_black_list: BlacklistResponse = app.wrap().query_wasm_smart(contract_addr.clone(), &QueryMsg::BlackList { }).unwrap();
//             println!("{:?}", query_black_list);

//             let bypass_blacklist = app.execute_contract(Addr::unchecked("owner"), contract_addr.clone(), &ExecuteMsg::Withdraw { destination: "oWner".to_string()},&[]).unwrap();
//             println!("{:?}",bypass_blacklist);
//             let balance = query_balance_native(&app, &Addr::unchecked("owner"), "ATOM");
//             assert_eq!(coin(100, "ATOM"), balance);
//         }
        