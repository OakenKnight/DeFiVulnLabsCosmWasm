# Vulnerability: Inadequate handling of unwrap

Vulnerability is reflected in the inadequate handling of *unwrap* method.In the following part of code, `PriceResponse` has been unwraped with `unwrap_or_default`.

When trying to *query* for `PriceResponse`, following error is being thrown:

```rust
called `Result::unwrap()` on an `Err` value: GenericErr { msg: "Querier contract error: cw_multi_test::wasm::ContractData not found" }
```

This leads to getting `default` values.

```rust
impl Default for PriceResponse {
    fn default() -> Self {
        Self {
            price: Coin {
                denom: "USD".to_string(),
                amount: Uint128::zero(),
            },
        }
    }
}
```

Vulnerable function:

```rust
 let usd_price = deps.querier
    .query_wasm_smart::<PriceResponse>(
        ORACLE.load(deps.storage)?,
        &OracleQueryMsg::Price { denom: denom },
        )
    .unwrap_or_default()
    .price
    .amount;
```
