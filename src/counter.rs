use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult};
use sylvia::{contract, interface};

use crate::{contract::CounterContract, error::ContractError};

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

#[contract]
impl Counter for CounterContract<'_> {
    type Error = ContractError;

    #[msg(exec)]
    fn increase_count(&self, ctx: (DepsMut, Env, MessageInfo)) -> Result<Response, ContractError> {
        let (deps, ..) = ctx;
        let step = self.step.load(deps.storage)?;
        let count = self.count.load(deps.storage)?;
        let new_count = step + count;
        if new_count > 42 {
            return Err(ContractError::Overflow);
        }

        self.count.save(deps.storage, &new_count)?;
        Ok(Response::new())
    }

    #[msg(query)]
    fn count(&self, ctx: (Deps, Env)) -> StdResult<CountResponse> {
        let (deps, _) = ctx;
        let count = self.count.load(deps.storage)?;
        Ok(CountResponse { count })
    }
}
