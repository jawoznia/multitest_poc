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

// Generated
#[cfg(test)]
pub mod test_utils {
    use anyhow::Error;
    use cosmwasm_std::{Addr, StdResult};
    use cw_multi_test::{AppResponse, Executor};

    use crate::sylvia_utils;

    use super::{CountResponse, ExecMsg, QueryMsg};

    pub struct CounterProxy<'app> {
        contract: Addr,
        app: &'app crate::sylvia_utils::App,
    }
    impl<'app> CounterProxy<'app> {
        fn new(contract: Addr, app: &'app sylvia_utils::App) -> Self {
            CounterProxy { contract, app }
        }

        fn increase_count(&self, params: sylvia_utils::ExecParams) -> Result<AppResponse, Error> {
            let msg = ExecMsg::IncreaseCount {};

            self.app
                .app
                .borrow_mut()
                .execute_contract(
                    params.sender.clone(),
                    self.contract.clone(),
                    &msg,
                    params.funds,
                )
                .map_err(|err| err.downcast().unwrap())
        }

        fn count(&self) -> StdResult<CountResponse> {
            let msg = QueryMsg::Count {};

            self.app
                .app
                .borrow()
                .wrap()
                .query_wasm_smart(self.contract.clone(), &msg)
        }
    }
}
