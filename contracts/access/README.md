# Vulnerability: Insufficient Access Control

The vulnerability is reflected in the lack of ownership verification prior to a change in ownership. In function `execute_update`, prior to updating `CONFIG` it is not checked if the owner is the one who is trying to execute that transaction. This leads to attacker exploiting and changing the ownership and late on being able to withdraw the funds.

Vulnerable part of code:

 ```rust
  fn execute_update(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    owner: String,
  ) -> Result<Response, ContractError> {
    let new_config = Config {
        owner: deps.api.addr_validate(&owner)?,
    };
    CONFIG.save(deps.storage, &new_config)?;
    let resp= Response::new()
    .add_attribute("action", "UpdateConfig")
    .add_attribute("Owner", owner.to_string());
    Ok(resp)
  }
```

## Solution

Solution is to check if the user executing `execute_update` transaction is the owner of the contract. If that is not the case, it should be returned `Unauthorized`.
Solution:

```rust
 fn execute_update(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    owner: String,
  ) -> Result<Response, ContractError> {
    let current_owner = CONFIG.load(deps.storage)?;
    if current_owner.owner != owner {
        return Err(ContractError::Unauthorized {  })
    }
    let new_config = Config {
        owner: deps.api.addr_validate(&owner)?,
    };
    CONFIG.save(deps.storage, &new_config)?;
    let resp= Response::new()
    .add_attribute("action", "UpdateConfig")
    .add_attribute("Owner", owner.to_string());
    Ok(resp)
  }
```
