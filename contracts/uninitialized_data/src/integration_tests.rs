#[cfg(test)]
pub mod tests{
    use cosmwasm_std::{Addr,Empty};
    use crate::msg::{InstantiateMsg, ExecuteMsg};
    use cw_multi_test::{App, Contract, ContractWrapper, Executor};
    
    pub fn challenge_contract() -> Box<dyn Contract<Empty>> {
        let contract = ContractWrapper::new(
            crate::contract::execute,
            crate::contract::instantiate,
            crate::contract::query,
        );
        Box::new(contract)
    }
    pub const MARKET : &str = "SOME_MARKET";
    #[test]
    fn basic_flow(){
        let mut app = App::default();
        let cw_template_id = app.store_code(challenge_contract());

        let contract_addr = app
            .instantiate_contract(
                cw_template_id,
                Addr::unchecked("owner"),
                &InstantiateMsg { owner: "owner".to_owned(), markets: vec![MARKET.to_string()] },
                &[],
                "Contract",
                None,
            )
            .unwrap();

        
        app.execute_contract(
                Addr::unchecked("owner"), 
                contract_addr.clone(), 
                &ExecuteMsg::UpdateMarkets { market: MARKET.to_string() }, 
                &[])
            .unwrap();
    }
}