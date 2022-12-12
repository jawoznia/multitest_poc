use cosmwasm_std::Addr;
use cw_multi_test::App;

use crate::contract::test_utils::CounterCodeId;

#[test]
fn basic() {
    let mut app = App::default();

    let owner = Addr::unchecked("owner");

    let code_id = CounterCodeId::store_code(&mut app);

    let contract = code_id
        .instantiate(&mut app, &owner, "CounterContract", None)
        .unwrap();

    let resp = contract.count(&app).unwrap();
    assert_eq!(resp.count, 0);

    contract.increase_counter(&mut app, &owner).unwrap();

    let resp = contract.count(&app).unwrap();
    assert_eq!(resp.count, 1);
}
