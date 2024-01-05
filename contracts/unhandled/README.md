# Vulnerability: Inadequate error handling

Vulnerability is reflected in the inadequate handling of error returned by function `check_authorization` called within `execute_update` function. This leads to attacker exploiting verification process. Even though attacker is not owner of the contract, because of this bug, he is able to update contract configuration and propose himself as the new owner of the contract and thus be able to withdraw funds from the contract.

Vulnerable part of code:

```rust
  fn execute_update(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    new_owner: String,
  ) -> Result<Response, ContractError> {
    ...

    check_authorization(info.sender,owner);
    
    ...
  }
```

## Solution

Solution is to use operator `?` to propagate the error if it has occured.

```rust
  fn execute_update(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    new_owner: String,
  ) -> Result<Response, ContractError> {
    ...
    
    check_authorization(info.sender,owner)?;
    
    ...
  }
```
