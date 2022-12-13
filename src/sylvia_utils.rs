use std::cell::RefCell;

use cosmwasm_std::{Addr, Coin};

pub struct App {
    pub app: RefCell<cw_multi_test::App>,
}

pub struct ExecParams<'a> {
    pub sender: &'a Addr,
    pub funds: &'a [Coin],
}
