#[cfg(test)]
pub mod tests {
    use cosmwasm_std::{Empty, Addr, Uint128};
    use cw20::{Cw20ReceiveMsg, MinterResponse};
    use cw_multi_test::{Contract, ContractWrapper, App, Executor};

    use crate::{msg::{InstantiateMsg, Cw20HookMsg, ExecuteMsg, UserResponse, QueryMsg}, assets::AssetInfo};

    pub fn challenge_contract() -> Box<dyn Contract<Empty>> {
        let contract = ContractWrapper::new(
            crate::contract::execute,
            crate::contract::instantiate,
            crate::contract::query,
        );
        Box::new(contract)
    }
    pub fn token_contract() -> Box<dyn Contract<Empty>> {
        let contract = ContractWrapper::new(
            cw20_base::contract::execute,
            cw20_base::contract::instantiate,
            cw20_base::contract::query,
        );
        Box::new(contract)
    }
    pub const OWNER : &str = "owner";
    pub const ADMIN : &str = "admin";
    pub const USER : &str = "user";
    pub const TOKEN_1 : &str = "tokenA";
    pub const TOKEN_2 : &str = "tokenB";

    #[test]
    fn basic_flow(){
        let mut app = App::default();
        let cw_template_id = app.store_code(challenge_contract());
        let cw_20_id = app.store_code(token_contract());

        let token_inst = cw20_base::msg::InstantiateMsg {
            name: TOKEN_1.to_string(),
            symbol: TOKEN_1.to_string(),
            decimals: 6,
            initial_balances: vec![],
            mint: Some(MinterResponse {
                minter: ADMIN.to_string(),
                cap: None,
            }),
            marketing: None,
        };
        let token_inst2 = cw20_base::msg::InstantiateMsg {
            name: TOKEN_2.to_string(),
            symbol: TOKEN_2.to_string(),
            decimals: 6,
            initial_balances: vec![],
            mint: Some(MinterResponse {
                minter: ADMIN.to_string(),
                cap: None,
            }),
            marketing: None,
        };
        let token_addr = app
            .instantiate_contract(
                cw_20_id,
                Addr::unchecked(ADMIN),
                &token_inst,
                &[],
                "test",
                None,
            )
            .unwrap();

        let token_addr2 = app
            .instantiate_contract(
                cw_20_id,
                Addr::unchecked(ADMIN),
                &token_inst2,
                &[],
                "test",
                None,
            )
            .unwrap();


        let contract_addr = app
            .instantiate_contract(
                cw_template_id,
                Addr::unchecked(ADMIN),
                &InstantiateMsg { 
                    owner: Addr::unchecked(ADMIN).to_string(), 
                    asset_infos: vec![
                        AssetInfo::Token{contract_addr:token_addr.clone()},
                        AssetInfo::Token{contract_addr:token_addr2.clone()},

                        ]
                    },
                &[],
                "Contract",
                None,
            ).unwrap();

        let encoded = cosmwasm_std::to_binary(&Cw20HookMsg::Deposit {});
        app.execute_contract(
            token_addr.clone(), 
            contract_addr.clone(), 
            &ExecuteMsg::Receive(Cw20ReceiveMsg {
                sender: USER.to_string(),
                amount: Uint128::new(2137),
                msg: encoded.unwrap(),
            }), 
        &[]
        ).unwrap();
        
        let query_user_info: UserResponse = app
            .wrap()
            .query_wasm_smart(
                contract_addr.clone(), 
                &QueryMsg::UserInfo{ user: USER.to_string()})
            .unwrap();
        assert_eq!(query_user_info , UserResponse{amount : Uint128::new(2137)});

    }
    #[test]
    fn multiple_deposits(){
        let mut app = App::default();
        let cw_template_id = app.store_code(challenge_contract());
        let cw_20_id = app.store_code(token_contract());

        let token_inst = cw20_base::msg::InstantiateMsg {
            name: TOKEN_1.to_string(),
            symbol: TOKEN_1.to_string(),
            decimals: 6,
            initial_balances: vec![],
            mint: Some(MinterResponse {
                minter: ADMIN.to_string(),
                cap: None,
            }),
            marketing: None,
        };
        let token_inst2 = cw20_base::msg::InstantiateMsg {
            name: TOKEN_2.to_string(),
            symbol: TOKEN_2.to_string(),
            decimals: 6,
            initial_balances: vec![],
            mint: Some(MinterResponse {
                minter: ADMIN.to_string(),
                cap: None,
            }),
            marketing: None,
        };
        let token_addr = app
            .instantiate_contract(
                cw_20_id,
                Addr::unchecked(ADMIN),
                &token_inst,
                &[],
                "test",
                None,
            )
            .unwrap();

        let token_addr2 = app
            .instantiate_contract(
                cw_20_id,
                Addr::unchecked(ADMIN),
                &token_inst2,
                &[],
                "test",
                None,
            )
            .unwrap();


        let contract_addr = app
            .instantiate_contract(
                cw_template_id,
                Addr::unchecked(ADMIN),
                &InstantiateMsg { 
                    owner: Addr::unchecked(ADMIN).to_string(), 
                    asset_infos: vec![
                        AssetInfo::Token{contract_addr:token_addr.clone()},
                        AssetInfo::Token{contract_addr:token_addr2.clone()},

                        ]
                    },
                &[],
                "Contract",
                None,
            ).unwrap();

        let encoded = cosmwasm_std::to_binary(&Cw20HookMsg::Deposit {});
        app.execute_contract(
            token_addr.clone(), 
            contract_addr.clone(), 
            &ExecuteMsg::Receive(Cw20ReceiveMsg {
                sender: USER.to_string(),
                amount: Uint128::new(2137),
                msg: encoded.unwrap(),
            }), 
        &[]
        ).unwrap();
        
        let query_user_info: UserResponse = app
            .wrap()
            .query_wasm_smart(
                contract_addr.clone(), 
                &QueryMsg::UserInfo{ user: USER.to_string()})
            .unwrap();
        assert_eq!(query_user_info , UserResponse{amount : Uint128::new(2137)});
        
        let encoded: Result<cosmwasm_std::Binary, cosmwasm_std::StdError> = cosmwasm_std::to_binary(&Cw20HookMsg::Deposit {});
        app.execute_contract(
            token_addr.clone(), 
            contract_addr.clone(), 
            &ExecuteMsg::Receive(Cw20ReceiveMsg {
                sender: USER.to_string(),
                amount: Uint128::new(7),
                msg: encoded.unwrap(),
            }), 
        &[]
        ).unwrap();
        
        let query_user_info: UserResponse = app
            .wrap()
            .query_wasm_smart(
                contract_addr.clone(), 
                &QueryMsg::UserInfo{ user: USER.to_string()})
            .unwrap();
        assert_eq!(query_user_info , UserResponse{amount : Uint128::new(2144)});
    }
}


