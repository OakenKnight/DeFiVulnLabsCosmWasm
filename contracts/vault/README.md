# Vunerability: Inadequately updating the state

Vulnerability is reflected in the inadequately updating `USER_INFO` state. If there has already been a deposit, this update will just overwrite it with new value, leading to the user losing tokens.

Vulnerable part of code:

```rust
    pub fn receive_cw20(
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        cw20_msg: Cw20ReceiveMsg,
    ) -> Result<Response, ContractError> {
        match from_binary(&cw20_msg.msg) {
            Ok(Cw20HookMsg::Deposit {}) => {
                ...
                
                USER_INFO.save(deps.storage, &cw20_sender, &UserInfo {amount: cw20_msg.amount })?;
                
                ...
            }
            Err(_) => Err(ContractError::MissingData {}),
        }
    }
```

## Solution

Instad of only saving new amount that user has deposited, solution is to update the state with the sum of current deposited amount of tokens and tokens that user wants to deposit in that transaction.

```rust
pub fn receive_cw20(
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        cw20_msg: Cw20ReceiveMsg,
    ) -> Result<Response, ContractError> {
        match from_binary(&cw20_msg.msg) {
            Ok(Cw20HookMsg::Deposit {}) => {
                ...
                
                USER_INFO.update(deps.storage, &cw20_sender, |user_info:Option<UserInfo>| -> StdResult<_>{
                    let mut info = user_info.unwrap_or(UserInfo { amount: 0u128.into()});
                    info.amount = info.amount + cw20_msg.amount;
                    Ok(info)
                })?; 
                
                ...
            }
            Err(_) => Err(ContractError::MissingData {}),
        }
    }
```