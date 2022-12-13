use std::cell::RefCell;

use cosmwasm_std::{Addr, Coin};

pub struct App {
    pub app: RefCell<cw_multi_test::App>,
}

impl App {
    pub fn new() -> Self {
        Self {
            app: RefCell::new(cw_multi_test::App::default()),
        }
    }
}

pub struct ExecParams<'a> {
    pub sender: &'a Addr,
    pub funds: &'a [Coin],
}

impl<'a> ExecParams<'a> {
    pub fn new(sender: &'a Addr, funds: &'a [Coin]) -> Self {
        Self { sender, funds }
    }
}
