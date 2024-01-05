# Vulnerability: Insufficient Token Address Validation Control

The vulnerability is reflected in the lack of token address verification prior to staking. Function `receive_cw20` is supposed to handle `Send` messages, but it should ensure the sender matches the token contract it expects to handle, and not allow arbirary addresses. The address of the contract is stored in `info.sender` so it cannot be faked, but if sender is something different then `token`, it is a problem. Due to the fact that contract does not verify `info.sender`, attacker can spoof the ReceiveMsg.

Vulnerable part of code:

```rust
pub fn receive_cw20(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    cw20_msg: Cw20ReceiveMsg,
) -> Result<Response, ContractError> {
    match from_binary(&cw20_msg.msg) {
        Ok(Cw20HookMsg::Stake {}) => {
            let cw20_sender = deps.api.addr_validate(&cw20_msg.sender)?;
            stake(deps, env, cw20_sender, cw20_msg.amount)
        }
        Err(_) => Err(ContractError::MissingData {}),
    }
}
```

## Solution

Solution is to always verify `info.sender` to `token` address and thus not allow for spoofing.

```rust
pub fn receive_cw20(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    cw20_msg: Cw20ReceiveMsg,
) -> Result<Response, ContractError> {
    match from_binary(&cw20_msg.msg) {
        Ok(Cw20HookMsg::Stake {}) => {
            let config : Config = read_config(deps.storage)?;

            // only owner can create allocation
            if config.token != info.sender {
                return Err(ContractError::Unauthorized {});
            }

            let cw20_sender = deps.api.addr_validate(&cw20_msg.sender)?;
            stake(deps, env, cw20_sender, cw20_msg.amount)
        }
        Err(_) => Err(ContractError::MissingData {}),
    }
}
```