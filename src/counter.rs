use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult};
use sylvia::interface;

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

impl Counter for CounterContract<'_> {
    type Error = ContractError;
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

    fn count(&self, ctx: (Deps, Env)) -> StdResult<CountResponse> {
        let (deps, _) = ctx;
        let count = self.count.load(deps.storage)?;
        Ok(CountResponse { count })
    }
}

// =====================================================
// Generated
// =====================================================
#[cfg(test)]
pub mod test_utils {
    use cosmwasm_std::{Addr, StdResult};
    use cw_multi_test::{AppResponse, Executor};

    use crate::{error::ContractError, sylvia_utils};

    use super::{CountResponse, ExecMsg, QueryMsg};

    pub struct CounterProxy<'app> {
        pub contract_addr: Addr,
        pub app: &'app crate::sylvia_utils::App,
    }
    impl<'app> CounterProxy<'app> {
        pub fn new(contract_addr: Addr, app: &'app sylvia_utils::App) -> Self {
            CounterProxy { contract_addr, app }
        }

        pub fn increase_count(
            &self,
            params: sylvia_utils::ExecParams,
        ) -> Result<AppResponse, ContractError> {
            let msg = ExecMsg::IncreaseCount {};

            self.app
                .app
                .borrow_mut()
                .execute_contract(
                    params.sender.clone(),
                    self.contract_addr.clone(),
                    &msg,
                    params.funds,
                )
                .map_err(|err| err.downcast().unwrap())
        }

        pub fn count(&self) -> StdResult<CountResponse> {
            let msg = QueryMsg::Count {};

            self.app
                .app
                .borrow()
                .wrap()
                .query_wasm_smart(self.contract_addr.clone(), &msg)
        }
    }

    impl Into<Addr> for CounterProxy<'_> {
        fn into(self) -> Addr {
            self.contract_addr
        }
    }
}
