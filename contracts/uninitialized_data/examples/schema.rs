use cosmwasm_schema::write_api;
use cw_unsaved_init_values::msg::{InstantiateMsg, ExecuteMsg, QueryMsg};


fn main() {
    write_api! {
        instantiate: InstantiateMsg,
        query: QueryMsg,
        execute: ExecuteMsg,
    }
}
