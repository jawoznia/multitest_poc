use cosmwasm_std::Addr;
use sylvia::multitest::App;

use crate::contract::CounterContract;
use crate::error::ContractError;
use crate::sylvia_utils::Multitest;

const LABEL: &str = "CounterContract";

#[test]
fn basic() {
    let mut app = App::default();
    let code_id = CounterContract::store_code(&mut app);

    let owner = "owner";
    let owner_addr = Addr::unchecked(owner);

    let contract = code_id.instantiate().call(owner).unwrap();

    let resp = contract.counter_proxy().count().unwrap();
    assert_eq!(resp.count, 0);

    contract
        .counter_proxy()
        .increase_count()
        .with_sender(owner_addr.as_str())
        .call()
        .unwrap();

    let resp = contract.counter_proxy().count().unwrap();
    assert_eq!(resp.count, 1);
}

#[test]
fn overflow() {
    let mut app = App::default();
    let code_id = CounterContract::store_code(&mut app);

    let owner = "owner";
    let owner_addr = Addr::unchecked(owner);

    let contract = code_id.instantiate().with_label(LABEL).call(owner).unwrap();

    let resp = contract.counter_proxy().count().unwrap();
    assert_eq!(resp.count, 0);

    contract
        .counter_proxy()
        .increase_count()
        .with_sender(owner_addr.as_str())
        .call()
        .unwrap();

    let resp = contract.counter_proxy().count().unwrap();
    assert_eq!(resp.count, 1);

    contract
        .set_counter_step(42)
        .with_sender(owner_addr.as_str())
        .call()
        .unwrap();

    let err = contract
        .counter_proxy()
        .increase_count()
        .with_sender(owner_addr.as_str())
        .call()
        .unwrap_err();
    assert_eq!(err, ContractError::Overflow);
}
