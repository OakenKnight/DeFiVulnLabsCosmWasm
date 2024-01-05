#[cfg(test)]

pub mod tests {
    use cosmwasm_std::{coin, Addr, Coin, BankQuery, QueryRequest, Querier, to_binary, from_binary, BalanceResponse, Empty};
    use cw_multi_test::{App, Contract, ContractWrapper, Executor};

    use crate::msg::{InstantiateMsg, ExecuteMsg};

    
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
        let res = app.raw_query(&to_binary(&req).unwrap()).unwrap().unwrap();
        let balance: BalanceResponse = from_binary(&res).unwrap();

        return balance.amount;        
    }
    pub fn challenge_contract() -> Box<dyn Contract<Empty>> {
        let contract = ContractWrapper::new(
            crate::contract::execute,
            crate::contract::instantiate,
            crate::contract::query,
        );
        Box::new(contract)
    }
    pub const ADMIN : &str = "admin";
    pub const OWNER : &str = "owner";
    pub const USER : &str = "user";
    pub const DENOM : &str = "ATOM";
    #[test]
    fn basic_flow(){
        let mut app = App::default();
        let cw_template_id = app.store_code(challenge_contract());

        let contract_addr = app
            .instantiate_contract(
                cw_template_id,
                Addr::unchecked(OWNER),
                &InstantiateMsg { owner: Addr::unchecked(OWNER).to_string() },
                &[],
                "Contract",
                None,
            )
            .unwrap();
            mint_native(&mut app, contract_addr.to_string(), DENOM.to_string(), 100);

            app.execute_contract(
                    Addr::unchecked(USER), 
                    contract_addr.clone(), 
                    &ExecuteMsg::Withdraw{destination: USER.to_string()}, 
                    &[]
            ).unwrap_err();

            app.execute_contract(
                    Addr::unchecked(OWNER), 
                    contract_addr.clone(), 
                    &ExecuteMsg::UpdateConfig { new_owner: USER.to_string() }, 
                    &[]
            ).unwrap();

            app.execute_contract(
                Addr::unchecked(USER), 
                contract_addr.clone(), 
                &ExecuteMsg::Withdraw{destination: USER.to_string()}, 
                &[]
            ).unwrap();
            
            let balance = query_balance_native(&mut app, &Addr::unchecked(USER), DENOM);
            assert_eq!(coin(100, DENOM), balance);
    }
}