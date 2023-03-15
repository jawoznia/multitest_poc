use crate::sylvia_utils::Contract;
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
#[cfg(test)]
use counter::multitest_utils::CounterProxy;
use cw_storage_plus::Item;
use sylvia::contract;

use crate::counter;
use crate::error::ContractError;

pub struct CounterContract<'a> {
    pub(crate) count: Item<'a, u32>,
    pub(crate) step: Item<'a, u32>,
}

impl<'a> Contract for CounterContract<'a> {
    type InstantiateMsg = InstantiateMsg;
    type ExecMsg = ExecMsg;
    type QueryMsg = QueryMsg;
    type MigrationMsg = sylvia::cw_std::Empty;
}

#[contract(error=ContractError)]
#[messages(counter as Counter)]
impl CounterContract<'_> {
    pub const fn new() -> Self {
        Self {
            count: Item::new("count"),
            step: Item::new("step"),
        }
    }

    #[msg(instantiate)]
    pub fn instantiate(&self, ctx: (DepsMut, Env, MessageInfo)) -> Result<Response, ContractError> {
        let (deps, ..) = ctx;
        self.count.save(deps.storage, &0)?;
        self.step.save(deps.storage, &1)?;
        Ok(Response::new())
    }

    #[msg(exec)]
    pub fn set_counter_step(
        &self,
        ctx: (DepsMut, Env, MessageInfo),
        step: u32,
    ) -> Result<Response, ContractError> {
        let (deps, ..) = ctx;
        self.step.save(deps.storage, &step)?;
        Ok(Response::new())
    }
}
// =====================================================
// To be Generated
// =====================================================
#[cfg(any(test, feature = "mt"))]
pub mod test_utils {
    use crate::sylvia_utils::Multitest;

    use super::multitest_utils::CounterContractCodeId;
    use super::multitest_utils::CounterContractProxy;
    use super::CounterContract;

    impl<'app> Multitest for CounterContract<'app> {
        type CodeId = CounterContractCodeId<'app>;
        type Contract = CounterContractProxy<'app>;
    }
}
