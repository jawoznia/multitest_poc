use cosmwasm_std::Addr;

use crate::{
    contract::test_utils::CounterContractCodeId,
    error::ContractError,
    sylvia_utils::{App, ExecParams, InstantiateParams},
};

const LABEL: &str = "CounterContract";

#[test]
fn basic() {
    let mut app = App::default();
    let code_id = CounterContractCodeId::store_code(&mut app);

    let owner = Addr::unchecked("owner");
    let exec_params = ExecParams::new(&owner, &[]);
    let instantiate_params = InstantiateParams::new(&owner, &[], LABEL, None);

    let contract = code_id.instantiate(instantiate_params).unwrap();

    let resp = contract.counter_proxy().count().unwrap();
    assert_eq!(resp.count, 0);

    contract
        .counter_proxy()
        .increase_count(exec_params)
        .unwrap();

    let resp = contract.counter_proxy().count().unwrap();
    assert_eq!(resp.count, 1);
}

#[test]
fn overflow() {
    let mut app = App::default();
    let code_id = CounterContractCodeId::store_code(&mut app);

    let owner = Addr::unchecked("owner");
    let exec_params = ExecParams::new(&owner, &[]);
    let instantiate_params = InstantiateParams::new(&owner, &[], LABEL, None);

    let contract = code_id.instantiate(instantiate_params).unwrap();

    let resp = contract.counter_proxy().count().unwrap();
    assert_eq!(resp.count, 0);

    contract
        .counter_proxy()
        .increase_count(exec_params.clone())
        .unwrap();

    let resp = contract.counter_proxy().count().unwrap();
    assert_eq!(resp.count, 1);

    contract.set_counter_step(exec_params.clone(), 42).unwrap();

    let err = contract
        .counter_proxy()
        .increase_count(exec_params.clone())
        .unwrap_err();
    assert_eq!(err, ContractError::Overflow);
}
