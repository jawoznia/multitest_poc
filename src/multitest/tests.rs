use cosmwasm_std::Addr;

use crate::{
    contract::test_utils::CounterContractCodeId,
    sylvia_utils::{App, ExecParams},
};

#[test]
fn basic() {
    let mut app = App::default();
    let code_id = CounterContractCodeId::store_code(&mut app);

    let owner = Addr::unchecked("owner");
    let params = ExecParams::new(&owner, &[]);

    let contract = code_id
        .instantiate(&owner, "CounterContract", None)
        .unwrap();

    let resp = contract.counter_proxy().count().unwrap();
    assert_eq!(resp.count, 0);

    contract.counter_proxy().increase_count(params).unwrap();

    let resp = contract.counter_proxy().count().unwrap();
    assert_eq!(resp.count, 1);
}
