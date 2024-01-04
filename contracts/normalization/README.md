# Vulnerability: Insufficient Token Address Validation Control

The vulnerability is reflected in the lack of address validation prior to a change in checking the `black_list`. In function `execute_withdraw`, prior to checking if the `black_list` contains `destination`, `destination` is not validated. This leads to attacker exploiting it and withdrawing funds, even though he should be blacklisted.

#### Exploit 1
When contract is instantiated, owner is added to the blacklist. Malicious owner is able to bypass the verification and withdraw the funds simply by passing `destination` that is not in the same format.

#### Exploit 2
When contract is instantiated with `black_list`, because of the implementation of `instantiate` function, if `blac_list` has been passed as a parameter, it will be ignored, leading the attacker to exploit this and withdraw the funds, even though contract owner thinks he has blacklisted him.

Vulnerable parts of code:

```rust
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    store_config(deps.storage, &Config {
        owner:  deps.api.addr_validate(&msg.owner)?,
        black_list: vec![deps.api.addr_validate(&msg.owner)?]
    })?;
    Ok(Response::default())
}
```

```rust
fn execute_withdraw(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    destination: String,
  ) -> Result<Response, ContractError> {
    let config = read_config(deps.storage)?;
    if info.sender != config.owner {
        return Err(ContractError::Unauthorized{});
    }
    let black_list= config.black_list;
    let destination = Addr::unchecked(destination);
    if black_list.iter().any(|addr| addr==&destination) {
        return Err(ContractError::Unauthorized  {});
    }
    
    let destination =destination.to_string().to_lowercase();
    let contract_address = env.contract.address;
    let amount = deps.querier.query_all_balances(&contract_address)?;
    Ok(send_tokens(destination, amount, "approve"))
  }
```

## Solution

Solution to expoit 1 is to validate `destination` before verificating that `black_list` does not contain it.

```rust
fn execute_withdraw(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    destination: String,
  ) -> Result<Response, ContractError> {
    let config = read_config(deps.storage)?;
    if info.sender != config.owner {
        return Err(ContractError::Unauthorized{});
    }
    let black_list= config.black_list;
    let destination =deps.api.addr_validate(&destination)?;

    if black_list.iter().any(|addr| addr==&destination) {
        return Err(ContractError::BlackListed  {});
    }
    
    let contract_address = env.contract.address;
    let amount = deps.querier.query_all_balances(&contract_address)?;
    Ok(send_tokens(destination.to_string(), amount, "approve"))
  }
```

Solution to exploit 2 is to accept `black_list` sent with `InstantiateMsg` and append the validated owner's address.

```rust
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let mut black_list : Vec<Addr>= vec![deps.api.addr_validate(&msg.owner)?];
    black_list.append(&mut msg.black_list.unwrap_or(vec![]));

    store_config(deps.storage, &Config {
        owner:  deps.api.addr_validate(&msg.owner)?,
        black_list: black_list
    })?;
    Ok(Response::default())
}
```
