use crate::contract::CounterContract;
use crate::counter::{CountResponse, Counter};
use cosmwasm_std::{Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use utils::error::ContractError;

// This is still required
#[cfg(test)]
use crate::counter;

// #[contract]
// #[messages(counter as Counter)] => This is required to point to interface file
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
#[cfg(any(test, feature = "mt"))]
pub mod test_utils {
    use super::*;
    use cosmwasm_std::StdResult;
    use cw_multi_test::{AppResponse, Executor};

    pub trait CounterMethods {
        fn increase_count(
            &self,
            params: utils::sylvia_utils::ExecParams,
        ) -> Result<AppResponse, ContractError>;

        fn count(&self) -> StdResult<CountResponse>;
    }

    impl<'app> CounterMethods for counter::trait_utils::CounterProxy<'app> {
        fn increase_count(
            &self,
            params: utils::sylvia_utils::ExecParams,
        ) -> Result<AppResponse, ContractError> {
            let msg = counter::ExecMsg::IncreaseCount {};

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

        fn count(&self) -> StdResult<CountResponse> {
            let msg = counter::QueryMsg::Count {};

            self.app
                .app
                .borrow()
                .wrap()
                .query_wasm_smart(self.contract_addr.clone(), &msg)
        }
    }
}
