use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, StdError};
use cw_storage_plus::Item;
use sylvia::contract;

use crate::counter;

pub struct CounterContract<'a> {
    pub(crate) count: Item<'a, u32>,
}

pub type ContractError = StdError;

#[contract]
#[messages(counter as Counter)]
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

#[cfg(any(test, feature = "mt"))]
pub mod test_utils {
    use anyhow::bail;
    use cosmwasm_std::{from_slice, Addr, DepsMut, Empty, Env, MessageInfo, Response, StdResult};
    use cw_multi_test::{App, Contract, Executor};

    use crate::counter::{self, CountResponse};

    use super::{
        ContractError, ContractExecMsg, ContractQueryMsg, CounterContract, InstantiateMsg,
    };

    impl Contract<Empty> for CounterContract<'_> {
        fn execute(
            &self,
            deps: DepsMut<Empty>,
            env: Env,
            info: MessageInfo,
            msg: Vec<u8>,
        ) -> anyhow::Result<Response<Empty>> {
            from_slice::<ContractExecMsg>(&msg)?
                .dispatch(self, (deps, env, info))
                .map_err(Into::into)
        }

        fn instantiate(
            &self,
            deps: DepsMut<Empty>,
            env: Env,
            info: MessageInfo,
            msg: Vec<u8>,
        ) -> anyhow::Result<Response<Empty>> {
            from_slice::<InstantiateMsg>(&msg)?
                .dispatch(self, (deps, env, info))
                .map_err(Into::into)
        }

        fn query(
            &self,
            deps: cosmwasm_std::Deps<Empty>,
            env: Env,
            msg: Vec<u8>,
        ) -> anyhow::Result<cosmwasm_std::Binary> {
            from_slice::<ContractQueryMsg>(&msg)?
                .dispatch(self, (deps, env))
                .map_err(Into::into)
        }

        fn sudo(
            &self,
            _deps: DepsMut<Empty>,
            _env: Env,
            _msg: Vec<u8>,
        ) -> anyhow::Result<Response<Empty>> {
            bail!("sudo not implemented for contract")
        }

        fn reply(
            &self,
            _deps: DepsMut<Empty>,
            _env: Env,
            _msg: cosmwasm_std::Reply,
        ) -> anyhow::Result<Response<Empty>> {
            bail!("sudo not implemented for contract")
        }

        fn migrate(
            &self,
            _deps: DepsMut<Empty>,
            _env: Env,
            _msg: Vec<u8>,
        ) -> anyhow::Result<Response<Empty>> {
            bail!("sudo not implemented for contract")
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct CounterCodeId(u64);

    impl CounterCodeId {
        pub fn store_code(app: &mut App) -> Self {
            let code_id = app.store_code(Box::new(CounterContract::new()));
            Self(code_id)
        }

        pub fn code_id(&self) -> u64 {
            self.0
        }

        #[track_caller]
        pub fn instantiate(
            self,
            app: &mut App,
            sender: &Addr,
            label: &str,
            admin: Option<String>,
        ) -> Result<CounterProxy, ContractError> {
            let msg = InstantiateMsg {};

            app.instantiate_contract(self.0, sender.clone(), &msg, &[], label, admin)
                .map_err(|err| err.downcast().unwrap())
                .map(CounterProxy)
        }
    }

    #[derive(Debug)]
    pub struct CounterProxy(Addr);

    impl CounterProxy {
        // cw20-base
        #[track_caller]
        pub fn increase_counter(&self, app: &mut App, sender: &Addr) -> Result<(), ContractError> {
            let msg = counter::ExecMsg::IncreaseCount {};

            app.execute_contract(sender.clone(), self.0.clone(), &msg, &[])
                .map_err(|err| err.downcast().unwrap())
                .map(|_| ())
        }

        pub fn count(&self, app: &App) -> StdResult<CountResponse> {
            let msg = counter::QueryMsg::Count {};

            app.wrap().query_wasm_smart(self.0.clone(), &msg)
        }
    }
}
