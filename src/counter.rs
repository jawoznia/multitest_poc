use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult};
use sylvia::interface;

use crate::contract::{ContractError, CounterContract};

#[cw_serde]
pub struct CountResponse {
    pub count: u32,
}

#[interface]
pub trait Counter {
    type Error: From<StdError>;

    #[msg(exec)]
    fn increase_count(&self, ctx: (DepsMut, Env, MessageInfo)) -> Result<Response, Self::Error>;

    #[msg(query)]
    fn count(&self, ctx: (Deps, Env)) -> StdResult<CountResponse>;
}

impl Counter for CounterContract<'_> {
    type Error = ContractError;
    fn increase_count(&self, ctx: (DepsMut, Env, MessageInfo)) -> Result<Response, ContractError> {
        let (deps, ..) = ctx;
        self.count
            .update(deps.storage, |c| -> StdResult<_> { Ok(c + 1) })?;
        Ok(Response::new())
    }

    fn count(&self, ctx: (Deps, Env)) -> StdResult<CountResponse> {
        let (deps, _) = ctx;
        let count = self.count.load(deps.storage)?;
        Ok(CountResponse { count })
    }
}
