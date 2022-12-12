use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, StdError};
use cw_storage_plus::Item;
use sylvia::contract;

pub struct CounterContract<'a> {
    pub(crate) count: Item<'a, u32>,
}

pub type ContractError = StdError;

#[contract]
impl CounterContract<'_> {
    pub const fn new() -> Self {
        Self {
            count: Item::new("count"),
        }
    }
    #[msg(instantiate)]
    pub fn instantiate(&self, ctx: (DepsMut, Env, MessageInfo)) -> Result<Response, ContractError> {
        let (deps, ..) = ctx;
        self.count.save(deps.storage, &0)?;
        Ok(Response::new())
    }
}
