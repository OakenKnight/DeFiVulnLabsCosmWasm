#[cfg(test)]
pub mod tests{
    use cosmwasm_std::{Empty, coin, Addr, Coin, BankQuery, QueryRequest, Querier, from_binary, BalanceResponse, Uint128, to_json_binary};
    use cw20::{Cw20ReceiveMsg, MinterResponse};
    use cw_multi_test::{Contract, ContractWrapper, App, Executor};

    use crate::msg::{InstantiateMsg, Cw20HookMsg, ExecuteMsg, StakeResponse, QueryMsg};


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
    pub fn mint_native(app: &mut App, beneficiary: String, denom: String, amount: u128) {
        app.sudo(cw_multi_test::SudoMsg::Bank(
            cw_multi_test::BankSudo::Mint {
                to_address: beneficiary,
                amount: vec![coin(amount, denom)],
            },
        ))
        .unwrap();
    }

    pub fn query_balance_native(app: &App, address: &Addr, denom: &str) -> Coin {
        let req: QueryRequest<BankQuery> = QueryRequest::Bank(BankQuery::Balance { address: address.to_string(), denom: denom.to_string() });
        let res = app.raw_query(&to_json_binary(&req).unwrap()).unwrap().unwrap();
        let balance: BalanceResponse = from_binary(&res).unwrap();

        return balance.amount;        
    }
    
    pub const OWNER : &str = "owner";
    pub const ADMIN : &str = "admin";
    pub const USER : &str = "user";

    #[test]
    fn basic_flow(){
        let mut app = App::default();
        let cw_template_id = app.store_code(challenge_contract());
        let cw_20_id = app.store_code(token_contract());

        let token_inst = cw20_base::msg::InstantiateMsg {
            name: "Atom token".to_string(),
            symbol: "ATOM".to_string(),
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


        let contract_addr = app
            .instantiate_contract(
            cw_template_id,
            Addr::unchecked(OWNER),
            &InstantiateMsg { 
                owner: Addr::unchecked(OWNER).to_string(), 
                token: token_addr.to_string(), 
            },
            &[],
            "Contract",
            None,
        )
        .unwrap();

        let encoded = cosmwasm_std::to_binary(&Cw20HookMsg::Stake {});
       
        app.execute_contract(
                token_addr.clone(), 
                contract_addr.clone(), 
            &ExecuteMsg::Receive(Cw20ReceiveMsg {
                sender: USER.to_string(),
                amount: Uint128::new(2137),
                msg: encoded.unwrap(),
            }), 
            &[])
            .unwrap();
        
        let query_stkaing_info: StakeResponse = app
            .wrap()
            .query_wasm_smart(contract_addr, 
                &QueryMsg::StakerInfo{ staker: USER.to_string() 
            }).unwrap();

        assert_eq!(Uint128::new(2137),query_stkaing_info.amount);
    }
}